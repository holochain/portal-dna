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
const TEST_DNA_PATH			= path.join( __dirname, "../content.dna" );
const APP_PORT				= 23_567;

let client;
let alice_client, alice_csr;
let bobby_client, bobby_csr;
let carol_client, carol_csr;


describe("Portal", () => {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": process.env.LOG_LEVEL === "trace",
    });

    before(async function () {
	this.timeout( 60_000 );

	const actors			= await holochain.backdrop({
	    "test": {
		"portal":	PORTAL_DNA_PATH,
		"content":	TEST_DNA_PATH,
	    },
	}, {
	    "app_port": APP_PORT,
	    "actors": [
		"alice",
		"bobby",
		"carol",
	    ],
	});

	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "fatal",
	});

	alice_client			= await client.app( "test-alice" );
	bobby_client			= await client.app( "test-bobby" );
	carol_client			= await client.app( "test-carol" );

	alice_csr			= alice_client.createZomeInterface( "portal", "portal_csr", PortalCSRZomelet ).functions;
	bobby_csr			= bobby_client.createZomeInterface( "portal", "portal_csr", PortalCSRZomelet ).functions;
	carol_csr			= carol_client.createZomeInterface( "portal", "portal_csr", PortalCSRZomelet ).functions;

	// Must call whoami on each cell to ensure that init has finished.
	{
	    let whoami			= await alice_csr.whoami();
	    log.normal("Alice whoami: %s", whoami.pubkey.initial );
	}
	{
	    let whoami			= await bobby_csr.whoami();
	    log.normal("Bobby whoami: %s", whoami.pubkey.initial );
	}
	{
	    let whoami			= await carol_csr.whoami();
	    log.normal("Carol whoami: %s", whoami.pubkey.initial );
	}
    });

    linearSuite("Host", function () { host_tests.call( this, holochain ) });

    after(async () => {
	await holochain.destroy();
    });

});



const dna_hash				= "uhC0kXracwD-PyrSU5m_unW3GA7vV1fY1eHH-0qV5HG7Y7s-DwLa5";
const zome_funcs			= {
    "zome_name": [
	"func_name",
    ],
}

function host_tests ( holochain ) {

    it("should register hosts", async function () {
	await bobby_csr.register_host({
	    "dna": dna_hash,
	    "zomes": zome_funcs,
	});

	await carol_csr.register_host({
	    "dna": dna_hash,
	    "zomes": zome_funcs,
	});

	await holochain.admin.disableApp( "test-carol" );
    });

    it("should get registered hosts", async function () {
	const hosts			= await alice_csr.get_registered_hosts( dna_hash );

	expect( hosts			).to.have.length( 2 );
    });

    it("should ping host", async function () {
	const resp			= await alice_csr.ping( bobby_client.agent_id );

	expect( resp			).to.be.true;
    });

    it("should get hosts for zome/function", async function () {
	const hosts		= await alice_csr.get_hosts_for_zome_function({
	    "dna": dna_hash,
	    "zome": "zome_name",
	    "function": "func_name",
	});

	expect( hosts			).to.have.length( 2 );
    });

    it("should get an available host for zome/function", async function () {
	const host_pubkey		= await alice_csr.get_available_host_for_zome_function({
	    "dna": dna_hash,
	    "zome": "zome_name",
	    "function": "func_name",
	});

	expect( host_pubkey		).to.deep.equal( bobby_client.agent_id );
    });

    linearSuite("Errors", async () => {

	it("should fail to get hosts for zome/function because invalid 'dna' input", async function () {
	    await expect_reject( async () => {
		await alice_csr.get_hosts_for_zome_function({
		    "dna": alice_client.agent_id,
		});
	    }, "prefix did not match" );
	});

	it("should fail to get hosts for zome/function because invalid 'zome' input", async function () {
	    await expect_reject( async () => {
		await alice_csr.get_hosts_for_zome_function({
		    "dna": dna_hash,
		    "zome": null,
		});
	    }, "Expected 'zome' input to be a string" );
	});

	it("should fail to get hosts for zome/function because invalid 'function' input", async function () {
	    await expect_reject( async () => {
		await alice_csr.get_hosts_for_zome_function({
		    "dna": dna_hash,
		    "zome": "zome_name",
		    "function": null,
		});
	    }, "Expected 'function' input to be a string" );
	});

	it("should fail to ping host", async function () {
	    await expect_reject( async () => {
		await alice_csr.ping( carol_client.agent_id );
	    }, "within 1 second(s)" );
	});

    });

}
