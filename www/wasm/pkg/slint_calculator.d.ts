/* tslint:disable */
/* eslint-disable */

export function wasm_main(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly wasm_main: () => void;
    readonly send_keyboard_string_sequence: (a: number, b: number) => void;
    readonly slint_get_mocked_time: () => bigint;
    readonly slint_mock_elapsed_time: (a: bigint) => void;
    readonly slint_send_keyboard_char: (a: number, b: number, c: number) => void;
    readonly slint_send_keyboard_key_text: (a: number, b: number, c: number) => void;
    readonly slint_send_mouse_click: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_4947: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_5023: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_9413: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_9413_3: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_2277: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_2277_5: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_2277_6: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_9413_7: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_9413_8: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_2277_9: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_2277_10: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_9413_11: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_9413_12: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_9413_13: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_2277_14: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_9413_15: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_9412: (a: number, b: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
