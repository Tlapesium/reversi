/* tslint:disable */
/* eslint-disable */
/**
*/
export function run(): void;
/**
* @param {any} player
* @param {any} board
* @param {any} depth
* @returns {Int32Array}
*/
export function alphabeta_js(player: any, board: any, depth: any): Int32Array;
/**
* @param {any} player
* @param {any} board
* @param {any} itr
* @returns {Int32Array}
*/
export function mcts_js(player: any, board: any, itr: any): Int32Array;
/**
* @param {any} player
* @param {any} board
* @param {any} depth
* @returns {Int32Array}
*/
export function negascout_js(player: any, board: any, depth: any): Int32Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run: () => void;
  readonly alphabeta_js: (a: number, b: number, c: number, d: number) => void;
  readonly mcts_js: (a: number, b: number, c: number, d: number) => void;
  readonly negascout_js: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        