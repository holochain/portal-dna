import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-portal", process.env.LOG_LEVEL );

import * as fs				from 'node:fs/promises';
import path				from 'path';
import crypto				from 'crypto';
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
    PortalCSRZomelet,
}					from '@holochain/portal-zomelets';
import {
    AppInterfaceClient,
}					from '@spartan-hc/app-interface-client';

import {
    expect_reject,
    linearSuite,
}					from '../utils.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const PORTAL_DNA_PATH			= path.join( __dirname, "../../portal.dna" );
const APP_PORT				= 23_567;

let client;
let alice_client, alice_csr;
let bobby_client, bobby_csr;


describe("Portal", () => {
    const holochain			= new Holochain({
	"default_stdout_loggers": process.env.LOG_LEVEL === "trace",
    });

    before(async function () {
	this.timeout( 60_000 );

	const actors			= await holochain.backdrop({
	    "test": {
		"portal":	PORTAL_DNA_PATH,
	    },
	}, {
	    "app_port": APP_PORT,
	    "actors": [
		"alice",
		"bobby",
	    ],
	});

	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "fatal",
	});

	alice_client			= await client.app( "test-alice" );
	bobby_client			= await client.app( "test-bobby" );

	// alice_orm			= alice_client.orm;
	// bobby_orm			= bobby_client.orm;

	alice_csr			= alice_client.createZomeInterface( "portal", "portal_csr", PortalCSRZomelet ).functions;
	bobby_csr			= bobby_client.createZomeInterface( "portal", "portal_csr", PortalCSRZomelet ).functions;

	// Must call whoami on each cell to ensure that init has finished.
	{
	    let whoami			= await alice_csr.whoami();
	    log.normal("Alice whoami: %s", whoami.pubkey.initial );
	}
	{
	    let whoami			= await bobby_csr.whoami();
	    log.normal("Bobby whoami: %s", whoami.pubkey.initial );
	}
    });

    linearSuite("Host", host_tests );

    after(async () => {
	await holochain.destroy();
    });

});



function host_tests () {

    it("should register hosts", async function () {
	const host			= await alice_csr.register_host({
	    "dna": "uhC0kXracwD-PyrSU5m_unW3GA7vV1fY1eHH-0qV5HG7Y7s-DwLa5",
	    "granted_functions": {
		"Listed": [
		    [ "testing", "testing" ],
		],
	    },
	});

	await bobby_csr.register_host({
	    "dna": "uhC0kXracwD-PyrSU5m_unW3GA7vV1fY1eHH-0qV5HG7Y7s-DwLa5",
	    "granted_functions": {
		"Listed": [
		    [ "testing", "testing" ],
		],
	    },
	});
    });

    it("should get registered hosts", async function () {
	const hosts			= await alice_csr.get_registered_hosts({
	    "dna": "uhC0kXracwD-PyrSU5m_unW3GA7vV1fY1eHH-0qV5HG7Y7s-DwLa5",
	});

	expect( hosts			).to.have.length( 2 );
    });

    it("should ping host", async function () {
	const resp			= await alice_csr.ping( bobby_client.agent_id );

	expect( resp			).to.be.true;
    });

    linearSuite("Host", async () => {
    });

}
