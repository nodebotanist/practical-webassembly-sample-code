function createWasmWorker() {
    return new Promise((resolve, reject) => {
        const wasmWorker = new Worker('worker.js', {type: 'module'})

        wasmWorker.addEventListener('message', (event) => {
            let {eventType, eventData} = event.data

            switch (eventType) {
                case "INITIALIZED":
                    resolve(wasmWorker)
                    break
                case "RESULT":
                    console.log(eventData)
                    resolve(`${eventData}`)
                    break
                case "ERROR":
                    reject(`Error running wasm worker: ${eventData}`)
                    break
            }
        }, false)

        wasmWorker.addEventListener('error', (error) => {
            reject(error)
        })
        
        wasmWorker.postMessage({
            eventType: "INIT"
        })
    })
}

createWasmWorker()
    .then((worker) => {
        console.log("Wasm worker initialized from index.js")
        worker.postMessage({
            eventType: "ROLL",
            eventData: "3d5+4"
        })
    })
    .catch((error) => console.log(error))