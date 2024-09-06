const fs = require('node:fs') // import the fs module
const readline = require('readline') // import the readline module

// Create an I/O interface for readline to listen on
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
})

// use fs to read the wasm bytecode into a buffer
const wasmBuffer = fs.readFileSync(`../build/stats_calc.wasm`)

// use WebAssembly.instantiate, passing in the buffer
WebAssembly.instantiate(wasmBuffer).then(
    (wasm) => { // when wasm is instantiated...
        // prompt user for number input
        console.log("Enter a stat amount (a number):")
        // create an event listener for user input
        rl.on(`line`, (line) => {
            // check the line in is a number
            if(isNaN(line)){ // if not a number
                console.log("Please enter a number")
            } else { // it's a number
                // call wasm function and print result
                console.log(`Result: ${wasm.instance.exports.calcStatMod(parseInt(line))}`)
            }
        })
    }
)