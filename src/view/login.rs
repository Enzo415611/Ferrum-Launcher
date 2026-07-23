use dioxus::{document::Style, html::u::style, prelude::{AssetVariant::Css, *}};
use dioxus_primitives::select::SelectGroup;
use dioxus_style::{scoped_style, with_css};
use crate::{Route};

#[component]
#[with_css(css, "src/view/css/login.css")]
pub fn Login() -> Element {
    let navigator = use_navigator();
    let mut border_color = use_signal(|| "#282A36") ;
    
    rsx! {
        div { class: css::login,
            div {
                class: css::login_center,
                border_color: border_color.read().to_string(),
                onmouseenter: move |_| border_color.set("#7E57C2"),
                onmouseleave: move |_| border_color.set("#282A36"),
                button { "nome da conta" }
                div { class: css::center,
                    button { "Online" }
                    button { "Offline" }
                }
            }
        }
    }
    

}