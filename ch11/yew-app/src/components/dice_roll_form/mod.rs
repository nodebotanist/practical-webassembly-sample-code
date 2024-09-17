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
use wasm_bindgen::{JsCast, JsError};

mod roll_dice;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub default_num_dice: i32,
    pub default_dice_max_value: i32,
    pub default_modifier: i32
}

#[function_component(DiceRollForm)]
pub fn dice_roll_form(props: &Props) -> Html {

    pub async fn roll_dice(roll: &str) -> Result<roll_dice::RollResult, JsError> {
        let roll_result = roll_dice(roll);
    }

    let onclick = move |e:MouseEvent| {
        e.prevent_default();

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let roll:String = document
            .get_element_by_id("roll_text")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse()
            .unwrap();
        console::log_1(&format!("Roll {:?}", roll).into());

        let roll_result = roll_dice(&roll);

        console::log_1(&format!("Roll result: {:?}", roll_result.unwrap()).into());

        // let results = document
        //     .get_element_by_id("dice_roll_results")
        //     .unwrap();

        // let val = document.create_element("p").unwrap();
        // val.set_text_content(Some(&format!("Dice roll total: {:?}, Rolls: {:?}", dice_roll_total, rolls)));
    
        // results.append_child(&val).unwrap();
    };

    html! {    
        <form id="dice_roller_form">
            <span id="error_message"></span>
            <label for="dice_text">{"Dice to roll:"}</label>
            <input type="text" placeholder={"7d7+7"} name="dice_text" id="roll_text" />
            <p>{"Result: "}<span id="dice_result"></span></p>
            <button id="roll_dice" {onclick}>{"Roll Dice"}</button>
        </form>
    }
}