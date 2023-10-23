use crate::{
    hdk,
    hc_crud,
};

use std::collections::BTreeMap;
use rand::seq::SliceRandom;
use hdk::prelude::*;
use holo_hash::DnaHash;
use hc_crud::{
    now, create_entity, get_entity,// update_entity,
    Entity,
};
use portal::{
    LinkTypes,

    HostEntry,

    Metadata,
};
use crate::{
    ANCHOR_HOSTS,
};



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub dna: DnaHash,
    pub granted_functions: GrantedFunctions,

    // optional
    pub cap_access: Option<CapAccess>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<Metadata>,
}


pub fn create(input: CreateInput) -> ExternResult<Entity<HostEntry>> {
    debug!("Creating Host of {}: {:?}", input.dna, input.granted_functions );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let host = HostEntry {
	dna: input.dna.clone(),
	capabilities: CapGrantEntry {
	    tag: String::from(""),
	    access: input.cap_access
		.unwrap_or( CapAccess::Unrestricted ),
	    functions: input.granted_functions,
	},

	author: pubkey,
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),

	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
    };
    let entity = create_entity( &host )?;

    { // Path via Agent's Hosts
	debug!("Hosting anchor: {}.{}", ANCHOR_HOSTS, input.dna.to_string() );
	let (_, pathhash ) = portal_sdk::path( ANCHOR_HOSTS, vec![
	    input.dna.to_string(),
	]);
	entity.link_from( &pathhash, LinkTypes::Host, None )?;
    }

    Ok( entity )
}


#[derive(Debug, Deserialize)]
pub struct GetInput {
    pub dna: DnaHash,
}

pub fn list_links (input: GetInput) -> ExternResult<Vec<ActionHash>> {
    debug!("Get links from hosting anchor: {}.{}", ANCHOR_HOSTS, &input.dna.to_string() );
    let (_, pathhash ) = portal_sdk::path( ANCHOR_HOSTS, vec![
	&input.dna.to_string(),
    ]);
    let links = get_links( pathhash, LinkTypes::Host, None )?;

    Ok(
	links
	    .into_iter()
	    .filter_map(|link| link.target.into_action_hash() )
	    .collect()
    )
}

pub fn list (input: GetInput) -> ExternResult<Vec<Entity<HostEntry>>> {
    let addrs = list_links( input )?;
    let mut hosts : Vec<Entity<HostEntry>> = Vec::new();

    for host_addr in addrs {
	let host : Entity<HostEntry> = get_entity( &host_addr )?;
	hosts.push( host );
    }

    Ok( hosts )
}

pub fn list_random (input: GetInput) -> ExternResult<Vec<Entity<HostEntry>>> {
    let mut list = list( input )?;

    list.shuffle(&mut rand::thread_rng());

    Ok( list )
}
