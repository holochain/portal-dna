use crate::hdi;

use hdi::prelude::*;
use crate::{
    Metadata,
    CommonFields,
};



//
// Host Entry
//
/// Defines the available zome calls for a specific host
#[hdk_entry_helper]
#[derive(Clone)]
pub struct HostEntry {
    pub dna: DnaHash,
    pub capabilities: ZomeCallCapGrant,

    // common fields
    pub author: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,
    pub metadata: Metadata,
}

impl<'a> CommonFields<'a> for HostEntry {
    fn author(&'a self) -> &'a AgentPubKey {
        &self.author
    }
    fn published_at(&'a self) -> &'a u64 {
        &self.published_at
    }
    fn last_updated(&'a self) -> &'a u64 {
        &self.last_updated
    }
    fn metadata(&'a self) -> &'a Metadata {
        &self.metadata
    }
}
