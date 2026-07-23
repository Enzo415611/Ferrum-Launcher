use dotenvy::dotenv;

fn main() {
    let _ = dotenv().expect("env not loaded");
}