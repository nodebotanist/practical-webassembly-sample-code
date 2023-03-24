import * as wasm from "../pkg/ch07";

console.log(JSON.parse(wasm.roll_dice(4, 4, 4)))
console.log(wasm.print_result_to_dom("4d4+4"))