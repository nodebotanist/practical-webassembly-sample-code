import {roll} from '../roll_dice/release.js'
import initRustWasm from '../pkg/ch05.js'
import {greet, attack_roll} from '../pkg/ch05.js'

(async () => {
    await initRustWasm()

    const rollDiceButton = document.querySelector('#roll_dice')
    const rollDiceInput = document.querySelector('input[name=dice_text]')
    const errorDiv = document.querySelector('#error_message')
    const resultSpan = document.querySelector('#dice_result')

    rollDiceButton.addEventListener('click', () => {
        const verificationRegExp = /^[0-9]+[d][0-9]+(\+[0-9]+)?$/
        if( rollDiceInput.value.match(verificationRegExp) == null ) {
            errorDiv.innerHTML = `<p>Invalid input: must be in the format [number]d[number], with an optional +[number]</p>`
        } else {
            errorDiv.innerHTML = ``
            const result = roll(rollDiceInput.value)
            resultSpan.innerHTML = result
        }
    })

    const generateAttackButton = document.querySelector('#create_attack_log')
    const targetNameInput = document.querySelector('input[name=target_name]')
    const diceRollInput = document.querySelector('input[name=dice_roll_text]')
    const targetHealthInput = document.querySelector('input[name=health_amount]')
    const attackLogErrorSpan = document.querySelector('#attack_log_error')
    const attackResult = document.querySelector('#attack_log_result')

    generateAttackButton.addEventListener('click', () => {
        const healOrHurt = document.querySelector('input[name=heal_or_hurt]:checked')
        const verificationRegExp = /^[0-9]+[d][0-9]+(\+[0-9]+)?$/
        if( diceRollInput.value.match(verificationRegExp) == null ) {
            attackLogErrorSpan.innerHTML = `<p>Invalid input: must be in the format [number]d[number], with an optional +[number]</p>`
        } else {
            attackLogErrorSpan.innerHTML = ``
            const rollResult = roll(diceRollInput.value)
            console.log(healOrHurt.value)
            const hurt = healOrHurt.value === 'heal' ? false : true
            const health = targetHealthInput.value
            const name = targetNameInput.value

            const result = attack_roll(name, rollResult, health, hurt)
            attackResult.innerHTML = result
        }      
    })

    console.log(attack_roll("Hello", 39, 36, true))
})()