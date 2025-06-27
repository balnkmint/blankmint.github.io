/* tslint:disable */
/* eslint-disable */
export class Game {
  free(): void;
  constructor();
  tick(): void;
  set_direction_from_key(key: string): void;
  toggle_pause(): void;
  is_paused(): boolean;
  render(ctx: CanvasRenderingContext2D): void;
  get_score(): number;
  is_game_over(): boolean;
  restart(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_game_free: (a: number, b: number) => void;
  readonly game_new: () => number;
  readonly game_tick: (a: number) => void;
  readonly game_set_direction_from_key: (a: number, b: number, c: number) => void;
  readonly game_toggle_pause: (a: number) => void;
  readonly game_is_paused: (a: number) => number;
  readonly game_render: (a: number, b: any) => void;
  readonly game_get_score: (a: number) => number;
  readonly game_is_game_over: (a: number) => number;
  readonly game_restart: (a: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
