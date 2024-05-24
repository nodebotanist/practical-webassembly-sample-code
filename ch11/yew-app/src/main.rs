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
        <DiceRollForm default_num_dice=3 default_dice_max_value=6 default_modifier=0 />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}