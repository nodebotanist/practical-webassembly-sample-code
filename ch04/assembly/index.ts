// The entry file of your WebAssembly module.

export function add(a: i32, b: i32): i32 {
  return a + b;
}

export function readDiceInput(input: String): Array<i32> {
  let splitForDie = input.split('d', 10)
  let splitForExtra = splitForDie[1].split('+')
  let numberOfDice = i32.parse(splitForDie[0], 10)
  let numberOnDie = i32.parse(splitForExtra[0], 10)
  let extraAdd = i32.parse(splitForExtra[1], 10)
  return [numberOfDice, numberOnDie, extraAdd]
}