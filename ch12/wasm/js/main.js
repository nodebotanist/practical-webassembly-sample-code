// get our HTML handles
// the stat inputs
let statStr = document.querySelector('[name=stat_str]')
let statDex = document.querySelector('[name=stat_dex]')
let statCon = document.querySelector('[name=stat_con]')
let statInt = document.querySelector('[name=stat_int]')
let statWis = document.querySelector('[name=stat_wis]')
let statCha = document.querySelector('[name=stat_cha]')
// the bonus modifier spans
let statStrBonus = document.querySelector('#stat_str_bonus')
let statDexBonus = document.querySelector('#stat_dex_bonus')
let statConBonus = document.querySelector('#stat_con_bonus')
let statIntBonus = document.querySelector('#stat_int_bonus')
let statWisBonus = document.querySelector('#stat_wis_bonus')
let statChaBonus = document.querySelector('#stat_cha_bonus')
// dice roll elements
const rollDiceButton = document.querySelector('#roll_dice')
const rollDiceInput = document.querySelector('input[name=dice_text]')
const errorDiv = document.querySelector('#error_message')
const resultSpan = document.querySelector('#dice_result')

function createWasmWorker() {
    return new Promise((resolve, reject) => {
        const wasmWorker = new Worker('js/worker.js', {
            type: 'module'
        })

        wasmWorker.addEventListener('message', (event) => {
            let {eventType, eventData} = event.data
            switch (eventType) {
                case 'INITIALIZED':
                    resolve(wasmWorker)
                    break
                case 'RESULT':
                    console.log(`Roll Result -- main:`, eventData)
                    resultSpan.innerHTML= `${eventData}`
                    resolve(eventData)
                    break
                case 'ERROR':
                    reject(`Error running WebAssembly worker: ${eventData}`)
                    break
                default:
                    reject(`Undefined event type ${eventType} in Main thread.`)
            }
        })

        wasmWorker.addEventListener('error', (err) => reject(err))

        setTimeout(() => {
            wasmWorker.postMessage({
                eventType: 'INIT'
            })
        }, 500)

    })
}

createWasmWorker()
    .then((worker) => {        
        // placeholder for wasm function
        let calcStatBonus;
        // fetch and instantiate the wasm
        WebAssembly.instantiateStreaming(fetch(`../lib/stats_calc.wasm`)).then(
            // this callback is run when the wasm instance is ready
            (wasm) => {
                // assigns the wasm function to a JS variable declared in the outmost scope
                calcStatBonus = wasm.instance.exports.calculate_stat_bonus
                calcStatMods()
            }
        )



        function calcStatMods() {
            // calculate the bonus mods with wasm if ready
            if(calcStatBonus) {
                statStrBonus.innerHTML = calcStatBonus(statStr.value).toString()
                statDexBonus.innerHTML = calcStatBonus(statDex.value).toString()
                statConBonus.innerHTML = calcStatBonus(statCon.value).toString()
                statIntBonus.innerHTML = calcStatBonus(statInt.value).toString()
                statWisBonus.innerHTML = calcStatBonus(statWis.value).toString()
                statChaBonus.innerHTML = calcStatBonus(statCha.value).toString()
            }
        }

        // set a listener to each input to calculate stat bonuses on change
        statStr.addEventListener('change', calcStatMods)
        statDex.addEventListener('change', calcStatMods)
        statCon.addEventListener('change', calcStatMods)
        statInt.addEventListener('change', calcStatMods)
        statWis.addEventListener('change', calcStatMods)
        statCha.addEventListener('change', calcStatMods)

        let rollDice = () => {
            worker.postMessage({
                eventType: 'ROLL',
                eventData: rollDiceInput.value
            })
        }

        rollDiceButton.addEventListener('click', rollDice)
    })
    .catch((err) => console.error(err))