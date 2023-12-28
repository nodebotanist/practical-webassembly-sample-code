function createWasmWorker (modulePath) {
    return new Promise((resolve, reject) => {
        const wasmWorker = new Worker('worker.js')
        wasmWorker.postMessage({
            eventType: "INIT",
            eventData: modulePath
        })

        wasmWorker.addEventListener('message', (event) => {
            let {eventType, eventData} = event.data

            switch (eventType) {
                case "INITIALIZED":
                    console.log("Wasm worker initialized")
                    resolve()
                    break
                case "RESULT":
                    console.log(`Roll result: ${eventData}`)
                    break
                case "ERROR":
                    console.log(`Error running wasm worker: ${eventData}`)
                    break
            }
        }, false)

        wasmWorker.addEventListener('error', (error) => {
            reject(error)
        })
    })
}

createWasmWorker("../target/wasm32-unknown-unknown/release/ch10.wasm")
    .then((value) => {
        console.log("Wasm worker initialized from index.js")
    })