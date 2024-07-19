extern crate web_sys;
use web_sys::{console, HtmlInputElement, MouseEvent};
use yew::{
    Html,
    html,
    Event,
    Callback, TargetCast,
    function_component,
    Properties
};
use crate::web_sys::wasm_bindgen::JsCast;
extern crate regex;
use rand::{thread_rng, Rng};

fn roll_dice(number_of_dice: i32, dice_max_value: i32, dice_modifier: i32) -> (i32, Vec<i32>) {
    // create an instance of a random number generator for us to use
    let mut rng = thread_rng();
    // start the sum for the dice roll total with the modifier
    let mut dice_roll_total = dice_modifier;
    // create a Vec<i32> to hold each roll so we can pass it with the total
    let mut rolls: Vec<i32> = Vec::new();
    // roll the dice!
    for _ in 0..number_of_dice {
        let this_dice_roll = rng.gen_range(1..dice_max_value + 1);
        // add the die result to the collection
        rolls.push(this_dice_roll);
        // add the die result to the total
        dice_roll_total += this_dice_roll;
    }
    console::log_1(&format!("Dice roll total: {:?}, Rolls: {:?}", dice_roll_total, rolls).into());

    (dice_roll_total, rolls)
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub default_num_dice: i32,
    pub default_dice_max_value: i32,
    pub default_modifier: i32
}

#[function_component(DiceRollForm)]
pub fn dice_roll_form(props: &Props) -> Html {
    let onchange_num_of_dice: Callback<Event> = Callback::from(move |e:Event| {
        let input_element: HtmlInputElement = e.target_unchecked_into();
        let input_value = input_element.value();
        console::log_1(&format!("Changed number of dice: {:?}", input_value).into());
    });

    let onchange_dice_max_value: Callback<Event> = Callback::from(move |e:Event| {
        let input_element: HtmlInputElement = e.target_unchecked_into();
        let input_value = input_element.value();
        console::log_1(&format!("Changed dice max value: {:?}", input_value).into());
    });

    let onchange_modifier: Callback<Event> = Callback::from(move |e:Event| {
        let input_element: HtmlInputElement = e.target_unchecked_into();
        let input_value = input_element.value();
        console::log_1(&format!("Changed dice modifier: {:?}", input_value).into());
    });

    let on_dice_roll = move |_:MouseEvent| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let number_of_dice :i32 = document
            .get_element_by_id("num_of_dice")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse()
            .unwrap();
        let dice_max_value: i32 = document
            .get_element_by_id("dice_max_value")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse()
            .unwrap();
        let dice_modifier: i32 = document
            .get_element_by_id("dice_modifier")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse()
            .unwrap();
        console::log_1(&format!("Roll {:?}d{:?}+{:?}", number_of_dice, dice_max_value, dice_modifier).into());

        let (dice_roll_total, rolls) = roll_dice(number_of_dice, dice_max_value, dice_modifier);

        let results = document
            .get_element_by_id("dice_roll_results")
            .unwrap();

        let val = document.create_element("p").unwrap();
        val.set_text_content(Some(&format!("Dice roll total: {:?}, Rolls: {:?}", dice_roll_total, rolls)));
    
        results.append_child(&val).unwrap();
    };

    html! {    
        <>         
            <label for="number_of_dice">{"Number of Dice:"}</label>
            <input type="number" id="num_of_dice" name="number_of_dice" placeholder=1 value={props.default_num_dice.to_string().clone()} min="0" max="100" onchange={ onchange_num_of_dice} /><br />
            <label for="number_of_dice">{"Max value of Dice:"}</label>
            <input type="number" id="dice_max_value" name="dice_max_value" placeholder=1 value={props.default_dice_max_value.to_string().clone()} min="1" max="100" onchange={onchange_dice_max_value} /><br />
            <label for="number_of_dice">{"Dice Roll Modifier:"}</label>
            <input type="number" id="dice_modifier" name="dice_modifier" placeholder=0 value={props.default_modifier.to_string().clone()} min="-100" max="100" onchange={onchange_modifier} /><br />
            <button id="roll_dice_betton" onclick={on_dice_roll} type="submit" >{"Roll Dice"}</button>
            <div id ="dice_roll_results"></div> 
        </>
    }
}