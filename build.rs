use std::env;
use std::fs;
use std::path::PathBuf;

const IGNORE_FILE: &'static str = "destructivator.ignore";
const MAPPING_FILE: &'static str = "type-mapping.toml";

#[allow(unused_must_use)]
fn main() {
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap().to_string();
    let root_dir = env::home_dir().unwrap().join(&format!(".{}", pkg_name));
    let assets_dir = root_dir.join("assets");
    
    fs::create_dir(&root_dir);
    fs::create_dir(&assets_dir);
    
    let man_dir: &str = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let man_assets_dir = PathBuf::from(man_dir).join("assets");
    
    fs::copy(
        man_assets_dir.join(IGNORE_FILE),
        &assets_dir.join(IGNORE_FILE).to_str().unwrap()
    );
    fs::copy(
        man_assets_dir.join(MAPPING_FILE),
        &assets_dir.join(MAPPING_FILE).to_str().unwrap()
    );
}