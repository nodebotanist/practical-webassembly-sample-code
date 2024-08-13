WebAssembly.instantiateStreaming(fetch(`build/stat_bonus_calculator.wasm`)).then(
    (wasm) => {
        let result = wasm.instance.exports.calculate_stat_bonus(15)
        console.log(result)
    }
)