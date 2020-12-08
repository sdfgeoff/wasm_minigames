# COnfigure input/output directories
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


# If DEBUG=1, add --debug to the WASM_PACK flags
DEBUG ?= 0
ifeq ($(DEBUG), 1)
    BUILD_FLAGS += 
    ARTIFACT_DIR = $(WORKSPACE_DIR)/target/wasm32-unknown-unknown/debug/
else
    BUILD_FLAGS += --release
    ARTIFACT_DIR = $(WORKSPACE_DIR)/target/wasm32-unknown-unknown/release/
endif

TARGET_NAMES = $(notdir $(patsubst %/,%,$(TARGET_FOLDERS)))
WASM_ARTIFACT_NAMES = $(patsubst %,$(WASM_OUTPUT_FOLDER)%_bg.wasm,$(TARGET_NAMES))
WASM_RAW_NAMES = $(patsubst %,$(ARTIFACT_DIR)%.wasm,$(TARGET_NAMES))
JS_ARTIFACT_NAMES = $(patsubst %,$(WASM_OUTPUT_FOLDER)%.js,$(TARGET_NAMES))
HTML_ARTIFACT_NAMES = $(patsubst %,$(WASM_OUTPUT_FOLDER)%.html,$(TARGET_NAMES))



# Default target
.PHONY: book wasm
.DEFAULT_GOAL: book


# Package as a statically-serveable bunch of HTML pages that have
# writing about how the programs were made
book: examples
	mkdir -p $(BOOK_GENERATED_CONTENT_DIR)
	cp -rf $(WASM_OUTPUT_FOLDER)/* $(BOOK_GENERATED_CONTENT_DIR)
	
	cd $(BOOK_DIR); $(MDBOOK) build -d $(BOOK_OUTPUT_FOLDER)


# Build all rust code into WASM
wasm:
	cd $(WORKSPACE_DIR); cargo build --target wasm32-unknown-unknown $(BUILD_FLAGS)


# Create a full-screen HTML page for each game
examples: $(TARGET_NAMES)
$(TARGET_NAMES): $(JS_ARTIFACT_NAMES) $(WASM_ARTIFACT_NAMES) $(HTML_ARTIFACT_NAMES) example_static_files

example_static_files: 
	cp $(STATIC_DIR)/example.css $(WASM_OUTPUT_FOLDER)	
	cp $(STATIC_DIR)/example.js $(WASM_OUTPUT_FOLDER)
	cp $(STATIC_DIR)/error.svg $(WASM_OUTPUT_FOLDER)
	cp $(STATIC_DIR)/click_icon.svg $(WASM_OUTPUT_FOLDER)
	cp $(STATIC_DIR)/loading.gif $(WASM_OUTPUT_FOLDER)


$(WASM_RAW_NAMES): wasm

# Create the "Example" page for a single game
$(WASM_OUTPUT_FOLDER)%.html: $(ARTIFACT_DIR)%.wasm 
	sed 's,{ID},$*,g' $(STATIC_DIR)/example.html > $@

# Create bindings for a single game
$(WASM_OUTPUT_FOLDER)%.js $(WASM_OUTPUT_FOLDER)%_bg.wasm: $(ARTIFACT_DIR)%.wasm
	cd $(WORKSPACE_DIR); wasm-bindgen $< $(WASM_BINDGEN_FLAGS) --out-dir $(WASM_OUTPUT_FOLDER)






fmt:
	cd $(WORKSPACE_DIR); cargo fmt

clean:
	rm -r $(WASM_OUTPUT_FOLDER)
	rm -r $(BOOK_OUTPUT_FOLDER)
	rm -r $(BOOK_GENERATED_CONTENT_DIR)

