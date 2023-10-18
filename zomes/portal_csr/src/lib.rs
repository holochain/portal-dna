mod constants;
mod host;

pub use portal::hdi;
pub use portal::hdi_extensions;
pub use portal::hc_crud;
pub use portal_sdk::hdk;
pub use portal_sdk::hdk_extensions;
pub use constants::*;

use rand::seq::SliceRandom;
use hdi_extensions::{
    guest_error,
};
use hdk::prelude::*;
use holo_hash::DnaHash;
use hc_crud::{
    Entity,
};
use portal::{
    EntryTypesUnit,
    HostEntry,
};
use portal_sdk::{
    Payload,
    DnaZomeFunction,
    RemoteCallInput,
    BridgeCallInput,
    BridgeCallDetails,
};



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut anonymous_caps = BTreeSet::new();
    let zome_info = zome_info()?;

    anonymous_caps.insert( (zome_info.name.to_owned(), FunctionName::new("bridge_call")) );
    anonymous_caps.insert( (zome_info.name.to_owned(), FunctionName::new("pong")) );

    create_cap_grant( CapGrantEntry {
	tag: String::from("Public Functions"),
	access: CapAccess::Unrestricted,
	functions: GrantedFunctions::Listed( anonymous_caps ),
    })?;

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}



#[hdk_extern]
fn my_host_entries(_:()) -> ExternResult<Vec<HostEntry>> {
    Ok( host_entries()? )
}

pub fn host_entries() -> ExternResult<Vec<HostEntry>> {
    query(ChainQueryFilter {
	sequence_range: ChainQueryFilterRange::Unbounded,
	entry_type: Some( vec![ EntryTypesUnit::Host.try_into()? ] ),
	entry_hashes: None,
	action_type: Some( vec![ ActionType::Create ] ),
	include_entries: true,
	order_descending: true,
    })?
	.into_iter()
	.map(|record| {
	    match record.entry {
		RecordEntry::Present(entry) => {
		    Ok( entry.try_into()? )
		},
		// Should be unreachable because of chain query filter settings
		_ => Err(guest_error!("Expected entry; Chain query filter provided Create with no entry present".to_string()))?,
	    }
	})
	.collect()
}

pub fn latest_host_entry_for_dna(dna: &DnaHash) -> ExternResult<Option<HostEntry>> {
    let dna_entries = host_entries()?
	.into_iter()
	.filter(|host_entry| host_entry.dna == *dna )
	.collect::<Vec<HostEntry>>();
    let host_entry = dna_entries.first();

    Ok( host_entry.map(|he| he.to_owned() ) )
}

fn handler_bridge_call(input: BridgeCallInput) -> ExternResult<Payload> {
    let agent_info = agent_info()?;

    // Need to add a check here for this agent's registered zome functions
    match latest_host_entry_for_dna( &input.dna )? {
	Some(host_entry) => {
	    match host_entry.capabilities.access {
		CapAccess::Unrestricted => (),
		_ => return Err(guest_error!(format!("Access is conditional for DNA {}, but only Unrestricted is supported at this time", input.dna )))?,
	    }

	    match host_entry.capabilities.functions {
		GrantedFunctions::Listed( granted_functions ) => {
		    if let None = granted_functions
			.into_iter()
			.find(|(zome, function)| {
			    return *zome == input.zome.clone().into()
				&& *function == input.function.clone().into()
			})
		    {
		        Err(guest_error!(format!("No capability granted for DNA zome/function {}/{}", input.zome, input.function )))?;
		    }
		}
		_ => (),
	    }
	},
	None => {
	    return Err(guest_error!(format!("No host record for DNA {}", input.dna )))?;
	},
    };

    let cell_id = CellId::new( input.dna, agent_info.agent_initial_pubkey );

    debug!("Received remote call to bridge: {}::{}->{}", cell_id, input.zome, input.function );
    let response = call(
	CallTargetCell::OtherCell( cell_id ),
	input.zome,
	input.function.into(),
	None,
	input.payload
    )?;
    let result = portal_sdk::zome_call_response_as_result( response )?;

    Ok(
        result.decode()
            .map_err( |err| guest_error!(format!("{:?}", err )) )?
    )
}

#[hdk_extern]
fn bridge_call(input: BridgeCallInput) -> ExternResult<Payload> {
    let result = handler_bridge_call( input )?;

     Ok( result )
}


fn handler_ping_call(host: AgentPubKey) -> ExternResult<bool> {
    let response = call_remote(
	host,
	"portal_csr",
	"pong".into(),
	None,
	(),
    )?;
    let result = portal_sdk::zome_call_response_as_result( response )?;
    let _response : String = result.decode()
        .map_err( |err| guest_error!(format!("{:?}", err )) )?;

    Ok( true )
}

#[hdk_extern]
fn ping(host: AgentPubKey) -> ExternResult<bool> {
    debug!("Sending ping to host: {}", host );
    let success = handler_ping_call( host )?;

    Ok( success )
}


#[hdk_extern]
fn pong(_: ()) -> ExternResult<String> {
    debug!("Responding with pong");
    Ok( String::from("pong") )
}



#[hdk_extern]
fn register_host(input: host::CreateInput) -> ExternResult<Entity<HostEntry>> {
    let entity = host::create( input )?;

    Ok( entity )
}

#[hdk_extern]
fn get_registered_hosts(input: host::GetInput) -> ExternResult<Vec<Entity<HostEntry>>> {
    let list = host::list( input )?;

    Ok( list )
}

#[hdk_extern]
fn get_registered_hosts_randomized(input: host::GetInput) -> ExternResult<Vec<Entity<HostEntry>>> {
    let mut list : Vec<Entity<HostEntry>> = host::list( input )?;

    list.shuffle(&mut rand::thread_rng());

    Ok( list )
}


fn handler_get_hosts_for_zome_function(dna: DnaHash, zome: ZomeName, function: FunctionName) -> ExternResult<Vec<Entity<HostEntry>>> {
    let hosts = host::list( host::GetInput {
	dna: dna,
    })?;

    Ok(
	hosts.into_iter()
	    .filter(|host_entry| {
		match &host_entry.content.capabilities.functions {
		    GrantedFunctions::Listed( granted_functions ) => {
			granted_functions
			    .into_iter()
			    .find(|(cap_zome, cap_function)| {
				return *cap_zome == zome
				    && *cap_function == function
			    })
			    .is_some()
		    },
		    GrantedFunctions::All => true,
		}
	    })
	    .collect()
    )
}


#[hdk_extern]
fn get_hosts_for_zome_function(input: DnaZomeFunction) -> ExternResult<Vec<Entity<HostEntry>>> {
    let hosts = handler_get_hosts_for_zome_function(input.dna, input.zome, input.function)?;

    Ok( hosts )
}



#[derive(Debug, Deserialize)]
pub struct CustomRemoteCallInput {
    host: AgentPubKey,
    call: RemoteCallInput,
}

fn handler_custom_remote_call(input: CustomRemoteCallInput) -> ExternResult<Payload> {
    let call_details = BridgeCallDetails {
	dna: input.call.dna,
	zome: input.call.zome,
	function: input.call.function,
	payload: input.call.payload,
    };

    let response = call_remote(
	input.host,
	"portal_csr",
	"bridge_call".into(),
	None,
	call_details,
    )?;

    let result = portal_sdk::zome_call_response_as_result( response )?;

    Ok(
        result.decode()
            .map_err( |err| guest_error!(format!("{:?}", err )) )?
    )
}

#[hdk_extern]
fn custom_remote_call(input: CustomRemoteCallInput) -> ExternResult<Payload> {
    Ok( handler_custom_remote_call( input )? )
}
