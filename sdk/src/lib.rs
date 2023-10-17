pub use hdk_extensions::hdi;
pub use hdk_extensions::holo_hash;
pub use hdk_extensions::hdk;
pub use hdk_extensions::hdi_extensions;
pub use hdk_extensions;

use hdi_extensions::{
    guest_error,
};
use hdk::prelude::*;
use hdk::hash_path::path::{ Component };



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
