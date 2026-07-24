use dioxus::{core::IntoAttributeValue, html::u::is, prelude::*};
use dioxus_style::with_css;

use crate::{STATE, launcher::{offline_login, online_login}, view::view::Route};


enum LoginMode {
    None,
    Online,
    Offline
}

#[with_css(css, "src/view/css/login.css")]
pub fn Login() -> Element {
    let navigator = use_navigator();
    let mut border_color = use_signal(|| "#282A36");
    let mut login_mode = use_signal(|| LoginMode::None);

    // para a conta offline
    let username_input = use_signal(|| String::new());
    rsx! {
        div {
            class: css::login,
            div {
                class: css::login_center,
                border_color: border_color.read().to_string(),
                onmouseenter: move |_| border_color.set("#7E57C2"),
                onmouseleave: move |_| border_color.set("#282A36"),
                
                UserButton {  }

            match *login_mode.read() {
                LoginMode::None => rsx!{
                    button {
                        onclick: move |_| {
                            spawn(async move {
                                login_mode.set(LoginMode::Online);
                                println!("{:?}", online_login().await);
                            });
                        },
                        "Online"
                    }
                    button {
                        onclick: move |_| {
                            login_mode.set(LoginMode::Offline);
                        },
                        "Offline"
                    }
                },
                LoginMode::Online => rsx!{
                    OnlineLoginPage { login_mode }
                },
                LoginMode::Offline => rsx!{
                    OfflineLoginPage { username_input, login_mode }
                }
            }
            }
        }
    }
}


#[with_css(css, "src/view/css/login.css")]
fn UserButton() -> Element {
    let navigator = use_navigator();
    match &STATE.read().current_user_profile {
        Some(r) => rsx!  {
            button {
                onclick: move |_| {
                    navigator.replace(Route::Home {  });
                },
                "{r.username}"
            }
        },
        None => rsx! {

        }
    }
}

#[with_css(css, "src/view/css/login.css")]
fn OnlineLoginPage(login_mode: Signal<LoginMode>) -> Element {
    rsx! {
        button {
            onclick: move |_| {
                login_mode.set(LoginMode::None);
            },
            "<"
        }
        div {
            "online"
        }
    }
}


#[with_css(css, "src/view/css/login.css")]
fn OfflineLoginPage(username_input: Signal<String>, login_mode: Signal<LoginMode>) -> Element {
    rsx! {
        div {
            class: css::offline_login_page,
            button {
                onclick: move |_| {
                    login_mode.set(LoginMode::None);
                },
                "<"
            }
            input {
                oninput: move |evt| {
                    username_input.set(evt.value());
                },
                placeholder: "Nome"
            }
            button { 
                onclick: move |_| {
                    if !username_input.read().is_empty() {
                        spawn(async move {
                            match offline_login(username_input.read().to_string()).await {
                                Ok(u) => {
                                    login_mode.set(LoginMode::None);
                                    info!("{:?}", u);
                                }
                                Err(err) => {
                                    info!("{}", err);
                                }
                            }
                        });
                    }
                    
                },
                "Criar"
            }
        }
        
    }
}
