
COMMON_SOURCE_FILES	= Makefile zomes/Cargo.toml zomes/Cargo.lock
DNA_SOURCE_FILES	= dnas/%/dna.yaml \
				zomes/*/Cargo.toml zomes/*/src/*.rs \
				$(COMMON_SOURCE_FILES)
ZOME_SOURCE_FILES	= zomes/%/Cargo.toml zomes/%/src/*.rs \
				../types/Cargo.toml ../types/src/*.rs \
				../sdk/Cargo.toml ../sdk/src/*.rs \
				$(COMMON_SOURCE_FILES)

# MODE			= production
MODE			= development

ifeq ($(MODE), development)
    CARGO_OPTS		=
    TARGET_DIR		= target/wasm32-unknown-unknown/debug
else ifeq ($(MODE), production)
    CARGO_OPTS		= --release
    TARGET_DIR		= target/wasm32-unknown-unknown/release
else
    $(error Invalid MODE definition '$(MODE)'; expected development/production)
endif

zomes/Cargo.lock:
	touch $@
%.dna:			$(DNA_SOURCE_FILES)
	rm -f $@
	@find zomes -maxdepth 1 -type d ! -path zomes -exec make {}.wasm \;
	@echo -e "\x1b[37mPackaging DNA '$@'\x1b[0m"
	@hc dna pack -o $@ dnas/$* 2> >(head -n 1 | sed 's/.*/\x1b[31;1m&\x1b[0m/')

zomes/%.wasm:			$(TARGET_DIR)/%.wasm
	@echo -e "\x1b[38;2mCopying WASM ($<) to 'zomes' directory: $@\x1b[0m"; \
	cp $< $@
$(TARGET_DIR)/%.wasm:		$(ZOME_SOURCE_FILES)
	rm -f zomes/$*.wasm
	@echo -e "\x1b[37mBuilding zome '$*' -> $@\x1b[0m"; \
	cd zomes; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=../target cargo build $(CARGO_OPTS) \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time
