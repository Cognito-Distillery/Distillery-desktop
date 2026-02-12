fn main() {
    if let Ok(path) = dotenvy::dotenv() {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    if let Ok(val) = std::env::var("API_BASE_URL") {
        println!("cargo:rustc-env=API_BASE_URL={}", val);
    }
    if let Ok(val) = std::env::var("WEB_BASE_URL") {
        println!("cargo:rustc-env=WEB_BASE_URL={}", val);
    }
    tauri_build::build()
}
