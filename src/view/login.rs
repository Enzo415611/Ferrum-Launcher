use std::sync::LazyLock;

use dioxus::prelude::*;
use dioxus_style::with_css;
use regex::Regex;
use sea_orm::ActiveModelTrait;

use crate::{
    db::{db::get_db, entity::user::ActiveModel},
    launcher::{new_online_login, offline_login},
    view::view::Route,
    AppState,
};

enum LoginMode {
    None,
    Online,
    Offline,
}

#[with_css(css, "src/view/css/login.css")]
pub fn Login() -> Element {
    let navigator = use_navigator();
    let mut border_color = use_signal(|| "#282A36");
    let mut login_mode = use_signal(|| LoginMode::None);
    // para a conta offline
    let username_input = use_signal(|| String::new());
    let mut current_user_profile = use_context::<AppState>().current_user_profile.clone();

    rsx! {
        div { class: css::login,
            div {
                class: css::login_center,
                border_color: border_color.read().to_string(),
                onmouseenter: move |_| border_color.set("#7E57C2"),
                onmouseleave: move |_| border_color.set("#282A36"),
                UserButton {}

                match *login_mode.read() {
                    LoginMode::None => rsx! {
                        button {
                            onclick: move |_| {
                                spawn(async move {
                                    login_mode.set(LoginMode::Online);
                                    match new_online_login().await {
                                        Ok(user) => {
                                            login_mode.set(LoginMode::None);
                                            info!("{:?}", user);
                                            *current_user_profile.write() = Some(user);
                                        }
                                        Err(err) => {}
                                    }
                                });
                            },
                            "Online Login"
                        }
                        button {
                            onclick: move |_| {
                                login_mode.set(LoginMode::Offline);
                            },
                            "Offline Login"
                        }
                    },
                    LoginMode::Online => rsx! {
                        OnlineLoginPage { login_mode }
                    },
                    LoginMode::Offline => rsx! {
                        OfflineLoginPage { username_input, login_mode }
                    },
                }
            }
        }
    }
}

#[with_css(css, "src/view/css/login.css")]
fn UserButton() -> Element {
    let navigator = use_navigator();

    match &*use_context::<AppState>().current_user_profile.read() {
        Some(r) => rsx! {
            button {
                onclick: move |_| {
                    navigator.replace(Route::Home {});
                },
                "{r.username}"
            }
        },
        None => rsx! {},
    }
}

#[with_css(css, "src/view/css/login.css")]
fn OnlineLoginPage(login_mode: Signal<LoginMode>) -> Element {
    let err_msg = use_signal(|| String::new());

    rsx! {
        button {
            onclick: move |_| {
                login_mode.set(LoginMode::None);
            },
            "<"
        }
        div {
            h2 { class: css::err_msg, "{err_msg}" }
        }

    }
}

#[with_css(css, "src/view/css/login.css")]
fn OfflineLoginPage(username_input: Signal<String>, login_mode: Signal<LoginMode>) -> Element {
    let mut err_msg = use_signal(|| String::new());
    let mut current_user_profile = use_context::<AppState>().current_user_profile.clone();

    rsx! {
        div { class: css::offline_login_page,
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
                placeholder: "Nome",
            }
            button {
                onclick: move |_| {
                    if !username_input.read().is_empty() {
                        if USERNAME_REGEX.is_match(&username_input.read().to_string()) {
                            spawn(async move {
                                match offline_login(username_input.read().to_string()).await {
                                    Ok(u) => {
                                        login_mode.set(LoginMode::None);
                                        info!("{:?}", u);

                                        let new_profile = ActiveModel {
                                            username: sea_orm::ActiveValue::Set(u.username.clone()),
                                            uuid: sea_orm::ActiveValue::Set(u.uuid.clone()),
                                            refresh_token: sea_orm::ActiveValue::Set(None),
                                            ..Default::default()
                                        };

                                        _ = new_profile.insert(get_db().await).await;

                                        *current_user_profile.write() = Some(u);



                                    }
                                    Err(err) => {
                                        err_msg.set(err.to_string());
                                    }
                                }
                            });
                        } else {
                            // invalid
                            err_msg.set(
                                "O nome deve ter pelo menos 3 a 16 caracteres\nUse apenas letras, números e sublinhados".to_string(),
                            );
                        }
                    } else {
                        err_msg.set("Digite um nome para sua conta".to_string());
                    }
                },
                "Criar"
            }
        }

        div {
            h2 { class: css::err_msg, "{err_msg}" }
        }
    }
}

static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[a-zA-Z0-9_]{3,16}$").unwrap());
