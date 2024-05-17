extern crate web_sys;
use web_sys::{console, window, HtmlInputElement, MouseEvent};
use yew::{
    Html,
    html,
    Event,
    Callback, TargetCast,
    function_component
};
use crate::web_sys::wasm_bindgen::JsCast;

#[function_component]
fn App() -> Html {
    let onchange_num_of_dice = move |_:Event| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let input_value = document
            .get_element_by_id("num_of_dice")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        console::log_1(&format!("Changed number of dice: {:?}", input_value).into());
    };

    let onchange_dice_max_value = move |_:Event| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let input_value = document
            .get_element_by_id("dice_max_value")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        console::log_1(&format!("Changed dice max value: {:?}", input_value).into());
    };

    let onchange_modifier = move |_:Event| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let input_value = document
            .get_element_by_id("dice_modifier")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        console::log_1(&format!("Changed dice modifier: {:?}", input_value).into());
    };

    let on_dice_roll = move |_:MouseEvent| {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let number_of_dice = document
            .get_element_by_id("num_of_dice")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        let dice_max_value = document
            .get_element_by_id("dice_max_value")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        let dice_modifier = document
            .get_element_by_id("dice_modifier")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        console::log_1(&format!("Roll {:?}d{:?}+{:?}", number_of_dice, dice_max_value, dice_modifier).into());
    };

    html! {
        <div>
            <label for="number_of_dice">{"Number of Dice:"}</label>
            <input type="number" id="num_of_dice" name="number_of_dice" placeholder=1 min="0" max="100" onchange={ onchange_num_of_dice} /><br />
            <label for="number_of_dice">{"Max value of Dice:"}</label>
            <input type="number" id="dice_max_value" name="dice_max_value" placeholder=1 min="1" max="100" onchange={onchange_dice_max_value} /><br />
            <label for="number_of_dice">{"Dice Roll Modifier:"}</label>
            <input type="number" id="dice_modifier" name="dice_modifier" placeholder=0 min="-100" max="100" onchange={onchange_modifier} /><br />
            <button id="roll_dice_betton" onclick={on_dice_roll} type="submit" >{"Roll Dice"}</button>
            <div id ="dice_roll_results"></div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}