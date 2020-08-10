OUT_DIR = ${CURDIR}/
SRC_DIR = ${CURDIR}/


WASM_PACK_FLAGS = build --out-dir $(OUT_DIR) --target web
WASM_PACK = wasm-pack


dev:
	cd $(SRC_DIR); $(WASM_PACK) $(WASM_PACK_FLAGS) --dev

	rm $(OUT_DIR)/core_bg.d.ts
	rm $(OUT_DIR)/core.d.ts
	rm $(OUT_DIR)/package.json
	rm $(OUT_DIR)/.gitignore


release:
	cd $(SRC_DIR); $(WASM_PACK) $(WASM_PACK_FLAGS) --release

	rm $(OUT_DIR)/core_bg.d.ts
	rm $(OUT_DIR)/core.d.ts
	rm $(OUT_DIR)/package.json
	rm $(OUT_DIR)/.gitignore


fmt:
	cd $(SRC_DIR); cargo fmt

