extern crate web_sys;
use yew::{
    Html,
    html,
    function_component
};

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