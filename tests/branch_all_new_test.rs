use destructivator::{destructive_changes_xml, NAME};
use std::env;
use std::path::PathBuf;

fn branch_all_new_dir() -> PathBuf {
    let root_dir = env::home_dir().unwrap().join(&format!(".{}", NAME));
    root_dir.join("test-repos").join("all-new")
}

#[test]
#[allow(unused_must_use)]
fn it_knows_its_first_members() {
    env::set_current_dir(&branch_all_new_dir());
    
    let destructive_changes = destructive_changes_xml();
    
    assert!(destructive_changes.contains("<name>CustomApplication</name>"));
    assert!(destructive_changes.contains("<members>standard__AppLauncher</members>"));
}

#[test]
#[allow(unused_must_use)]
fn it_knows_its_middle_members() {
    env::set_current_dir(&branch_all_new_dir());
    
    let destructive_changes = destructive_changes_xml();
    
    assert!(destructive_changes.contains("<name>HomePageLayout</name>"));
    assert!(destructive_changes.contains("<members>DE Default</members>"));
}

#[test]
#[allow(unused_must_use)]
fn it_knows_its_last_members() {
    env::set_current_dir(&branch_all_new_dir());
    
    let destructive_changes = destructive_changes_xml();
    
    assert!(destructive_changes.contains("<name>ApexTrigger</name>"));
    assert!(destructive_changes.contains("<members>orderBeforeInsert</members>"));
}