# COnfigure input/output directories
WORKSPACE_DIR = ${CURDIR}/src_rust/
BOOK_DIR = ${CURDIR}/src_book/
WASM_OUTPUT_FOLDER = ${CURDIR}/bin_wasm/
BOOK_OUTPUT_FOLDER = ${CURDIR}/bin_book/
STATIC_DIR = ${CURDIR}/static

# Programs and build options
MDBOOK = mdbook
WASM_BINDGEN_FLAGS = --target web --no-typescript


# Figure out what targets are available in the cargo workspace
TARGET_FOLDERS = $(dir $(wildcard $(WORKSPACE_DIR)/*/*/Cargo.toml))
TARGET_NAMES = $(notdir $(patsubst %/,%,$(TARGET_FOLDERS)))

# If DEBUG=1, add --debug to the WASM_PACK flags
DEBUG ?= 0
ifeq ($(DEBUG), 1)
    BUILD_FLAGS += --debug
    ARTIFACT_DIR = $(WORKSPACE_DIR)/target/wasm32-unknown-unknown/debug/
else
    BUILD_FLAGS += --release
    ARTIFACT_DIR = $(WORKSPACE_DIR)/target/wasm32-unknown-unknown/release/
endif



# Default target
.PHONY: book
.DEFAULT_GOAL: book


# Package as a statically-serveable bunch of HTML pages that have
# writing about how the programs were made
book: examples
	mkdir -p $(BOOK_DIR)/gen
	
	cp -rf $(STATIC_DIR)/book_extra.js $(BOOK_DIR)/gen
	cp -rf $(STATIC_DIR)/book_extra.css $(BOOK_DIR)/gen
	cp -rf $(STATIC_DIR)/error.svg $(BOOK_DIR)/gen
	cp -rf $(STATIC_DIR)/click_icon.svg $(BOOK_DIR)/gen
	cp -rf $(STATIC_DIR)/loading.gif $(BOOK_DIR)/gen
	
	cp -rf $(WASM_OUTPUT_FOLDER)/*.wasm $(BOOK_DIR)/gen
	cp -rf $(WASM_OUTPUT_FOLDER)/*.js $(BOOK_DIR)/gen
	
	cd $(BOOK_DIR); $(MDBOOK) build -d $(BOOK_OUTPUT_FOLDER)


# Build all rust code into WASM
wasm:
	cd $(WORKSPACE_DIR); cargo build --target wasm32-unknown-unknown $(BUILD_FLAGS)


# Create a full-screen HTML page for each game
examples: $(TARGET_NAMES)
	cp $(STATIC_DIR)/example.css $(WASM_OUTPUT_FOLDER)	
	cp $(STATIC_DIR)/example.js $(WASM_OUTPUT_FOLDER)
	cp $(STATIC_DIR)/error.svg $(WASM_OUTPUT_FOLDER)
	cp $(STATIC_DIR)/click_icon.svg $(WASM_OUTPUT_FOLDER)
	cp $(STATIC_DIR)/loading.gif $(WASM_OUTPUT_FOLDER)
	

# Create bindings for a single WASM file
$(TARGET_NAMES): wasm
	cd $(WORKSPACE_DIR); wasm-bindgen $(ARTIFACT_DIR)/$@.wasm $(WASM_BINDGEN_FLAGS) --out-dir $(WASM_OUTPUT_FOLDER)
	sed 's,{ID},$@,g' $(STATIC_DIR)/example.html > $(WASM_OUTPUT_FOLDER)/$@.html
	

fmt:
	cd $(WORKSPACE_DIR); cargo fmt

clean:
	rm -r $(WASM_OUTPUT_FOLDER)
	rm -r $(BOOK_OUTPUT_FOLDER)
	rm -r $(BOOK_DIR)/gen

