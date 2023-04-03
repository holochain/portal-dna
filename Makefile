
SHELL			= bash

PORTAL_DNA		= bundled/portal.dna
TARGET			= release

# Zomes (WASM)
PORTAL_WASM		= zomes/portal.wasm
PORTAL_API_WASM		= zomes/portal_api.wasm


#
# Project
#
preview-crate:			test-debug
	cd portal_types; cargo publish --dry-run
publish-crate:			test-debug
	cd portal_types; CARGO_HOME=$(HOME)/.cargo cargo publish

tests/package-lock.json:	tests/package.json
	touch $@
tests/node_modules:		tests/package-lock.json
	cd tests; \
	npm install
	touch $@
clean:
	rm -rf \
	    tests/node_modules \
	    .cargo \
	    target \
	    zomes/target \
	    $(PORTAL_DNA) \
	    $(PORTAL_WASM) $(PORTAL_API_WASM)

rebuild:			clean build


$(PORTAL_DNA):			$(PORTAL_WASM) $(PORTAL_API_WASM)

bundled/%.dna:			bundled/%/dna.yaml
	@echo "Packaging '$*': $@"
	@hc dna pack -o $@ bundled/$*
zomes/%.wasm:			zomes/target/wasm32-unknown-unknown/release/%.wasm
	cp $< $@
zomes/target/wasm32-unknown-unknown/release/%.wasm:	Makefile zomes/%/src/*.rs zomes/%/Cargo.toml zomes/%/Cargo.lock *_types/* *_types/*/*
	@echo "Building  '$*' WASM: $@"; \
	cd zomes; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time
zomes/%/Cargo.lock:
	touch $@

$(MERE_MEMORY_WASM):
	curl --fail -L "https://github.com/mjbrisebois/hc-zome-mere-memory/releases/download/v$$(echo $(NEW_MM_VERSION))/mere_memory.wasm" --output $@
$(MERE_MEMORY_API_WASM):
	curl --fail -L "https://github.com/mjbrisebois/hc-zome-mere-memory/releases/download/v$$(echo $(NEW_MM_VERSION))/mere_memory_api.wasm" --output $@

tests/devhub/%.dna:
	wget -O $@ "https://github.com/holochain/devhub-dnas/releases/download/$(DEVHUB_VERSION)/$*.dna"

use-local-client:
	cd tests; npm uninstall @whi/holochain-client
	cd tests; npm install --save ../../js-holochain-client/whi-holochain-client-0.78.0.tgz
use-npm-client:
	cd tests; npm uninstall @whi/holochain-client
	cd tests; npm install --save @whi/holochain-client

use-local-backdrop:
	cd tests; npm uninstall @whi/holochain-backdrop
	cd tests; npm install --save ../../node-holochain-backdrop/
use-npm-backdrop:
	cd tests; npm uninstall @whi/holochain-backdrop
	cd tests; npm install --save @whi/holochain-backdrop



#
# Testing
#
test:				test-unit test-integration
test-debug:			test-unit test-integration-debug

test-unit:			test-unit-portal test-unit-portal_api
test-unit-%:
	cd zomes;		RUST_BACKTRACE=1 cargo test $* -- --nocapture


# DNAs
test-setup:			tests/node_modules

test-integration:		test-setup	test-portal
test-integration-debug:		test-setup	test-portal-debug

test-portal:			test-setup $(PORTAL_DNA)
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_portal.js
test-portal-debug:		test-setup $(PORTAL_DNA)
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_portal.js



#
# Repository
#
clean-remove-chaff:
	@find . -name '*~' -exec rm {} \;
clean-files:		clean-remove-chaff
	git clean -nd
clean-files-force:	clean-remove-chaff
	git clean -fd
clean-files-all:	clean-remove-chaff
	git clean -ndx
clean-files-all-force:	clean-remove-chaff
	git clean -fdx

PRE_HDK_VERSION = "0.1.0"
NEW_HDK_VERSION = "0.2.0-beta-rc.1"

PRE_HDI_VERSION = "0.2.0"
NEW_HDI_VERSION = "0.3.0-beta-rc.1"

PRE_CRUD_VERSION = "0.3.0"
NEW_CRUD_VERSION = "0.4.0"

PRE_MM_VERSION = "0.79.0"
NEW_MM_VERSION = "0.80.0"

GG_REPLACE_LOCATIONS = ':(exclude)*.lock' zomes/*/ *_types/ hc_utils

update-hdk-version:
	git grep -l $(PRE_HDK_VERSION) -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's/$(PRE_HDK_VERSION)/$(NEW_HDK_VERSION)/g'
update-hdi-version:
	git grep -l $(PRE_HDI_VERSION) -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's/$(PRE_HDI_VERSION)/$(NEW_HDI_VERSION)/g'
update-crud-version:
	git grep -l $(PRE_CRUD_VERSION) -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's/$(PRE_CRUD_VERSION)/$(NEW_CRUD_VERSION)/g'
update-mere-memory-version:
	rm -f zomes/mere_memory*.wasm
#	git grep -l $(PRE_MM_VERSION) -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's/$(PRE_MM_VERSION)/$(NEW_MM_VERSION)/g'
