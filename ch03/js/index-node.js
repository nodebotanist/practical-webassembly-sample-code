const fs = require(`node:fs`)
const readline = require(`readline`)

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
})

const wasmBuffer = fs.readFileSync(`../build/stat_bonus_calculator.wasm`)

WebAssembly.instantiate(wasmBuffer).then(
    (wasm) => {
        console.log(`Enter a stat amount`)
        rl.on(`line`, (line) => {
            if(isNaN(line)){
                console.log(`Please enter a number (press CTRL-C to exit):`)
            } else {
                console.log(`Result: ${wasm.instance.exports.calculate_stat_bonus(parseInt(line))}`)
            }
        })
    }
)