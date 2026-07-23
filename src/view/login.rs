use dioxus::prelude::*;
use dioxus_primitives::select::SelectGroup;

use crate::{Route};

#[derive(Debug, Clone, )]
struct Users {

}

#[component]
pub fn Login() -> Element {
    let navigator = use_navigator();
    let mut border_color = use_signal(|| "#282A36") ;
    rsx! {
        div {
            class: "login",
            div {
                class: "login-center",
                border_color: border_color.read().to_string(),
                onmouseenter: move |_| border_color.set("#7E57C2"),
                onmouseleave: move |_| border_color.set("#282A36"),
                button {
                    "nome da conta"
                }
                div {
                    class: "center",
                    button { "Online" }
                    button { "Offline"}
                }
            }
        }
    }
}