use dioxus::prelude::*;

use crate::view::{login::Login, home::Home};

const MAIN_CSS: Asset = asset!("/assets/main.css");


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Login {},
    #[route("/home")]
    Home {}
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
