use crate::{
    db::{
        db::get_db,
        entity::user::{ActiveModel, Entity},
    },
    launcher::online_login,
    view::{home::Home, login::Login},
    AppState,
};
use dioxus::prelude::*;
use lighty_launcher::{auth::AuthProvider, UserProfile};
use sea_orm::EntityTrait;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Login {},
    #[route("/home")]
    Home {}
}
#[derive(Debug, Clone)]
enum InitApp {
    Loading,
    Loaded(AppState),
    Err(String),
}

#[component]
pub fn App() -> Element {
    use_context_provider(|| AppState {
        current_user_profile: Signal::new(None),
    });

    let mut current_user_profile = use_context::<AppState>().current_user_profile.clone();

    match &*current_user_profile.read() {
        Some(profile) => {
            match &profile.provider {
                AuthProvider::Microsoft {
                    client_id,
                    refresh_token,
                } => {
                    spawn(async move {
                        match online_login().await {
                            Ok(profile) => {
                                info!("{:?}", profile);
                                *current_user_profile.write() = Some(profile);
                            }
                            Err(_) => {
                                *current_user_profile.write() = None;
                            }
                        }
                    });
                }
                AuthProvider::Offline => {
                    spawn(async move {
                        let conn = get_db().await;

                        let last_profile = match Entity::find().one(conn).await {
                            Ok(profile) => profile,
                            Err(_) => None,
                        };

                        if let Some(profile) = last_profile {
                            info!("aqui");
                            *current_user_profile.write() = Some(UserProfile {
                                access_token: None,
                                banned: false,
                                email: None,
                                email_verified: false,
                                id: None,
                                money: None,
                                provider: AuthProvider::Offline,
                                xuid: None,
                                role: None,
                                uuid: profile.uuid,
                                token_handle: None,
                                username: profile.name,
                            });
                        }

                        //let all_offline_profiles = match Entity::find().all(&conn).await {
                        //    Ok(profiles) => profiles,
                        //    Err(_) => vec![]
                        //};
                    });
                }
                _ => {}
            }
        }
        None => {}
    };

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
