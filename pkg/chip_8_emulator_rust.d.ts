/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
*/
export class CPU {
  free(): void;
/**
*/
  constructor();
/**
* @returns {number}
*/
  dump_opcode(): number;
/**
* @returns {Uint8Array}
*/
  dump_memory(): Uint8Array;
/**
* @returns {Uint8Array}
*/
  get_frame_buffers(): Uint8Array;
/**
* @param {Uint8Array} rom
*/
  load_rom(rom: Uint8Array): void;
/**
*/
  run_cycle(): void;
/**
* @param {string} key
*/
  key_down(key: string): void;
/**
* @param {string} key
*/
  key_up(key: string): void;
/**
*/
  keypad: Keypad;
}
/**
*/
export class Keypad {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} key
*/
  key_down(key: string): void;
/**
* @param {string} key
*/
  key_up(key: string): void;
/**
* @param {number} key
* @returns {number}
*/
  key_state(key: number): number;
/**
* @returns {number | undefined}
*/
  get_down_key(): number | undefined;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_cpu_free: (a: number) => void;
  readonly __wbg_get_cpu_keypad: (a: number) => number;
  readonly __wbg_set_cpu_keypad: (a: number, b: number) => void;
  readonly cpu_new: () => number;
  readonly cpu_dump_opcode: (a: number) => number;
  readonly cpu_dump_memory: (a: number) => number;
  readonly cpu_get_frame_buffers: (a: number) => number;
  readonly cpu_load_rom: (a: number, b: number, c: number) => void;
  readonly cpu_run_cycle: (a: number) => void;
  readonly cpu_key_down: (a: number, b: number) => void;
  readonly cpu_key_up: (a: number, b: number) => void;
  readonly greet: (a: number, b: number) => void;
  readonly __wbg_keypad_free: (a: number) => void;
  readonly keypad_new: () => number;
  readonly keypad_key_down: (a: number, b: number) => void;
  readonly keypad_key_up: (a: number, b: number) => void;
  readonly keypad_key_state: (a: number, b: number) => number;
  readonly keypad_get_down_key: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
