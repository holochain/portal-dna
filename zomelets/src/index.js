import {
    AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash'; // approx. 11kb
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets'; // approx. 7kb
import {
    HostEntry,
}					from './types.js';

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
    "host_entries":			true,
    "latest_host_entry_for_dna":	true,
    "bridge_call":			true,
    "ping":				true,
    "pong":				true,
    "register_host":			true,
    "get_registered_hosts":		true,
    "get_registered_hosts_randomized":	true,
    "get_hosts_for_zome_function":	true,
    "custom_remote_call":		true,
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
