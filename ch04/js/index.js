import {parse_roll_string, roll_die} from '../build/release.js'

(() => {
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
            let rollNumbers = parse_roll_string(rollDiceInput.value)
            let result = roll_die(rollNumbers[1])
            resultSpan.innerHTML = result
        }
    })
})()