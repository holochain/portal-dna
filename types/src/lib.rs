mod host_entry;

pub use hdi_extensions;
pub use hdi_extensions::hdi;

pub use host_entry::*;

use std::collections::BTreeMap;
use hdi::prelude::*;



//
// General-use Types
//
pub type Payload = rmpv::Value;
pub type Metadata = BTreeMap<String, rmpv::Value>;

pub type RemoteCallInput = RemoteCallDetails<String, String, Payload>;
pub type BridgeCallInput = BridgeCallDetails<String, String, Payload>;



// Trait for common fields
pub trait CommonFields<'a> {
    fn author(&'a self) -> &'a AgentPubKey;
    fn published_at(&'a self) -> &'a u64;
    fn last_updated(&'a self) -> &'a u64;
    fn metadata(&'a self) -> &'a Metadata;
}



//
// General-use Structs
//
#[derive(Debug, Deserialize, Serialize)]
pub struct DnaZomeFunction {
    pub dna: holo_hash::DnaHash,
    pub zome: ZomeName,
    pub function: FunctionName,
}



//
// Input Structs
//
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteCallDetails<Z,F,I>
where
    Z: Into<ZomeName>,
    F: Into<FunctionName>,
    I: Serialize + core::fmt::Debug,
{
    pub dna: DnaHash,
    pub zome: Z,
    pub function: F,
    pub payload: I,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BridgeCallDetails<Z,F,P>
where
    Z: Into<ZomeName>,
    F: Into<FunctionName>,
    P: Serialize + core::fmt::Debug,
{
    pub dna: DnaHash,
    pub zome: Z,
    pub function: F,
    pub payload: P,
}
