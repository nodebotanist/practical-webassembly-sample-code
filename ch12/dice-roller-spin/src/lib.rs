use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http::Response;
use spin_sdk::http_component;

extern crate regex;
use regex::Regex;

use roll_checker::roll_dice_from_numbers;

/// A simple Spin HTTP component.
#[http_component]
fn handle_dice_roller_spin(req: Request) -> anyhow::Result<impl IntoResponse> {
    // these statements print to the console when running locally
    println!("Handling request to {:?}", req.header("spin-full-url"));
    // get the query string from the request
    let roll_string = req.query();
    // create a regex to capture numbers and validate the roll string
    let roll_regex = Regex::new(r"roll=([0-9]+)[d]([0-9]+)\+?([0-9]+)?").unwrap();
    // parse the roll numbers out of the query string -- if there are missing matches, you'll know the roll is invalid
    let captures = roll_regex.captures(roll_string);
    match captures {
        Some(ref roll_match) => {
            if roll_match.get(2) == None || roll_match.get(1) == None {
                return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain")
                .body(format!("Invalid roll {}", roll_string).to_owned())
                .build());
            }
            
            // // use the capture groups to pull the numbers we need out, then turn them into numbers
            let number_of_dice: i32 = roll_match.get(1).unwrap().as_str().parse().unwrap();
            let die_max:i32 = roll_match.get(2).unwrap().as_str().parse().unwrap();
            let modifier:i32 = if roll_match.get(3) == None { 0 } else { roll_match.get(3).unwrap().as_str().parse().unwrap() };
            println!("Dice roll numbers: {:?}, {:?}, {:?}", number_of_dice, die_max, modifier);

            let roll_result = roll_checker::roll_dice_from_numbers(number_of_dice, die_max, modifier);
            println!("Roll result {:?}", roll_result);

            // // return an HTTP response with the total and the rolls
            Ok(Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(format!("{:?}", roll_result).to_owned())
            .build())
            // .body(format!("Dice roll total: {:?}, Rolls: {:?}", dice_roll_total, rolls).into())?);
        }
        None => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain")
                .body(format!("Invalid roll {}", roll_string).to_owned())
                .build());
        }
    }
}
