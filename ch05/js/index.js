import {roll} from '../roll_dice/release.js'
import init from '../pkg/ch05.js'
import {greet} from '../pkg/ch05.js'

(async () => {
    await init()
    greet()
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
            let result = roll(rollDiceInput.value)
            resultSpan.innerHTML = result
        }
    })
})()