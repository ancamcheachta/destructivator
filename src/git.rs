use super::*;
use config::File;
use git2::{Diff, Oid, Reference, Repository};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::env;
use std::path::PathBuf;
use std::process::exit;

/// The diff between master and head commits.
pub fn diff<'a>(repo: &'a Repository, master_oid: &'a Oid, head_oid: &'a Oid) -> Diff<'a> {
    if master_oid == head_oid {
        println!("{}","HEAD identical to master. Exiting.");
        exit(0);
    }
    
    match repo.diff_tree_to_tree(
        Some(&repo.find_commit(*master_oid).unwrap().tree().unwrap()),
        Some(&repo.find_commit(*head_oid).unwrap().tree().unwrap()),
        None
    ) {
        Ok(diff) => diff,
        Err(e) => panic!("failed to produce diff: {}", e),
    }
}

/// The repository feature branch to be compared to master.
pub fn head(repo: &Repository) -> Reference {
    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => panic!("failed to resolve HEAD: {}", e),
    };
    
    if !head.is_branch() {
        panic!("{} is not a branch.", head.name().unwrap());
    }
    
    head
}

/// The destructivator.ignore file compiled
pub fn gitignore(file_path: Option<&PathBuf>) -> Gitignore {
    let ignore_file_path = ignore_file_path(file_path);
    let mut builder = GitignoreBuilder::new(env::current_dir().unwrap().as_path());
    
    builder.add(ignore_file_path.as_path());
    builder.build().unwrap()
}

/// The path to the destructivator.ignore file
fn ignore_file_path(file_path: Option<&PathBuf>) -> PathBuf {
    match file_path {
        None => {
            let root_dir = env::home_dir().unwrap().join(&format!(".{}", NAME));
            let assets_dir = root_dir.join("assets");
            assets_dir.join(IGNORE_FILE)
        },
        Some(path) => path.join(IGNORE_FILE),
    }
}

/// The repository master branch.
pub fn master(repo: &Repository) -> Reference {
    match repo.find_reference("refs/remotes/origin/master") {
        Ok(master) => master,
        Err(e) => panic!("failed to resolve origin/master: {}", e),
    }
}

/// The repository of the Force.com project for which to generate a rollback branch.
pub fn repo() -> Repository {
    match Repository::open(env::current_dir().unwrap()) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}

/// The `type-mapping.toml` config file loaded into a `HashMap<String, String>`
pub fn type_mapping(file_path: Option<&PathBuf>) -> HashMap<String, String> {
    let type_mapping_file_path = type_mapping_file_path(file_path);
    let mut settings = config::Config::default();
    settings.merge(File::from(type_mapping_file_path)).unwrap();
    match settings.try_into::<HashMap<String, String>>() {
        Err(e) => panic!("could not read type mapping: {}", e),
        Ok(type_mapping) => type_mapping,
    }
}

/// The path to the `type-mapping.toml` file
fn type_mapping_file_path(file_path: Option<&PathBuf>) -> PathBuf {
    match file_path {
        None => {
            let root_dir = env::home_dir().unwrap().join(&format!(".{}", NAME));
            let assets_dir = root_dir.join("assets");
            assets_dir.join(MAPPING_FILE)
        },
        Some(path) => path.join(MAPPING_FILE),
    }
}
