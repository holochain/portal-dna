.PHONY:			FORCE

SHELL			= bash
NAME			= portal

# DNA
PORTAL_DNA		= $(NAME).dna

# Zomes (WASM)
PORTAL_WASM		= zomes/portal.wasm
PORTAL_CSR_WASM		= zomes/portal_csr.wasm

TARGET			= release
TARGET_DIR		= zomes/target/wasm32-unknown-unknown/release
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
%/package-lock.json:		%/package.json
	touch $@
%/node_modules:			%/package-lock.json
	cd $*; \
	npm install
	touch $@
clean:
	rm -rf \
	    tests/node_modules \
	    zomelets/node_modules \
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

PRE_EDITION = edition = "2018"
NEW_EDITION = edition = "2021"

PRE_HDIE_VERSION = whi_hdi_extensions = "0.13"
NEW_HDIE_VERSION = whi_hdi_extensions = "0.14"

PRE_HDKE_VERSION = whi_hdk_extensions = "0.13"
NEW_HDKE_VERSION = whi_hdk_extensions = "0.14"

PRE_CRUD_VERSION = hc_crud_caps = "0.18"
NEW_CRUD_VERSION = hc_crud_caps = "0.19"

GG_REPLACE_LOCATIONS = ':(exclude)*.lock' zomes/*/ types sdk tests/zomes
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
    SED_INPLACE := sed -i ''
else
    SED_INPLACE := sed -i
endif

update-all-version:
	rm -fr target;
	make -s update-hdk-version
	make -s update-hdi-version
	make -s update-crud-version

update-hdk-version:
	git grep -l '$(PRE_HDKE_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs $(SED_INPLACE) 's/$(PRE_HDKE_VERSION)/$(NEW_HDKE_VERSION)/g'
update-hdi-version:
	git grep -l '$(PRE_HDIE_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs $(SED_INPLACE) 's/$(PRE_HDIE_VERSION)/$(NEW_HDIE_VERSION)/g'
update-crud-version:
	git grep -l '$(PRE_CRUD_VERSION)' -- $(GG_REPLACE_LOCATIONS) | xargs $(SED_INPLACE) 's/$(PRE_CRUD_VERSION)/$(NEW_CRUD_VERSION)/g'
update-edition:
	git grep -l '$(PRE_EDITION)' -- $(GG_REPLACE_LOCATIONS) | xargs $(SED_INPLACE) 's/$(PRE_EDITION)/$(NEW_EDITION)/g'

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
	NPM_PACKAGE=@spartan-hc/holochain-backdrop LOCAL_PATH=../../node-backdrop make npm-reinstall-$*

last-release:
	@git tag --list --sort=committerdate | grep -e '^v.*' | tail -n 1

RELEASE_VERSION = v0.9.2
RELEASE_PREP_DIR = ~/Downloads/Portal\ DNA\ Release\ Assets
prepare-release:		$(PORTAL_DNA)
	mkdir -p $(RELEASE_PREP_DIR)
	cp portal.dna zomes/*.wasm $(RELEASE_PREP_DIR)
	@if [ "$$(make -s last-release)" == "$(RELEASE_VERSION)" ]; then \
		echo -e "\n\x1b[33mWARNING: current release version matches Makefile ($(RELEASE_VERSION)) \x1b[0m"; \
	else \
		git tag $(RELEASE_VERSION); \
	fi
	@echo -e "\n\x1b[37mRelease Asset Sha1sums\x1b[0m\n";
	@bash -c "cd $(RELEASE_PREP_DIR); sha1sum *.dna *.wasm; echo ''"
	ls -l $(RELEASE_PREP_DIR)


#
# Rust Packages
#
.cargo/credentials:
	mkdir -p .cargo
	cp ~/$@ $@
preview-types-crate:
	DEBUG_LEVEL=trace make -s test
	cd types; cargo publish --dry-run --allow-dirty
	touch types/src/lib.rs # Force rebuild to fix 'missing debug macro' issue after dry run
publish-types-crate:		.cargo/credentials
	DEBUG_LEVEL=trace make -s test
	cd types; cargo publish
	touch types/src/lib.rs # Force rebuild to fix 'missing debug macro' issue after dry run
preview-sdk-crate:
	DEBUG_LEVEL=trace make -s test
	cd sdk; cargo publish --dry-run --allow-dirty
	touch sdk/src/lib.rs # Force rebuild to fix 'missing debug macro' issue after dry run
publish-sdk-crate:		.cargo/credentials
	DEBUG_LEVEL=trace make -s test
	cd sdk; cargo publish
	touch sdk/src/lib.rs # Force rebuild to fix 'missing debug macro' issue after dry run



#
# Testing
#
DEBUG_LEVEL	       ?= warn
TEST_ENV_VARS		= LOG_LEVEL=$(DEBUG_LEVEL)
MOCHA_OPTS		= -n enable-source-maps

tests/%.dna:			FORCE
	cd tests; make $*.dna
test:
	make -s test-unit-debug
	make -s test-integration

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
test-setup:			tests/node_modules zomelets/node_modules

test-integration:
	make -s test-portal
	make -s test-no-portal

CONTENT_DNA			= tests/content.dna

test-portal:			test-setup $(PORTAL_DNA) $(CONTENT_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) integration/test_portal.js

test-no-portal:			test-setup $(CONTENT_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) integration/test_no_portal.js



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
preview-zomelets-package:	clean-files prepare-zomelets-package
	DEBUG_LEVEL=trace make -s test
	cd zomelets; npm pack --dry-run .
create-zomelets-package:	clean-files prepare-zomelets-package
	DEBUG_LEVEL=trace make -s test
	cd zomelets; npm pack .
publish-zomelets-package:	clean-files prepare-zomelets-package
	DEBUG_LEVEL=trace make -s test
	cd zomelets; npm publish --access public .
