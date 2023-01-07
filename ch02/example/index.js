// bring in the compiled bytecode with a JS wrapper created for us by wasm-pack
import init from './pkg/ch02.js'

const runWasm = async () => {
    // create a helloWorld object out of instantiated wasm code
    const helloWorld = await init('./pkg/ch02_bg.wasm')
    // call our hello_world function (console.log)
    helloWorld.hello_world()
}
runWasm()