import {parse_roll_string, roll_die} from '../build/release.js'

// placeholder for wasm function
let calcStatBonus;
// fetch and instantiate the wasm
WebAssembly.instantiateStreaming(fetch(`lib/stat_bonus_calculator.wasm`)).then(
    // this callback is run when the wasm instance is ready
    (wasm) => {
        // assigns the wasm function to a JS variable declared in the outmost scope
        calcStatBonus = wasm.instance.exports.calculate_stat_bonus
        calcStatBonuses()
    }
)

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

const rollDiceButton = document.querySelector('#roll_dice')
const rollDiceInput = document.querySelector('input[name=dice_text]')
const errorDiv = document.querySelector('#error_message')
const resultSpan = document.querySelector('#dice_result')

function calcStatBonuses() {
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

function rollDice() {
    const verificationRegExp = /^[0-9]+[d][0-9]+(\+[0-9]+)?$/
    if( rollDiceInput.value.match(verificationRegExp) == null ) {
        errorDiv.innerHTML = `<p>Invalid input: must be in the format [number]d[number], with an optional +[number]</p>`
    } else {
        errorDiv.innerHTML = ``
        let rollNumbers = parse_roll_string(rollDiceInput.value)
        let result = roll_die(rollNumbers[1])
        resultSpan.innerHTML = result
    }
}

// set a listener to each input to calculate stat bonuses on change
statStr.addEventListener('change', calcStatBonuses)
statDex.addEventListener('change', calcStatBonuses)
statCon.addEventListener('change', calcStatBonuses)
statInt.addEventListener('change', calcStatBonuses)
statWis.addEventListener('change', calcStatBonuses)
statCha.addEventListener('change', calcStatBonuses)

rollDiceButton.addEventListener('click', rollDice)