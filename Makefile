
SHELL			= bash
NAME			= portal

# DNA
PORTAL_DNA		= $(NAME).dna

# Zomes (WASM)
PORTAL_WASM		= zomes/portal.wasm
PORTAL_CSR_WASM		= zomes/portal_csr.wasm

TARGET			= release
TARGET_DIR		= target/wasm32-unknown-unknown/release
COMMON_SOURCE_FILES	= Makefile zomes/Cargo.toml
INT_SOURCE_FILES	= $(COMMON_SOURCE_FILES) \
				types/Cargo.toml types/src/*.rs \
				scoped_types/Cargo.toml scoped_types/src/*.rs \
				zomes/%/Cargo.toml zomes/%/src/*.rs
CSR_SOURCE_FILES	= $(COMMON_SOURCE_FILES) $(INT_SOURCE_FILES) \
				zomes/%_csr/Cargo.toml zomes/%_csr/src/*.rs \
				sdk/Cargo.toml sdk/src/*.rs


#
# Project
#
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
	    $(PORTAL_DNA) \
	    $(PORTAL_WASM) $(PORTAL_CSR_WASM)

rebuild:			clean build


$(PORTAL_DNA):			$(PORTAL_WASM) $(PORTAL_CSR_WASM)
	@echo "Packaging 'portal': $@"
	@hc dna pack -o $@ .
zomes/%.wasm:			$(TARGET_DIR)/%.wasm
	cp $< $@

$(TARGET_DIR)/%.wasm:		$(INT_SOURCE_FILES)
	rm -f zomes/$*.wasm
	@echo -e "\x1b[37mBuilding zome '$*' -> $@\x1b[0m";
	cd zomes; \
	RUST_BACKTRACE=1 cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time
$(TARGET_DIR)/%_csr.wasm:	$(CSR_SOURCE_FILES)
	rm -f zomes/$*_csr.wasm
	@echo -e "\x1b[37mBuilding zome '$*_csr' -> $@\x1b[0m";
	cd zomes; \
	RUST_BACKTRACE=1 cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*_csr
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time

PRE_HDIE_VERSION = whi_hdi_extensions = "0.4"
NEW_HDIE_VERSION = whi_hdi_extensions = "0.4"

PRE_HDKE_VERSION = whi_hdk_extensions = "0.4"
NEW_HDKE_VERSION = whi_hdk_extensions = "0.4"

PRE_CRUD_VERSION = hc_crud_caps = "0.9.0"
NEW_CRUD_VERSION = hc_crud_caps = "0.10.2"

GG_REPLACE_LOCATIONS = ':(exclude)*.lock' zomes/*/ *_types/ hc_utils

update-all-version:
	rm -r target;
	make update-hdk-version;
	make update-hdi-version;
	make update-crud-version;
update-hdk-version:
	git grep -l $(PRE_HDK_VERSION) -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's/$(PRE_HDK_VERSION)/$(NEW_HDK_VERSION)/g'
update-hdi-version:
	git grep -l $(PRE_HDI_VERSION) -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's/$(PRE_HDI_VERSION)/$(NEW_HDI_VERSION)/g'
update-crud-version:
	git grep -l $(PRE_CRUD_VERSION) -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's/$(PRE_CRUD_VERSION)/$(NEW_CRUD_VERSION)/g'

npm-reinstall-local:
	cd tests; npm uninstall $(NPM_PACKAGE); npm i --save $(LOCAL_PATH)
npm-reinstall-public:
	cd tests; npm uninstall $(NPM_PACKAGE); npm i --save $(NPM_PACKAGE)
npm-use-app-interface-client-public:
npm-use-app-interface-client-local:
npm-use-app-interface-client-%:
	NPM_PACKAGE=@spartan-hc/app-interface-client LOCAL_PATH=../../app-interface-client-js make npm-reinstall-$*
npm-use-backdrop-public:
npm-use-backdrop-local:
npm-use-backdrop-%:
	NPM_PACKAGE=@spartan-hc/holochain-backdrop LOCAL_PATH=../../node-holochain-backdrop make npm-reinstall-$*



#
# Packages
#
preview-crate:			test-debug
	cd portal_types; cargo publish --dry-run --allow-dirty
	touch portal_types/src/lib.rs # Force rebuild to fix issue after dry run
publish-crate:			test-debug .cargo/credentials
	cd portal_types; cargo publish
.cargo/credentials:
	cp ~/$@ $@



#
# Testing
#
test:				test-unit test-integration
test-debug:			test-unit test-integration-debug

test-unit:			test-unit-portal test-unit-portal_csr
test-unit-%:
	cd zomes;		RUST_BACKTRACE=1 cargo test $* -- --nocapture



# DNAs
test-setup:			tests/node_modules

test-integration:
	make test-portal
test-integration-debug:
	make test-portal-debug

test-portal:			test-setup $(PORTAL_DNA)
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_portal.js
test-portal-debug:		test-setup $(PORTAL_DNA)
	cd tests; RUST_LOG=info LOG_LEVEL=trace npx mocha integration/test_portal.js



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



#
# NPM packaging
#
prepare-zomelets-package:
	cd zomelets; rm -f dist/*
	cd zomelets; npx webpack
	cd zomelets; MODE=production npx webpack
	cd zomelets; gzip -kf dist/*.js
preview-zomelets-package:	clean-files test prepare-zomelets-package
	cd zomelets; npm pack --dry-run .
create-zomelets-package:	clean-files test prepare-zomelets-package
	cd zomelets; npm pack .
publish-zomelets-package:	clean-files test prepare-zomelets-package
	cd zomelets; npm publish --access public .
