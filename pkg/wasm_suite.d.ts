/* tslint:disable */
/* eslint-disable */
/**
* Generate a testnet2 address from a private key
* @param {string} private_key
* @returns {string}
*/
export function testnet2_address(private_key: string): string;
/**
* Generate a mainnet address from a mainnet private key
* @param {string} private_key
* @returns {string}
*/
export function mainnet_address(private_key: string): string;
/**
* Sign a message with a mainnet private key
* @param {string} private_key
* @param {string} message
* @returns {string}
*/
export function testnet2_sign(private_key: string, message: string): string;
/**
* Verify a signature with a testnet2 address and message
* @param {string} address
* @param {string} message
* @param {string} signature
* @returns {boolean}
*/
export function testnet2_verify(address: string, message: string, signature: string): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly testnet2_address: (a: number, b: number, c: number) => void;
  readonly mainnet_address: (a: number, b: number, c: number) => void;
  readonly testnet2_sign: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly testnet2_verify: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
