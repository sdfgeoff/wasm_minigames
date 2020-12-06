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

.PHONY: book

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
all: book

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


# Build all wasm
wasm:
	cd $(WORKSPACE_DIR); cargo build --target wasm32-unknown-unknown $(BUILD_FLAGS)


static:
	cp $(STATIC_DIR)/example.css $(WASM_OUTPUT_FOLDER)
	cp $(STATIC_DIR)/example.js $(WASM_OUTPUT_FOLDER)
	cp $(STATIC_DIR)/error.svg $(WASM_OUTPUT_FOLDER)
	cp $(STATIC_DIR)/click_icon.svg $(WASM_OUTPUT_FOLDER)
	cp $(STATIC_DIR)/loading.gif $(WASM_OUTPUT_FOLDER)


examples: $(TARGET_NAMES)
$(TARGET_NAMES): wasm static
	cd $(WORKSPACE_DIR); wasm-bindgen $(ARTIFACT_DIR)/$@.wasm $(WASM_BINDGEN_FLAGS) --out-dir $(WASM_OUTPUT_FOLDER)
	sed 's,{ID},$@,g' $(STATIC_DIR)/example.html > $(WASM_OUTPUT_FOLDER)/$@.html
	

fmt:
	cd $(WORKSPACE_DIR); cargo fmt



