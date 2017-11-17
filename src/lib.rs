extern crate git2;

use git2::{Delta, DiffDelta, Repository};
use std::env::current_dir;
use std::process::exit;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let repo = match Repository::open(current_dir().unwrap()) {
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
        
        let deltas: Vec<DiffDelta> = diff.deltas()
            .filter_map(|d| match d.status() == Delta::Added { true => Some(d), false => None, })
            .inspect(|d| println!("New file: {:?}", d.new_file().path().unwrap()))
            .collect();
    }
}
