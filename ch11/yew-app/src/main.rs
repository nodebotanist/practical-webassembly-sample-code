extern crate web_sys;
use web_sys::{console, window, HtmlInputElement};
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

    // let onchange = Callback::from(|e: );
    fn onchange(change_event:Event) {

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
            let input_value = document
            .get_element_by_id("num_of_dice")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        console::log_1(&format!("Changed: {:?}", input_value).into());
    }

    html! {
        <div>
            <input type="number" id="num_of_dice" name="number_of_dice" placeholder=0 min="0" max="100" onchange={onchange} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}