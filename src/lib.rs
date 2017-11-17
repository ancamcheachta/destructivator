extern crate git2;

use git2::Repository;
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
        
        println!("head oid: {:?}, master oid: {:?}", head_oid, master_oid);
    }
}
