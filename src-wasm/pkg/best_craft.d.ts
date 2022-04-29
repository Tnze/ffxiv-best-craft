/* tslint:disable */
/* eslint-disable */
/**
* @param {number} rlv
* @param {number} difficulty_factor
* @param {number} quality_factor
* @param {number} durability_factor
* @returns {any}
*/
export function new_recipe(rlv: number, difficulty_factor: number, quality_factor: number, durability_factor: number): any;
/**
* @param {any} attrs
* @param {any} recipe
* @param {number} init_quality
* @returns {any}
*/
export function new_status(attrs: any, recipe: any, init_quality: number): any;
/**
* @param {any} status
* @param {any} skills
* @returns {any}
*/
export function simulate(status: any, skills: any): any;
/**
* @param {any} status
* @param {any} skills
* @returns {any}
*/
export function allowed_list(status: any, skills: any): any;
/**
* @param {any} status
* @param {any} skills
* @returns {any}
*/
export function craftpoints_list(status: any, skills: any): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly new_recipe: (a: number, b: number, c: number, d: number) => number;
  readonly new_status: (a: number, b: number, c: number, d: number) => void;
  readonly simulate: (a: number, b: number) => number;
  readonly allowed_list: (a: number, b: number) => number;
  readonly craftpoints_list: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
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
