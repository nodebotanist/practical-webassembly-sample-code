use anyhow::Result;
use wasm_workers_rs::{
    worker,
    http::{self, Request, Response},
    Content,
};
extern crate regex;

use rand::{thread_rng, Rng};
use regex::Regex;

#[worker]
fn reply(req: Request<String>) -> Result<Response<Content>> {
    let roll_string = req.uri();
    let roll_regex = Regex::new(r"roll=([0-9]+)[d]([0-9]+)\+?([0-9]+)?").unwrap();
    // parse the roll out of the query string
    let captures = if roll_string.query() == None { None } else { roll_regex.captures(roll_string.query().unwrap()) };
    match captures {
        Some(ref roll_match) => {
            if roll_match.get(2) == None || roll_match.get(1) == None {
                return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain")
                .body(format!("Invalid roll").into())?);
            }
            // use the capture groups to pull the numbers we need out, then turn them into numbers
            let number_of_dice: i32 = roll_match.get(1).unwrap().as_str().parse().unwrap();
            let dice_max: i32 = roll_match.get(2).unwrap().as_str().parse().unwrap();
            let modifier: i32 = if roll_match.get(3) == None { 0 } else { roll_match.get(3).unwrap().as_str().parse().unwrap() };
            // println!("Dice roll numbers: {:?}, {:?}, {:?}", number_of_dice, dice_max, modifier);
            // create dice roll using the numbers we parsed out
            let mut rng = thread_rng();
            // start the sum for the dice roll total with the modifier
            let mut dice_roll_total = modifier;
            // create a Vec<i32> to hold each roll so we can pass it with the total
            let mut rolls: Vec<i32> = Vec::new();
            // roll the dice!
            for _ in 0..number_of_dice {
                let this_dice_roll = rng.gen_range(1..dice_max + 1);
                // add the die result to the collection
                rolls.push(this_dice_roll);
                // add the die result to the total
                dice_roll_total += this_dice_roll;
            }
            // println!("Dice roll total: {:?}, Rolls: {:?}", dice_roll_total, rolls);

            // return an HTTP response with the total and the rolls
            return Ok(http::Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(format!("Dice roll total: {:?}, Rolls: {:?}", dice_roll_total, rolls).into())?);
        }
        None => {
            return Ok(http::Response::builder()
                .status(400)
                .header("content-type", "text/plain")
                .body(format!("Invalid roll").into())?);
        }
    }
}