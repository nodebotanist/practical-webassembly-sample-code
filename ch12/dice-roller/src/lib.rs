extern crate regex;

use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use rand::{thread_rng, Rng};
use regex::Regex;

#[http_component]
fn handle_dice_roller(req: Request) -> anyhow::Result<impl IntoResponse> {
    // these statements print to the console when running locally
    println!("Handling request to {:?}", req.header("spin-full-url"));
    // get the query string from the request
    let roll_string = req.query();
    // create a regex to capture numbers and validate the roll string
    let roll_regex = Regex::new(r"roll=([0-9]+)[d]([0-9]+)\+?([0-9]+)?").unwrap();
    // parse the roll numbers out of the query string -- if there are missing matches, you'll know the roll is invalid
    let captures = roll_regex.captures(roll_string);
    // match against the captured values
    match captures {
        Some(ref roll_match) => { // this means there was at least a partial match
            // more verification; if roll numbers 1 or 2 are not there, it's not a valid roll (modifier is optional)
            if roll_match.get(2) == None || roll_match.get(1) == None {
                // return a 400 because it's an invalid roll string
                return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain")
                .body("Invalid roll")
                .build());
            }
            // use the capture groups to pull the numbers we need out, then turn them into numbers
            let number_of_dice: i32 = roll_match.get(1).unwrap().as_str().parse().unwrap();
            let dice_max: i32 = roll_match.get(2).unwrap().as_str().parse().unwrap();
            // if the modifier isn't there, set it to 0
            let modifier: i32 = if roll_match.get(3) == None { 0 } else { roll_match.get(3).unwrap().as_str().parse().unwrap() };
            // for debugging purposes
            println!("Dice roll numbers: {:?}, {:?}, {:?}", number_of_dice, dice_max, modifier);
            // create an instance of a random number generator for us to use
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
            println!("Dice roll total: {:?}, Rolls: {:?}", dice_roll_total, rolls);

            // return an HTTP response with the total and the rolls
            return Ok(Response::builder()
            .status(200)
            .header("content-type", "text/json")
            .body(format!("[{:?}, {:?}]", dice_roll_total, rolls))
            .build());
        }
        None => {
            // return a 400 invalid b/c there was no roll found
            return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body("Invalid roll")
            .build());
        }
    }
}
