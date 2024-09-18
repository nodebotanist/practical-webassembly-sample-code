import init, {roll_dice} from '../lib/ch07/roll_checker.js'
import * as wasm from '../lib/ch07/roll_checker_bg.js'

// create promise for wasm ready state
let wasmReadyResolve, wasmReadyReject
let wasmReady = new Promise((resolve, reject) => {
    wasmReadyResolve = resolve
    wasmReadyReject = reject
})

self.addEventListener('error', (err) => wasmReadyReject(err))

self.addEventListener('message', async (event) => {
    const { eventType, eventData } = event.data
    console.log(`message type: ${eventType} data: ${eventData}`)
    switch (eventType) {
        case "INIT":
            init().then((wasm) => {
                self.postMessage({
                    eventType: 'INITIALIZED'
                })
                wasmReadyResolve(self)
            })
            break
        case "ROLL":
            const rollResult = await roll_dice(eventData)

            // get total from the RollResult
            let total = rollResult.total

            self.postMessage({
                eventType: 'RESULT',
                eventData: total
            })
            break
        default:
            throw new Error(`Undefined event type ${eventType} in WebAssembly worker.`)
    }
}, false) // the false prevents event bubbling

wasmReadyResolve()

