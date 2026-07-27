use crate::{
    db::{db::get_db, entity::user::Entity},
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

#[component]
pub fn App() -> Element {
    use_context_provider(move || AppState {
        current_user_profile: Signal::new(None),
    });

    let current_user_profile = use_context::<AppState>().current_user_profile.clone();

    init_app(current_user_profile);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

fn init_app(mut current_user_profile: Signal<Option<UserProfile>>) {
    use_resource(move || async move {
        match Entity::find().one(get_db().await).await {
            Ok(profile) => {
                if let Some(profile) = profile {
                    if profile.refresh_token.is_none() {
                        // offline
                        info!("offline");
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
                            username: profile.username,
                        });
                    } else {
                        // online
                        info!("online");
                        match online_login().await {
                            Ok(profile) => {
                                info!("{:?}", profile);
                                *current_user_profile.write() = Some(profile);
                            }
                            Err(_) => {
                                *current_user_profile.write() = None;
                            }
                        }
                    }
                }
            }
            Err(err) => {
                info!("{}", err);
            }
        }
    });
}
