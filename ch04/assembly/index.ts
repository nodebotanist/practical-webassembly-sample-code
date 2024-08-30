// The entry file of your WebAssembly module.

export function parse_roll_string(input: String): Array<i32> {
  // split '##d##+##' into ['##', '##+##']
  let splitForNumOfDice: Array<String> = input.split('d', 10)
  // split '##+##' into ['##', '##'] OR '##-##' into ["##", '-##']
  let delimiter = '+'
  if(input.indexOf('+') === -1) {
    delimiter = '-'
  }
  console.log("Delimiter " + delimiter)
  let splitForModifier: Array<String> = splitForNumOfDice[1].split(delimiter)
  // parse '##' for # of dice to an i32
  let numberOfDice: i32 = i32.parse(splitForNumOfDice[0] as string, 10)
  // parse '##' for max value of dice to an i32
  let diceMaxValue: i32 = i32.parse(splitForModifier[0] as string, 10)
  
  // because the '##' for the modifier is optional, check that it exists before parsing
  let modifier: i32 = 0
  if(splitForModifier.length == 2) {
    // parse '##' for modifier into an i32
    modifier = i32.parse(splitForModifier[1] as string, 10)
    if(delimiter === '-') {
      modifier = -modifier
    }
  }

  // return the parsed i32 values
  return [numberOfDice, diceMaxValue, modifier]
}

export function roll_die(dieMax: i32): i32 {
  // Step 1: get a random f64 between 0 and 1
  // Step 2: multiply it by dieMax, which is cast to a f64 as well
  // Step 3: get the ceiling integer value of the multiplied f64 values and cast it to i32
  // Step 4: return the cast i32 from step 3
  return Math.ceil((Math.random() * dieMax as f64)) as i32
}