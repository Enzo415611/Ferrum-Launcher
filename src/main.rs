mod view;
mod launcher;

use std::thread;

use dioxus::signals::{GlobalSignal, Signal};
use lighty_launcher::UserProfile;

use crate::view::view::App;


pub static STATE: GlobalSignal<AppState> = Signal::global(|| {
    AppState {
        current_user_profile: None
    }
});

fn main() {
    thread::spawn(|| lighty_launcher::core::AppState::init("FerrumLauncher").expect(""));

    // variavel de ambiente em tempo de compilação para fazer build do app
    //const CLIENT_ID: &str = env!("CLIENT_ID");

    dioxus::launch(App);
}

#[derive(Debug, Clone, Default)]
struct AppState {
    current_user_profile: Option<UserProfile>
}

