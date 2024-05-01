use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use rand::Rng;

/// A simple Spin HTTP component.
#[http_component]
fn handle_dice_roller(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1..=6);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(format!("{:?}", roll))
        .build())
}
