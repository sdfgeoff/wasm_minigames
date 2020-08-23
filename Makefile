# COnfigure input/output directories
WORKSPACE_DIR = ${CURDIR}/src/
BOOK_DIR = ${CURDIR}/

# Programs and build options
MDBOOK = mdbook
WASM_PACK = wasm-pack
WASM_PACK_FLAGS = --target web --no-typescript


# Figure out what targets are available in the cargo workspace
TARGET_FOLDERS = $(dir $(wildcard $(WORKSPACE_DIR)/*/Cargo.toml))
TARGET_NAMES = $(notdir $(patsubst %/,%,$(TARGET_FOLDERS)))


# If DEBUG=1, add --debug to the WASM_PACK flags
DEBUG ?= 0
ifeq ($(DEBUG), 1)
    WASM_PACK_FLAGS += --debug
else
    WASM_PACK_FLAGS += --release
endif


# Default target
all: book

book: wasm
	$(MDBOOK) build $(BOOK_DIR)
	rm $(BOOK_DIR)/book/*/pkg/.gitignore

# Generate a target for each entry in the Cargo workspace and group them
# under "wasm" to build all of them
wasm: $(TARGET_NAMES)
$(TARGET_NAMES):
	cd $(WORKSPACE_DIR); wasm-pack build $(WASM_PACK_FLAGS) $@
