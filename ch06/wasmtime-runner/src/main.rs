extern crate roll_checker;

#[no_mangle]
pub fn main() {
    roll_dice_log()
}

#[no_mangle]
pub extern "C" fn roll_dice_log() {
    let result = roll_checker::roll_die(25);
    match result {
        Err(_) => { println!("Error"); }
        Ok(answer) => { println!("Result of 7d6+5: {:?}", answer); }
    }
}
