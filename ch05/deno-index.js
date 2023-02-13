import { roll } from './roll_dice/release.js'
import { attackroll } from './pkg/ch05.js'

console.log(roll('5d6+40'))

const name = Deno.args[0]
const rollInput = Deno.args[1]
const totalHealth = Deno.args[2]
const hurt = !!Deno.args[3]

const verificationRegExp = /^[0-9]+[d][0-9]+(\+[0-9]+)?$/

if( rollInput.match(verificationRegExp) == null ) {
    console.log('Invalid roll- needs to be #d#(+#)')
} else {
    const rollResult = roll(rollInput)
    const result = attackroll(name, rollResult, totalHealth, hurt)
    console.log(result)
}
