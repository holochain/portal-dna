pub use hdk_extensions::hdi;
pub use hdk_extensions::holo_hash;
pub use hdk_extensions::hdk;
pub use hdk_extensions::hdi_extensions;
pub use hdk_extensions;
pub use rmpv;
pub use hc_crud;
pub use portal_types;

use hdi_extensions::{
    guest_error,
};
use hdk::prelude::*;
use hdk::hash_path::path::{ Component };
use holo_hash::DnaHash;



//
// General-use Types
//
pub type Payload = rmpv::Value;



//
// Input defintions
//
pub type RemoteCallInput = RemoteCallDetails<String, String, Payload>;
pub type BridgeCallInput = BridgeCallDetails<String, String, Payload>;


#[derive(Debug, Deserialize, Serialize)]
pub struct DnaZomeFunction {
    pub dna: DnaHash,
    pub zome: ZomeName,
    pub function: FunctionName,
}


// Input Structs
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



//
// Path creation helper
//
pub fn path<T>( base: &str, segments: T ) -> (Path, EntryHash)
where
    T: IntoIterator,
    T::Item: std::fmt::Display,
{
    let mut components : Vec<Component> = vec![];

    for seg in base.split(".") {
	let component = Component::from( format!("{}", seg ).as_bytes().to_vec() );
	components.push( component );
    }

    for seg in segments {
	let component = Component::from( format!("{}", seg ).as_bytes().to_vec() );
	components.push( component );
    }

    let path = Path::from( components );
    let hash = path.path_entry_hash().unwrap();

    ( path, hash )
}



//
// ZomeCallResponse handler
//
pub fn zome_call_response_as_result(response: ZomeCallResponse) -> ExternResult<ExternIO> {
    Ok( match response {
	ZomeCallResponse::Ok(bytes)
	    => Ok(bytes),
	ZomeCallResponse::Unauthorized(zome_call_auth, cell_id, zome, func, agent)
	    => Err(guest_error!(format!("UnauthorizedError( {}, {}, {}, {}, {} )", zome_call_auth, cell_id, zome, func, agent ))),
	ZomeCallResponse::NetworkError(message)
	    => Err(guest_error!(format!("NetworkError( {} )", message ))),
	ZomeCallResponse::CountersigningSession(message)
	    => Err(guest_error!(format!("CountersigningSessionError( {} )", message ))),
    }? )
}



#[macro_export]
macro_rules! call_local_cell {
    ( $role:literal, $zome:literal, $fn:literal, $($input:tt)+ ) => {
        {
            use portal_sdk::hdk::prelude::*;
            use portal_sdk::hdi_extensions::guest_error;

            let call_response = call(
                CallTargetCell::OtherRole( $role.into() ),
                $zome,
                $fn.into(),
                None,
                $($input)+,
            )?;

            match call_response {
                ZomeCallResponse::Ok(extern_io) => Ok(extern_io),
                ZomeCallResponse::NetworkError(msg) => Err(guest_error!(format!("{}", msg))),
                ZomeCallResponse::CountersigningSession(msg) => Err(guest_error!(format!("{}", msg))),
                _ => Err(guest_error!(format!("Zome call response: Unauthorized"))),
            }
        }
    };
}

#[macro_export]
macro_rules! call_local_cell_decode {
    ( $role:literal, $zome:literal, $fn:literal, $($input:tt)+ ) => {
        {
            use portal_sdk::hdk::prelude::*;

            portal_sdk::call_local_cell!( $role, $zome, $fn, $($input)+ )?
                .decode()
                .map_err(|err| wasm_error!(WasmErrorInner::from(err)) )
        }
    };
    ( $into_type:ident, $role:literal, $zome:literal, $fn:literal, $($input:tt)+ ) => {
        {
            use portal_sdk::hdk::prelude::*;

            portal_sdk::call_local_cell!( $role, $zome, $fn, $($input)+ )?
                .decode::<$into_type>()
                .map_err(|err| wasm_error!(WasmErrorInner::from(err)) )
        }
    };
}

pub type ZomeFunction<T1,T2> = (T1, T2);

#[derive(Debug, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct ListedFunctions {
    pub Listed: Vec<ZomeFunction<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct RegisterHostInput {
    pub dna: DnaHash,
    pub granted_functions: ListedFunctions,
}

#[derive(Debug, Serialize)]
pub struct RegisterInput<T1,T2>
where
    T1: Into<String>,
    T2: Into<String>,
{
    pub dna: DnaHash,
    pub granted_functions: Vec<ZomeFunction<T1,T2>>,
}

#[macro_export]
macro_rules! register {
    ( $($def:tt)* ) => {
        {
            use portal_sdk::hdk::prelude::*;
            use portal_sdk::hc_crud::Entity;
            use portal_sdk::portal_types::HostEntry;

            let input = portal_sdk::RegisterInput $($def)*;
            let payload = portal_sdk::RegisterHostInput {
                dna: input.dna,
                granted_functions: portal_sdk::ListedFunctions {
                    Listed: input.granted_functions.into_iter()
                        .map( |(zome, func)| (zome.into(), func.into()) )
                        .collect()
                },
            };

            type Response = Entity<HostEntry>;

            portal_sdk::call_local_cell_decode!(
                Response,
                "portal",
                "portal_csr",
                "register_host",
                payload
            )
        }
    };
}

#[macro_export]
macro_rules! register_if_exists {
    ( $($def:tt)* ) => {
        {
            use portal_sdk::hdk::prelude::*;

            let result = portal_sdk::register!( $($def)* );

            debug!("Register self ({}) result: {:#?}", dna_info()?.hash, result );
            match result {
                Err(err) => match err {
                    WasmError {
                        error: WasmErrorInner::Host(ref msg),
                        ..
                    } if msg.contains("Role not found") => Ok(None),
                    err => Err(err),
                },
                Ok(value) => Ok(Some(value)),
            }
        }
    };
}
