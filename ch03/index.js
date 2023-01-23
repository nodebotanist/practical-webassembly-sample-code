const fs = require('fs')

const wasmBuffer = fs.readFileSync('example.wasm')

const importObject = {
    console: {
        log (arg) {
            console.log(arg)
        }
    }
}

WebAssembly.instantiate(wasmBuffer, importObject).then((wasmModule) => {
    const {add, addAndLog} = wasmModule.instance.exports
    const sum1 = add(14, 39)
    console.log(sum1)
    addAndLog(34, 54)
})