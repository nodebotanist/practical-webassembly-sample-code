import init, { roll_dice, validate_roll_string } from '../pkg/roll_checker.js'
await init()

const rollInput = Deno.args[0]

if( !validate_roll_string(rollInput) ) {
    console.log('Invalid roll- needs to be ##d##(+##)')
} else {
    const rollResult = roll_dice(rollInput)
    console.log(rollResult)
}