mod host_entry;

pub use hdi_extensions;
pub use hdi_extensions::hdi;

pub use host_entry::*;

use std::collections::BTreeMap;
use hdi::prelude::*;



//
// General-use Types
//
pub type Metadata = BTreeMap<String, rmpv::Value>;



// Trait for common fields
pub trait CommonFields<'a> {
    fn author(&'a self) -> &'a AgentPubKey;
    fn published_at(&'a self) -> &'a u64;
    fn last_updated(&'a self) -> &'a u64;
    fn metadata(&'a self) -> &'a Metadata;
}
