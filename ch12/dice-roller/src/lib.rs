extern crate regex;

use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use rand::{thread_rng, Rng};
use regex::Regex;


/// A simple Spin HTTP component.
#[http_component]
fn handle_dice_roller(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let roll_string = req.query();
    let roll_regex = Regex::new(r"([0-9]+)[d]([0-9]+)\+?([0-9]+)?").unwrap();
    let captures = roll_regex.captures(roll_string);
    match captures {
        Some(ref roll_match) => {
            if roll_match.get(3) == None || roll_match.get(2) == None || roll_match.get(1) == None {
                return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain")
                .body("Invalid roll")
                .build());
            }
            let number_of_dice: i32 = roll_match.get(1).unwrap().as_str().parse().unwrap();
            let dice_max: i32 = roll_match.get(2).unwrap().as_str().parse().unwrap();
            let modifier: i32 = roll_match.get(3).unwrap().as_str().parse().unwrap();
            println!("Dice roll numbers: {:?}, {:?}, {:?}", number_of_dice, dice_max, modifier);
            // create dice roll
            let mut rng = thread_rng();
            let mut dice_roll_total = modifier;
            let mut rolls: Vec<i32> = Vec::new();
            for _ in 0..number_of_dice {
                let this_dice_roll = rng.gen_range(1..dice_max + 1);
                rolls.push(this_dice_roll);
                dice_roll_total += this_dice_roll;
            }
            println!("Dice roll total: {:?}, Rolls: {:?}", dice_roll_total, rolls);

            return Ok(Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(format!("Dice roll total: {:?}, Rolls: {:?}", dice_roll_total, rolls))
            .build());
        }
        None => {
            return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body("Invalid roll")
            .build());
        }
    }
}
