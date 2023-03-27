
use rand::Rng;

#[derive(Debug)]
pub struct RollResult {
    total: i32,
    dice_results: Vec<i32>
}

pub fn roll_die(die_max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..(die_max +1))
}

pub fn roll_dice(number_of_dice: i32, die_max: i32, modifier: i32) -> RollResult {
    let mut dice_result: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    for _n in 0..number_of_dice {
        let roll = roll_die(die_max);
        dice_result.push(roll);
        total += roll;
    }
    return RollResult {
        dice_results: dice_result,
        total: total + modifier
    };
}

pub fn parse_roll(roll_string: &str) -> [i32;3] {
    let roll_split = roll_string.split(['d', '+']);
    let mut result: [i32;3] = [0, 0, 0];
    let mut i: usize = 0;
    for dice in roll_split {
        result[i] = dice.parse::<i32>().unwrap();
        i+=1;
    }
    return result;
}

pub fn main() {
    let dice_roll = "5d4+4";
    let dice_to_roll = parse_roll(dice_roll);
    let result = roll_dice(dice_to_roll[0], dice_to_roll[1], dice_to_roll[2]);
    println!("{:?}", result);
}