import * as wasm from "./rdfa_wasm_bg.wasm";
import { __wbg_set_wasm } from "./rdfa_wasm_bg.js";
__wbg_set_wasm(wasm);
export * from "./rdfa_wasm_bg.js";
