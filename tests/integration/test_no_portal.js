import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-no-portal", process.env.LOG_LEVEL );

import path				from 'path';
import { expect }			from 'chai';

import json				from '@whi/json';
import {
    HoloHash,
    DnaHash, AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash';
import HolochainBackdrop		from '@spartan-hc/holochain-backdrop';
const { Holochain }			= HolochainBackdrop;

import {
    AppInterfaceClient,
}					from '@spartan-hc/app-interface-client';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const TEST_DNA_PATH			= path.join( __dirname, "../content.dna" );

let app_port;
let client;
let alice_client, alice_csr;


describe("No Portal", () => {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": process.env.LOG_LEVEL === "trace",
    });

    before(async function () {
	this.timeout( 60_000 );

	const installations		= await holochain.install([
	    "alice",
	], [
	    {
		"app_name": "test",
		"bundle": {
		    "content":	TEST_DNA_PATH,
		},
	    },
	]);

	app_port			= await holochain.ensureAppPort();

	client				= new AppInterfaceClient( app_port, {
	    "logging": process.env.LOG_LEVEL || "fatal",
	});

	const alice_token		= installations.alice.test.auth.token;
	alice_client			= await client.app( alice_token );
    });

    it("should init and fail silently", async function () {
	this.timeout( 30_000 );

	let whoami			= await alice_client.orm.content.content_csr.whoami();
	log.normal("Alice [content] whoami: %s", new AgentPubKey( whoami.agent_initial_pubkey ) );
    });

    after(async function () {
	this.timeout( 5_000 );
	await holochain.destroy();
    });

});
