extern crate config;
extern crate git2;
extern crate ignore;

pub const IGNORE_FILE: &'static str = "destructivator.ignore";
pub const MAPPING_FILE: &'static str = "type-mapping.toml";
pub const NAME: &'static str = "destructivator";

pub mod forcedotcom;
pub mod git;

use forcedotcom::*;
use git::*;
use git2::{Delta, DiffDelta};
use std::collections::HashMap;
use std::path::{Path};

/// The destructiveChanges.xml file rendered as a `String`.
/// TODO: Expose to other languages:
/// https://doc.rust-lang.org/1.5.0/book/rust-inside-other-languages.html
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
