/** Exported memory */
export declare const memory: WebAssembly.Memory;
/**
 * assembly/index/parse_roll_string
 * @param input `~lib/string/String`
 * @returns `~lib/array/Array<i32>`
 */
export declare function parse_roll_string(input: string): Array<number>;
/**
 * assembly/index/roll_die
 * @param dieMax `i32`
 * @returns `i32`
 */
export declare function roll_die(dieMax: number): number;
