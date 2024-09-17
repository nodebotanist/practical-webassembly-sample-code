use yew::prelude::*;
use gloo_console::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{HtmlInputElement, EventTarget};

#[function_component(App)]
pub fn app() -> Html {

    let dice_roll_string_handle = use_state(||"7d7+7");
    let dice_roll_string = (*dice_roll_string_handle).clone();
    
    let oninput = {
        let dice_roll_string = dice_roll_string.clone();

        Callback::from(move |e: InputEvent| {
            let dice_roll_string = dice_roll_string.clone();
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let mut pinned = std::pin::pin!(dice_roll_string);
                pinned.as_mut().set(&input.value());
                log!(JsValue::from(dice_roll_string));
            }
        })
    };

    html! {
        <main>
            <h1>{ "My Yew Dice Rolling App" }</h1>

        </main>
    }
}
