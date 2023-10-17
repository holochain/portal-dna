
import { AgentPubKey, DnaHash,
	 ActionHash, EntryHash }	from '@spartan-hc/holo-hash';
import {
    intoStruct,
    OptionType, VecType, MapType,
}					from '@whi/into-struct';


export const HostStruct = {
    "dn":			DnaHash,
    "capabilities":		Object,
    "author":			Number,
    "published_at":		Date,
    "last_updated":		Date,
    "metadata":			Object,
};

export function HostEntry ( entry ) {
    return intoStruct( entry, HostStruct );
}


export default {
    HostStruct,
    HostEntry,
};
