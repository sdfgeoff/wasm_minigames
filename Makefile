# Configure input/output directories
WORKSPACE_DIR = ${CURDIR}/src_rust/
BOOK_DIR = ${CURDIR}/src_book/
BOOK_GENERATED_CONTENT_DIR = ${BOOK_DIR}/content/gen
WASM_OUTPUT_FOLDER = ${CURDIR}/bin_wasm/
BOOK_OUTPUT_FOLDER = ${CURDIR}/bin_book/
STATIC_DIR = ${CURDIR}/static

# Programs and build options
MDBOOK = mdbook
WASM_BINDGEN_FLAGS = --target web --no-typescript


# Figure out what targets are available in the cargo workspace
TARGET_FOLDERS = $(dir $(wildcard $(WORKSPACE_DIR)/*/*/Cargo.toml))
TARGET_NAMES = $(foreach target_folder, $(TARGET_FOLDERS), $(patsubst ${WORKSPACE_DIR}/%/,%,$(target_folder)))


# Default target
.PHONY: book wasm
.DEFAULT_GOAL: book

# Disable default rules for easier debugging with -d flag
.SUFFIXES:



# Package as a statically-serveable bunch of HTML pages that have
# writing about how the programs were made
book: examples
	mkdir -p $(BOOK_GENERATED_CONTENT_DIR)
	cp -rf $(WASM_OUTPUT_FOLDER)/* $(BOOK_GENERATED_CONTENT_DIR)
	
	cd $(BOOK_DIR); $(MDBOOK) build -d $(BOOK_OUTPUT_FOLDER)


# Create all games
examples: $(TARGET_NAMES)


# Build a single example
$(TARGET_NAMES): %: $(WASM_OUTPUT_FOLDER)/%/game_bg.wasm $(WASM_OUTPUT_FOLDER)/%/game.html


	
$(WASM_OUTPUT_FOLDER)%/game.html: $(shell find $(STATIC_DIR))
	mkdir -p $(dir $@)
	# Most static files we just copy
	cp -r $(STATIC_DIR)/* $(dir $@)
	rm $(dir $@)/example.html
	
	# But we have to tell the name of the game to the HTML page
	sed 's,{ID},$(notdir $*),g' $(STATIC_DIR)/example.html > $@
	
	


# Create bindings for a single game
$(WASM_OUTPUT_FOLDER)%/game_bg.wasm: $(shell find $(WORKSPACE_DIR)/**/*)
	cd $(WORKSPACE_DIR)/$*; wasm-pack build  $(WASM_BINDGEN_FLAGS) --out-dir $(WASM_OUTPUT_FOLDER)/$* --out-name game
	rm $(WASM_OUTPUT_FOLDER)/$*/package.json


fmt:
	cd $(WORKSPACE_DIR); cargo fmt

clean:
	rm -r $(WASM_OUTPUT_FOLDER)
	rm -r $(BOOK_OUTPUT_FOLDER)
	rm -r $(BOOK_GENERATED_CONTENT_DIR)

