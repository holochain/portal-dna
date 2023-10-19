pub use content::hdi_extensions;
pub use portal_sdk::hdk;
pub use portal_sdk::hdk_extensions;
pub use portal_sdk::rmpv;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    GetEntityInput,
};
use hdi_extensions::{
    ScopedTypeConnector,
};
use holo_hash::DnaHash;
use content::{
    ContentEntry,
};



#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ListedFunctions {
    Listed: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterHostInput {
    pub dna: DnaHash,
    pub granted_functions: ListedFunctions,
}


#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    debug!("'{}' init", zome_info()?.name );

    // Register with portal if the cell is present
    let result = portal_sdk::register!({
        dna: dna_info()?.hash,
        granted_functions: vec![
            ( "content_csr", "get_content" ),
        ],
    });

    if let Err(err) = result {
        if let WasmError { error: WasmErrorInner::Host(msg), .. } = &err {
            if !msg.contains("Role not found") {
                return Err(err);
            }
        }
    }

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}


#[hdk_extern]
pub fn create_content(content: ContentEntry) -> ExternResult<ActionHash> {
    debug!("Creating new content entry: {:#?}", content );
    let action_hash = create_entry( content.to_input() )?;

    Ok( action_hash )
}


#[hdk_extern]
pub fn get_content(input: GetEntityInput) -> ExternResult<ContentEntry> {
    debug!("Get latest content entry: {:#?}", input );
    let record = must_get( &input.id )?;

    Ok( ContentEntry::try_from_record( &record )? )
}


#[hdk_extern]
pub fn get_content_by_hash(input: EntryHash) -> ExternResult<ContentEntry> {
    debug!("Get latest content entry: {:#?}", input );
    let record = must_get( &input )?;

    Ok( ContentEntry::try_from_record( &record )? )
}


#[hdk_extern]
pub fn hash_content(content: ContentEntry) -> ExternResult<EntryHash> {
    debug!("Creating new content entry: {:#?}", content );
    let entry_hash = hash_entry( content )?;

    Ok( entry_hash )
}
