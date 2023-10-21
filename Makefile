.PHONY:			FORCE

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
				zomes/%/Cargo.toml zomes/%/src/*.rs \
				types/Cargo.toml types/src/*.rs
CSR_SOURCE_FILES	= $(COMMON_SOURCE_FILES) $(INT_SOURCE_FILES) \
				zomes/%_csr/Cargo.toml zomes/%_csr/src/*.rs \
				types/Cargo.toml types/src/*.rs \
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
# Rust Packages
#
.cargo/credentials:
	cp ~/$@ $@
preview-types-crate:		test-debug
	cd types; cargo publish --dry-run --allow-dirty
	touch types/src/lib.rs # Force rebuild to fix 'missing debug macro' issue after dry run
publish-types-crate:		test-debug .cargo/credentials
	cd types; cargo publish
	touch types/src/lib.rs # Force rebuild to fix 'missing debug macro' issue after dry run
preview-sdk-crate:		test-debug
	cd sdk; cargo publish --dry-run --allow-dirty
	touch sdk/src/lib.rs # Force rebuild to fix 'missing debug macro' issue after dry run
publish-sdk-crate:		test-debug .cargo/credentials
	cd sdk; cargo publish
	touch types/src/lib.rs # Force rebuild to fix 'missing debug macro' issue after dry run



#
# Testing
#
CONTENT_DNA			= tests/content.dna

tests/%.dna:			FORCE
	cd tests; make $*.dna
test:				test-unit test-integration
test-debug:			test-unit test-integration-debug

# Unit tests
test-crate:
	cd $(SRC); CARGO_TARGET_DIR=../target cargo test --quiet --tests
test-crate-debug:
	cd $(SRC); RUST_BACKTRACE=1 CARGO_TARGET_DIR=../target cargo test -- --nocapture --show-output
test-unit:
	SRC=zomes make test-crate
test-unit-debug:
	SRC=zomes make test-crate-debug

# Integration tests
test-setup:			tests/node_modules

test-integration:
	make test-portal
test-integration-debug:
	make test-portal-debug

test-portal:			test-setup $(PORTAL_DNA) $(CONTENT_DNA)
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_portal.js
test-portal-debug:		test-setup $(PORTAL_DNA) $(CONTENT_DNA)
	cd tests; RUST_LOG=info LOG_LEVEL=trace npx mocha integration/test_portal.js



#
# Documentation
#
TYPES_DOCS		= target/doc/portal_types/index.html
SDK_DOCS		= target/doc/portal_sdk/index.html

docs:			$(TYPES_DOCS) $(SDK_DOCS)
$(TYPES_DOCS):		types/src/*
	cd types; cargo test --doc
	cd types; cargo doc
	@echo -e "\x1b[37mOpen docs in file://$(shell pwd)/$(TYPES_DOCS)\x1b[0m";
	touch types/src/lib.rs # Force rebuild to fix compile issue
	touch $(TYPES_DOCS)
$(SDK_DOCS):		sdk/src/*
	cd sdk; cargo test --doc
	cd sdk; cargo doc
	@echo -e "\x1b[37mOpen docs in file://$(shell pwd)/$(SDK_DOCS)\x1b[0m";
	touch sdk/src/lib.rs # Force rebuild to fix compile issue
	touch $(SDK_DOCS)



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
preview-zomelets-package:	clean-files test-debug prepare-zomelets-package
	cd zomelets; npm pack --dry-run .
create-zomelets-package:	clean-files test-debug prepare-zomelets-package
	cd zomelets; npm pack .
publish-zomelets-package:	clean-files test-debug prepare-zomelets-package
	cd zomelets; npm publish --access public .
