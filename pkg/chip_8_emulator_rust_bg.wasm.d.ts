/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_cpu_free(a: number): void;
export function __wbg_get_cpu_keypad(a: number): number;
export function __wbg_set_cpu_keypad(a: number, b: number): void;
export function cpu_new(): number;
export function cpu_dump_opcode(a: number): number;
export function cpu_dump_memory(a: number): number;
export function cpu_get_frame_buffers(a: number): number;
export function cpu_load_rom(a: number, b: number, c: number): void;
export function cpu_run_cycle(a: number): void;
export function cpu_key_down(a: number, b: number): void;
export function cpu_key_up(a: number, b: number): void;
export function greet(a: number, b: number): void;
export function __wbg_keypad_free(a: number): void;
export function keypad_new(): number;
export function keypad_key_down(a: number, b: number): void;
export function keypad_key_up(a: number, b: number): void;
export function keypad_key_state(a: number, b: number): number;
export function keypad_get_down_key(a: number): number;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_exn_store(a: number): void;
