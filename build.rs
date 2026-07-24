pub fn main() {
    dotenvy::from_filename(".env")
        .expect(".env not loaded");
    let client_id: String = std::env::var("CLIENT_ID").expect("CLIENT_ID not found");

    println!("cargo:rustc-env=CLIENT_ID={}", client_id);
    println!("cargo:rerun-if-changed=.env");
}
