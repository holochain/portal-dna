
import { AgentPubKey, DnaHash,
	 ActionHash, EntryHash }	from '@spartan-hc/holo-hash';
import {
    ScopedEntity,
    intoStruct,
    OptionType, VecType, MapType,
}					from '@spartan-hc/caps-entities';



export const HostStruct = {
    "dna":			DnaHash,
    "capabilities":		Object,
    "author":			AgentPubKey,
    "published_at":		Number,
    "last_updated":		Number,
    "metadata":			Object,
};

export function HostEntry ( entry ) {
    return intoStruct( entry, HostStruct );
}

export class Host extends ScopedEntity {
    static STRUCT		= HostStruct;
}



export default {
    HostStruct,
    HostEntry,
    Host,
};
