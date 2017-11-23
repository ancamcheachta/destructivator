extern crate git2;

use git2::build::{RepoBuilder};
use std::env;
use std::fs;
use std::path::PathBuf;

const FORCEDOTCOM_BRANCH_ALL_NEW: &'static str = "all-new";
const FORCEDOTCOM_TEST_REPO: &'static str = "https://github.com/ancamcheachta/forcedotcom-project.git";
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
    
    let test_repos = root_dir.join("test-repos");
    let branch_all_new = test_repos.join(FORCEDOTCOM_BRANCH_ALL_NEW);
    
    if !branch_all_new.exists() {
        RepoBuilder::new().branch(FORCEDOTCOM_BRANCH_ALL_NEW)
            .clone(FORCEDOTCOM_TEST_REPO, &branch_all_new);
    }
}