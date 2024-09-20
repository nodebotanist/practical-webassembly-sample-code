use anyhow::Result;
use wasm_workers_rs::{
    worker,
    http::{self, Request, Response},
    Content,
};
extern crate regex;

use regex::Regex;

use roll_checker;

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
            // create dice roll using the numbers we parsed out
            let roll_result = roll_checker::roll_dice_from_numbers(number_of_dice, dice_max, modifier);
            // get the total and the roll results
            let dice_roll_total = roll_result.total;
            let rolls = roll_result.get_dice_rolls();

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
