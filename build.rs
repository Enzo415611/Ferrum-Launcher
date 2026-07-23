use std::path::Path;

pub fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let env_path = Path::new(&manifest_dir).join(".env");
    dotenvy::from_path(env_path).ok();
    let client_id: String = std::env::var("CLIENT_ID").expect("CLIENT_ID not found");

    println!("cargo:rustc-env=CLIENT_ID={}", client_id);
    println!("cargo:rerun-if-changed=.env");
}
