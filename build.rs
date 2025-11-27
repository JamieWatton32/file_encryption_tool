fn main() {
    if std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("src/RustProofIcon.ico");
        res.compile().unwrap();
    }
}