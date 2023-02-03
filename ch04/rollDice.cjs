const fs = require('fs')

const args = process.argv.slice(2);

const wasm = import('./build/release.js').then((instance) => {
    const rollInput = args[0]
    const verificationRegExp = /^[0-9]+[d][0-9]+(\+[0-9]+)?$/
    if( rollInput.match(verificationRegExp) == null ) {
        console.log('Invalid roll- needs to be #d#(+#)')
    } else {
        const result = instance.roll(rollInput)
        console.log(result)
    }
})
