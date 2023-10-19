mod validation;

pub use hc_crud;
pub use portal_types;
pub use portal_types::*;

use serde::{
    Deserialize, Deserializer,
};
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
    scoped_type_connector,
    ScopedTypeConnector,
};
use hc_crud::{
    entry_model,
};



#[hdk_entry_defs]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    #[entry_def]
    Host(HostEntry),
}

scoped_type_connector!(
    EntryTypesUnit::Host,
    EntryTypes::Host( HostEntry )
);

// Entry Types with CRUD models
entry_model!( EntryTypes::Host( HostEntry ) );

#[hdk_link_types]
pub enum LinkTypes {
    Agent,

    Host,

    Anchor,
}

impl TryFrom<String> for LinkTypes {
    type Error = WasmError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Ok(
            match name.as_str() {
	        "Agent" => LinkTypes::Agent,
	        "Host" => LinkTypes::Host,
	        "Anchor" => LinkTypes::Anchor,
                _ => return Err(guest_error!(format!("Unknown LinkTypes variant: {}", name ))),
            }
        )
    }
}

impl<'de> Deserialize<'de> for LinkTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(
            LinkTypes::try_from( s.clone() )
                .or(Err(serde::de::Error::custom(format!("Unknown LinkTypes variant: {}", s))))?
        )
    }
}
