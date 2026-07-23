mod view;

use dioxus::prelude::*;
use view::{home::Home, login::Login};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Login {},
    #[route("/home")]
    Home {}
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    // variavel de ambiente em tempo de compilação para fazer build do app
    const CLIENT_ID: &str = env!("CLIENT_ID");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
