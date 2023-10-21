//! Other Resources
//!
//! - Source code - [github.com/holochain/portal-dna](https://github.com/holochain/portal-dna)
//! - Cargo package - [crates.io/crates/hc_portal_types](https://crates.io/crates/hc_portal_types)
//!

mod host_entry;

pub use hdi_extensions;
pub use hdi_extensions::hdi;

pub use host_entry::*;

use std::collections::BTreeMap;
use hdi::prelude::*;



//
// General-use Types
//
/// A placeholder for additional data that does not affect integrity validation
pub type Metadata = BTreeMap<String, rmpv::Value>;



// Trait for common fields
/// Fields that are useful for any entry type that implements CRUD
pub trait CommonFields<'a> {
    fn author(&'a self) -> &'a AgentPubKey;
    fn published_at(&'a self) -> &'a u64;
    fn last_updated(&'a self) -> &'a u64;
    fn metadata(&'a self) -> &'a Metadata;
}
