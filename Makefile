WORKSPACE_DIR = ${CURDIR}/src/
WASM_PACK = wasm-pack
WASM_PACK_FLAGS = --target web --no-typescript

BOOK_DIR = ${CURDIR}/src/

MDBOOK = mdbook

all: book


book: wasm
	$(MDBOOK) build $(BOOK_DOR)

wasm:
	cd $(WORKSPACE_DIR); wasm-pack build $(WASM_PACK_FLAGS) a_first_shader --release
	cd $(WORKSPACE_DIR); wasm-pack build $(WASM_PACK_FLAGS) binding_events --release
	cd $(WORKSPACE_DIR); wasm-pack build $(WASM_PACK_FLAGS) binding_textures --release
	cd $(WORKSPACE_DIR); wasm-pack build $(WASM_PACK_FLAGS) building_and_loading_wasm --release
	cd $(WORKSPACE_DIR); wasm-pack build $(WASM_PACK_FLAGS) cancel_load_animation --release
	cd $(WORKSPACE_DIR); wasm-pack build $(WASM_PACK_FLAGS) fixing_resolution --release
	cd $(WORKSPACE_DIR); wasm-pack build $(WASM_PACK_FLAGS) passing_in_uniforms --release
