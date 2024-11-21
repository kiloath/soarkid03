use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config =
        cbindgen::Config::from_file(format!("{}\\assist\\{}", &crate_dir, "cbindgen.toml"))
            .expect("Failed to read cbindgen.toml");

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!("{}\\soarkid03\\{}", &crate_dir, "soarkid03.hpp"));
}
