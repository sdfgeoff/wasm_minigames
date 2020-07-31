OUT_DIR = ${CURDIR}/../../website/src/games/$(shell basename $(CURDIR))
SRC_DIR = ${CURDIR}/core


WASM_PACK_FLAGS = build --out-dir $(OUT_DIR) --target web
WASM_PACK = wasm-pack


dev: static_files
	cd $(SRC_DIR); $(WASM_PACK) $(WASM_PACK_FLAGS) --dev

	rm $(OUT_DIR)/core_bg.d.ts
	rm $(OUT_DIR)/core.d.ts
	rm $(OUT_DIR)/package.json


release: static_files
	cd $(SRC_DIR); $(WASM_PACK) $(WASM_PACK_FLAGS) --release

	rm $(OUT_DIR)/core_bg.d.ts
	rm $(OUT_DIR)/core.d.ts
	rm $(OUT_DIR)/package.json
	rm $(OUT_DIR)/.gitignore


fmt:
	cd $(SRC_DIR); cargo fmt

static_files:
	mkdir -p $(OUT_DIR)
	cp $(SRC_DIR)/Cargo.toml $(OUT_DIR)/Cargo.toml
	cp $(SRC_DIR)/src/lib.rs $(OUT_DIR)/lib.rs
