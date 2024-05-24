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
extern crate regex;
use rand::{thread_rng, Rng};
use regex::Regex;

pub mod components;

use components::dice_roll_form::DiceRollForm;

#[function_component]
fn App() -> Html {


    html! {
        <DiceRollForm />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}