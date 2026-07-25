mod db;
mod launcher;
mod view;

use std::thread;

use dioxus::signals::Signal;
use lighty_launcher::UserProfile;

use crate::{db::db::get_db, view::view::App};

#[tokio::main]
async fn main() {
    thread::spawn(|| lighty_launcher::core::AppState::init("FerrumLauncher").expect(""));
    get_db().await;
    // variavel de ambiente em tempo de compilação para fazer build do app
    //const CLIENT_ID: &str = env!("CLIENT_ID");

    dioxus::launch(App);
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    current_user_profile: Signal<Option<UserProfile>>,
}
