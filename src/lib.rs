extern crate git2;
extern crate ignore;

use git2::{Delta, DiffDelta, Repository};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::env;
use std::path::Path;
use std::process::exit;

const IGNORE_FILE: &'static str = "destructivator.ignore";
const PKG_NAME: &'static str = "destructivator";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let repo = match Repository::open(env::current_dir().unwrap()) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };
        
        let head = match repo.head() {
            Ok(head) => head,
            Err(e) => panic!("failed to resolve HEAD: {}", e),
        };
        
        if(!head.is_branch()) {
            panic!("{} is not a branch.", head.name().unwrap());
        };
        
        let master = match repo.find_reference("refs/remotes/origin/master") {
            Ok(master) => master,
            Err(e) => panic!("failed to resolve origin/master: {}", e),
        };
        
        let (master_oid, head_oid) = (
            master.target().unwrap(),
            head.target().unwrap()
        );
        
        if master_oid == head_oid {
            println!("{}","HEAD identical to master. Exiting.");
            exit(0);
        }
        
        let diff = match repo.diff_tree_to_tree(
            Some(&repo.find_commit(master_oid).unwrap().tree().unwrap()),
            Some(&repo.find_commit(head_oid).unwrap().tree().unwrap()),
            None
        ) {
            Ok(diff) => diff,
            Err(e) => panic!("failed to produce diff: {}", e),
        };
        
        let root_dir = env::home_dir().unwrap().join(&format!(".{}", PKG_NAME));
        let assets_dir = root_dir.join("assets");
        let ignore_file_path = assets_dir.join(IGNORE_FILE);
        let mut builder = GitignoreBuilder::new(env::current_dir().unwrap().as_path());
        builder.add(ignore_file_path.as_path());
        let gitignore = builder.build().unwrap();
        let m = |path: &str| gitignore.matched_path_or_any_parents(Path::new(path), false);
        
        let deltas: Vec<DiffDelta> = diff.deltas()
            .filter_map(|d| match d.status() == Delta::Added { true => Some(d), false => None, })
            .filter(|d| !m(d.new_file().path().unwrap().to_str().unwrap()).is_ignore())
            .inspect(|d| println!("New file: {:?}", d.new_file().path().unwrap()))
            .collect();
    }
}
