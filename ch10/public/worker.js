import initRustWasm, { roll_dice } from '../pkg/ch10.js'

// create promise for wasm ready state
let wasmReadyResolve, wasmReadyReject
let wasmReady = new Promise((resolve, reject) => {
    wasmReadyResolve = resolve
    wasmReadyReject = reject
}) 

self.addEventListener('error', (error) => wasmReadyReject(error))

// incoming message handler
self.addEventListener('message', async (event) => {
    const { eventType, eventData } = event.data
    switch (eventType) {
        case "INIT":
            await initRustWasm()
            wasmReadyResolve()

            // Send back initialised message to main thread
            self.postMessage({
                eventType: "INITIALIZED",
            })
            break
        case "ROLL":
            const rollResult = await roll_dice('2d4')
            self.postMessage({
                eventType: "RESULT",
                eventData: rollResult
            })
            break
        default:
            throw new Error(`Undefined event type ${eventType} in WebAssembly worker.`)
    }
}, false) // this false prevents bubbled events from triggering



