pub use hdi_extensions;
pub use hdi_extensions::hdi;

use hdi::prelude::*;
use hdi_extensions::{
    ScopedTypeConnector,
    scoped_type_connector,
};
use hdi_extensions::{
    // Macros
    valid,
};



//
// Content Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ContentEntry {
    pub name: String,
    pub content: String,
}


//
// Entry Types
//
#[hdk_entry_types]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    #[entry_type]
    Content(ContentEntry),
}

scoped_type_connector!(
    EntryTypesUnit::Content,
    EntryTypes::Content( ContentEntry )
);


//
// Link Types
//
#[hdk_link_types]
pub enum LinkTypes {
    Generic,
}


//
// Validation
//
#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        _ => valid!(),
    }
}
