import {
    AgentPubKey, DnaHash,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash'; // approx. 11kb
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets'; // approx. 7kb
import {
    Host,
}					from './types.js';



function host_map ( hosts, zomelet ) {
    if ( !zomelet )
	throw new TypeError(`Missing 'CallContext' for entity instance`);

    if ( zomelet?.constructor?.name !== "CallContext" )
	throw new TypeError(`'zomelet' input must be a 'CallContext'; not type '${zomelet?.constructor?.name}'`);

    return hosts.map( entity => new Host( entity, zomelet ) );
}



export const PortalCSRZomelet		= new Zomelet({
    "whoami": {
	output ( response ) {
	    // Struct - https://docs.rs/hdk/*/hdk/prelude/struct.AgentInfo.html
	    return {
		"pubkey": {
		    "initial":		new AgentPubKey( response.agent_initial_pubkey ),
		    "latest":		new AgentPubKey( response.agent_latest_pubkey ),
		},
		"chain_head": {
		    "action":		new ActionHash( response.chain_head[0] ),
		    "sequence":		response.chain_head[1],
		    "timestamp":	response.chain_head[2],
		},
	    };
	},
    },
    "my_host_entries":			true,
    "latest_host_entry_for_dna":	true,
    async ping ( input ) {
	this.log.trace("Pinging host '%s'", input );
	return await this.call( input, {
	    "timeout": 1_000,
	});
    },
    async register_host ( input ) {
	if ( input.zomes ) {
	    if ( input.granted_functions )
		this.log.warn("Ignoring 'zomes' because 'granted_functions' input overrides it");
	    else {
		const Listed		= [];

		for ( let [name, func_names] of Object.entries( input.zomes ) ) {
		    func_names.forEach( func_name => {
			Listed.push([ name, func_name ]);
		    });
		}

		input.granted_functions	= {
		    Listed,
		};
	    }
	}

	return await this.call( input );
    },
    async get_registered_hosts ( input ) {
	if ( typeof input === "string" || input instanceof Uint8Array ) {
	    input			= {
		"dna": new DnaHash( input ),
	    };
	}

	return host_map( await this.call( input ), this );
    },
    async get_registered_hosts_randomized ( input ) {
	if ( typeof input === "string" || input instanceof Uint8Array ) {
	    input			= {
		"dna": new DnaHash( input ),
	    };
	}

	return host_map( await this.call( input ), this );
    },
    async get_hosts_for_zome_function ( input ) {
	if ( !input.dna )
	    throw new TypeError(`Missing 'dna' input`);

	input.dna			= new DnaHash( input.dna );

	if ( typeof input.zome !== "string" )
	    throw new TypeError(`Expected 'zome' input to be a string; not type ${typeof input.zome}`);

	if ( typeof input.function !== "string" )
	    throw new TypeError(`Expected 'function' input to be a string; not type ${typeof input.function}`);

	return host_map( await this.call( input ), this );
    },
    "custom_remote_call":		true,

    //
    // Virtual functions
    //
    async get_available_host_for_zome_function ( input ) {
	const hosts			= await this.functions.get_hosts_for_zome_function( input );
	const host_pings		= hosts.map( async host => {
	    try {
		await this.functions.ping( host.author );
		return host.author;
	    } catch (err) {
		this.log.debug("Ping to '%s' failed with: %s", host.author, err );
		return null;
	    }
	});
	const available_host		= await Promise.any( host_pings );

	return available_host;
    },
    async remote_call ( input ) {
	const host			= await this.functions.get_available_host_for_zome_function({
	    "dna": input.dna,
	    "zome": input.zome,
	    "function": input.function,
	});

	this.log.info("Remote calling host (%s) for %s::%s->%s", host, input.dna, input.zome, input.function, input.payload );
	return await this.functions.custom_remote_call({
	    host,
	    "call": {
		"dna": input.dna,
		"zome": input.zome,
		"function": input.function,
		"payload": input.payload,
	    },
	});
    },
});


export const PortalCell			= new CellZomelets({
    "portal_csr":	PortalCSRZomelet,
});


export default {
    // Zomelets
    PortalCSRZomelet,

    // CellZomelets
    PortalCell,
};
