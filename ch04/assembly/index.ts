// The entry file of your WebAssembly module.

export function roll(input: String): i32 {
  let numbersToRoll: Array<i32> = readDiceInput(input)
  return rollDice(numbersToRoll)
}

function readDiceInput(input: String): Array<i32> {
  let splitForDie: Array<String> = input.split('d', 10)
  let splitForExtra: Array<String> = splitForDie[1].split('+')
  let numberOfDice: i32 = i32.parse(splitForDie[0] as string, 10)
  let numberOnDie: i32 = i32.parse(splitForExtra[0] as string, 10)
  let extraAdd: i32 = 0

  if(splitForExtra.length == 2) {
    extraAdd = i32.parse(splitForExtra[1] as string, 10)
  }
  return [numberOfDice, numberOnDie, extraAdd]
}

function rollDice(diceToRoll: Array<i32>): i32 {
  let numberOfDice: i32 = diceToRoll[0]
  let numberOnDie: i32 = diceToRoll[1]
  let extraAdd: i32 = diceToRoll[2]
  let total:i32 = 0
  for(let i=0; i<numberOfDice; i++) {
    total += rollDie(numberOnDie)
  }
  return total + extraAdd
}

function rollDie(dieMax: i32): i32 {
  return Math.ceil((Math.random() * dieMax as f64)) as i32
}