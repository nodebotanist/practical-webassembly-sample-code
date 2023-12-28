// create promise for wasm ready state
let wasmReadyResolve
let wasmReady = new Promise((resolve) => {
    wasmReadyResolve = resolve
}) 

// incoming message handler
self.addEventListener('message', (event) => {
    const { eventType, eventData } = event.data

    switch (eventType) {
        case "INIT":
            console.log("Initialize wasm")
            WebAssembly.instantiateStreaming(fetch(eventData), {})
            .then(instantiatedModule => {
                const wasmExports = instantiatedModule.instance.exports;
 
                // Resolve our exports for when the messages
                // to execute functions come through
                wasmReadyResolve(wasmExports);
 
                // Send back initialised message to main thread
                self.postMessage({
                    eventType: "INITIALIZED",
                    eventData: Object.keys(wasmExports)
                });
     
            });
            break
        case "ROLL":
            console.log("Roll dice")
            break
        default:
            throw new Error(`Undefined event type ${eventType} in WebAssembly worker.`)
    }
}, false) // this false prevents bubbled events from triggering



