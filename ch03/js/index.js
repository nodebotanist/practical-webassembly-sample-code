// placeholder for wasm function
let calcStatMod;

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

// fetch and instantiate the wasm
WebAssembly.instantiateStreaming(await fetch(`build/stats_calc.wasm`)).then(
    // this callback is run when the wasm instance is ready
    (wasm) => {
        console.log(wasm)
        // assigns the wasm function to a JS variable declared in the outmost scope
        calcStatMod = wasm.instance.exports.calcStatMod
        calcStatMods()
    }
)

function calcStatMods() {
    // calculate the bonus mods with wasm if ready
    if(calcStatMod) {
        statStrBonus.innerHTML = calcStatMod(statStr.value).toString()
        statDexBonus.innerHTML = calcStatMod(statDex.value).toString()
        statConBonus.innerHTML = calcStatMod(statCon.value).toString()
        statIntBonus.innerHTML = calcStatMod(statInt.value).toString()
        statWisBonus.innerHTML = calcStatMod(statWis.value).toString()
        statChaBonus.innerHTML = calcStatMod(statCha.value).toString()
    }
}

// set a listener to each input to calculate stat bonuses on change
statStr.addEventListener('change', calcStatMods)
statDex.addEventListener('change', calcStatMods)
statCon.addEventListener('change', calcStatMods)
statInt.addEventListener('change', calcStatMods)
statWis.addEventListener('change', calcStatMods)
statCha.addEventListener('change', calcStatMods)