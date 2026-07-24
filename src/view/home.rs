use dioxus::prelude::*;

use crate::view::view::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "home",
            Link { to: Route::Login {},
                h1 { "Login" }
            }
        }
    }
}