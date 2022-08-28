use reqwasm::http::Request;
use train_model::HelloResponse;
use yew::prelude::*;

#[function_component(App)]
fn app_component() -> Html {
    const ACTIX_PORT: &str = std::env!("ACTIX_PORT");
    let actix_url: String = format!("http://localhost:{}", ACTIX_PORT);
}

fn main() {
    yew::start_app::<App>();
}
