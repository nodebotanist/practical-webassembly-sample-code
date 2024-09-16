use yew::prelude::*;

mod roll_dice;

#[function_component(App)]
pub fn app() -> Html {
    let str = use_state(|| 10);
    let dex = use_state(|| 10);
    let con = use_state(|| 10);
    let int = use_state(|| 10);
    let wis = use_state(|| 10);
    let cha = use_state(|| 10);

    let dice_roll_string = use_state(||"7d7+7");
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
