extern crate config;
extern crate git2;
extern crate ignore;

use config::File;
use git2::{Delta, Diff, DiffDelta, Oid, Reference, Repository};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;

const IGNORE_FILE: &'static str = "destructivator.ignore";
const MAPPING_FILE: &'static str = "type-mapping.toml";
const PKG_NAME: &'static str = "destructivator";

pub mod forcedotcom;
use forcedotcom::{Members, Package, Types, ToXML};

pub trait ToPackage {
    /// Converts to a `Package` struct
    fn to_package(&self, type_mapping_file: &HashMap<String,String>) -> Package;
}

impl<'a> ToPackage for Vec<DiffDelta<'a>> {
    fn to_package(&self, type_mapping: &HashMap<String,String>) -> Package {
        let mut package = Package::new(None);
        let mut types_hash_map: HashMap<String, Types> = HashMap::new();
        for delta in self.iter() {
            let (mut md_folder, mut component): (String, String) = (Default::default(), Default::default());
            let new_file_path = &delta.new_file().path().unwrap().to_str().unwrap();
            match new_file_path.rsplit("/").nth(1) {
                None => println!("warning: {} not in a type folder. Consider adding to {}", new_file_path, IGNORE_FILE),
                Some(m) => md_folder.push_str(&m),
            };
            if md_folder.len() > 0 {
                match new_file_path.rsplit("/").nth(0) {
                    None => component.push_str(&new_file_path),
                    Some(f) => match f.split(".").nth(0) {
                        None => component.push_str(&f),
                        Some(c) => component.push_str(&c),
                    },
                };
                
                if component.len() > 0 && type_mapping.contains_key(&md_folder) {
                    let md_type: String = type_mapping.get(&md_folder).unwrap().to_owned();
                    let members = Members::from(component);
                    
                    if types_hash_map.contains_key(&md_type) {
                        types_hash_map.insert(md_type.clone(), Types::new(&md_type));
                    }
                    
                    match types_hash_map.get_mut(&md_type) {
                        Some(ref mut types) => types.members.push(members),
                        None => {},
                    }
                }
            }
        }

        for key in types_hash_map.keys() {
            if let Some(types) = types_hash_map.get(key) {
                package.types.push(types.clone());
            }
        }
        
        package
    }
}

/// The destructiveChanges.xml file rendered as a `String`.
pub fn destructive_changes_xml() -> String {
    let repo = repo();
    let (head, master) = (head(&repo), master(&repo));
    let (master_oid, head_oid) = (
        master.target().unwrap(),
        head.target().unwrap()
    );
    let diff = diff(&repo, &master_oid, &head_oid);
    let (gitignore, type_mapping) = (gitignore(None), type_mapping(None));
    let m = |path: &str| gitignore.matched_path_or_any_parents(Path::new(path), false);
    
    let deltas: Vec<DiffDelta> = diff.deltas()
        .filter_map(|d| match d.status() == Delta::Added { true => Some(d), false => None, })
        .filter(|d| !m(d.new_file().path().unwrap().to_str().unwrap()).is_ignore())
        .collect();
    
    deltas.to_package(&type_mapping).to_xml()
}

/// The diff between master and head commits.
fn diff<'a>(repo: &'a Repository, master_oid: &'a Oid, head_oid: &'a Oid) -> Diff<'a> {
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
fn head(repo: &Repository) -> Reference {
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
fn gitignore(file_path: Option<&PathBuf>) -> Gitignore {
    let ignore_file_path = ignore_file_path(file_path);
    let mut builder = GitignoreBuilder::new(env::current_dir().unwrap().as_path());
    
    builder.add(ignore_file_path.as_path());
    builder.build().unwrap()
}

/// The path to the destructivator.ignore file
fn ignore_file_path(file_path: Option<&PathBuf>) -> PathBuf {
    match file_path {
        None => {
            let root_dir = env::home_dir().unwrap().join(&format!(".{}", PKG_NAME));
            let assets_dir = root_dir.join("assets");
            assets_dir.join(IGNORE_FILE)
        },
        Some(path) => path.join(IGNORE_FILE),
    }
}

/// The repository master branch.
fn master(repo: &Repository) -> Reference {
    match repo.find_reference("refs/remotes/origin/master") {
        Ok(master) => master,
        Err(e) => panic!("failed to resolve origin/master: {}", e),
    }
}

/// The repository of the Force.com project for which to generate a rollback branch.
fn repo() -> Repository {
    match Repository::open(env::current_dir().unwrap()) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}

fn type_mapping(file_path: Option<&PathBuf>) -> HashMap<String, String> {
    let type_mapping_file_path = type_mapping_file_path(file_path);
    let mut settings = config::Config::default();
    settings.merge(File::from(type_mapping_file_path)).unwrap();
    match settings.try_into::<HashMap<String, String>>() {
        Err(e) => panic!("could not read type mapping: {}", e),
        Ok(type_mapping) => type_mapping,
    }
}

fn type_mapping_file_path(file_path: Option<&PathBuf>) -> PathBuf {
    match file_path {
        None => {
            let root_dir = env::home_dir().unwrap().join(&format!(".{}", PKG_NAME));
            let assets_dir = root_dir.join("assets");
            assets_dir.join(MAPPING_FILE)
        },
        Some(path) => path.join(MAPPING_FILE),
    }
}

#[cfg(test)]
mod tests {
    use super::destructive_changes_xml;
    #[test]
    fn it_works() {
        println!("{}", destructive_changes_xml());
    }
}
