use crate::*;

use hdi_extensions::{
    guest_error,
//     // Macros
//     valid, // invalid,
};



pub fn store_entry_deconstruct<ET>( store_entry: &StoreEntry ) -> ExternResult<Option<ET>>
where
    ET: EntryTypesHelper,
    WasmError: From<<ET as EntryTypesHelper>::Error>,
{
    Ok(match store_entry.action.hashed.content.entry_type() {
        EntryType::App(AppEntryDef {
            zome_index,
            entry_index,
            ..
        }) => {
            Some(
                ET::deserialize_from_type( *zome_index, *entry_index, &store_entry.entry )?
                    .ok_or( guest_error!("No entry type matched for:".to_string()) )?
            )
        },
        _ => None,
    })
}

pub fn register_update_deconstruct<ET>( register_update: &RegisterUpdate ) -> ExternResult<Option<ET>>
where
    ET: EntryTypesHelper,
    WasmError: From<<ET as EntryTypesHelper>::Error>,
{
    let update = register_update.update.hashed.content.clone();
    let original_action = must_get_action( update.original_action_address )?;
    let entry_type = original_action.action().entry_type()
        .ok_or(guest_error!(format!(
            "Original action ({}) does not have an entry",
            original_action.action_address(),
        )))?;

    Ok(match entry_type {
        EntryType::App(AppEntryDef {
            zome_index,
            entry_index,
            visibility,
        }) => {
            Some(match &register_update.new_entry {
                None => Err( guest_error!(format!("New entry is None meaning visibility is Private: {:?}", visibility )) )?,
                Some(entry) => {
                    ET::deserialize_from_type( *zome_index, *entry_index, &entry )?
                        .ok_or( guest_error!("No entry type matched for:".to_string()) )?
                },
            })
        },
        _ => None,
    })
}

pub fn register_delete_deconstruct<ET>( register_delete: &RegisterDelete ) -> ExternResult<Option<ET>>
where
    ET: EntryTypesHelper,
    WasmError: From<<ET as EntryTypesHelper>::Error>,
{
    let delete = register_delete.delete.hashed.content.clone();
    let original_action = must_get_action( delete.deletes_address )?;
    let entry_type = original_action.action().entry_type()
        .ok_or(guest_error!(format!(
            "Original action ({}) does not have an entry",
            original_action.action_address(),
        )))?;
    let original_entry = must_get_entry( delete.deletes_entry_address )?;

    Ok(match entry_type {
        EntryType::App(AppEntryDef {
            zome_index,
            entry_index,
            visibility: _,
        }) => {
            Some(
                ET::deserialize_from_type( *zome_index, *entry_index, &original_entry )?
                    .ok_or( guest_error!("No entry type matched for:".to_string()) )?
            )
        },
        _ => None,
    })
}



#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.clone() {
        // When any entry is being posted to the DHT
        Op::StoreEntry( store_entry ) => {
            if let Some( entry_type ) = store_entry_deconstruct( &store_entry )? {
                debug!("Op::{} => Running validation for: {:?}", op.action_type(), entry_type );
                return match entry_type {
                    EntryTypes::Host(content) => validate_host_create( &op, content ),
                };
            } else {
                if let Entry::CapGrant(_) = store_entry.entry {
                    return Ok(ValidateCallbackResult::Valid);
                }
            }
        },

        // When the created entry is an update
        Op::RegisterUpdate( register_update ) => {
            if let Some( entry_type ) = register_update_deconstruct( &register_update )? {
                debug!("Op::{} => Running validation for: {:?}", op.action_type(), entry_type );
                return match entry_type {
                    EntryTypes::Host(content) => {
                        let original_entry : HostEntry = must_get_entry(
                            register_update.update.hashed.content.original_entry_address
                        )?.try_into()?;
                        validate_host_update( &op, content, original_entry )
                    },
                };
            }
        },

        // When deleting an entry creation
        Op::RegisterDelete( register_delete ) => {
            if let Some( entry_type ) = register_delete_deconstruct( &register_delete )? {
                debug!("Op::{} => Running validation for: {:?}", op.action_type(), entry_type );
                return match entry_type {
                    EntryTypes::Host(original_entry) => validate_host_delete( &op, original_entry ),
                };
            }
        },

        // Ignore the rest
        //  - StoreRecord
        //  - RegisterAgentActivity
        //  - RegisterCreateLink
        //  - RegisterDeleteLink
        _ => {
            debug!("Op::{} => No validation", op.action_type() );
            return Ok(ValidateCallbackResult::Valid);
        }
    }

    debug!("Op::{} => Validation fall-through: {:#?}", op.action_type(), op );
    Ok(ValidateCallbackResult::Valid)
}



fn validate_common_fields_create<'a, T>(op: &Op, entry: &'a T) -> ExternResult<ValidateCallbackResult>
where
    T: CommonFields<'a>,
{
    if entry.author() != op.author() {
        Ok(ValidateCallbackResult::Invalid(format!("Entry author does not match Action author: {} != {}", entry.author(), op.author() )))
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_common_fields_update<'a, T>(op: &Op, entry: &'a T, prev_entry: &'a T) -> ExternResult<ValidateCallbackResult>
where
    T: CommonFields<'a>,
{
    if let Err(error) = validate_common_fields_create(op, entry) {
        Err(error)?
    }

    if prev_entry.author() != op.author() {
        return Ok(ValidateCallbackResult::Invalid(format!("Previous entry author does not match Action author: {} != {}", prev_entry.author(), op.author() )));
    }
    else if entry.author() != prev_entry.author()  {
        return Ok(ValidateCallbackResult::Invalid(format!("Cannot change app author: {} => {}", prev_entry.author(), entry.author() )));
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}


//
// Host
//
fn validate_host_create(op: &Op, entry: HostEntry) -> ExternResult<ValidateCallbackResult> {
    if let Err(error) = validate_common_fields_create(op, &entry) {
        Err(error)?
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_host_update(op: &Op, entry: HostEntry, prev_entry: HostEntry) -> ExternResult<ValidateCallbackResult> {
    if let Err(error) = validate_common_fields_update(op, &entry, &prev_entry) {
        Err(error)?
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_host_delete(_op: &Op, _entry: HostEntry) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
