extern crate destructivator;

// TODO: Support these arguments:
// **repo_dir** (default: current)
// **base** (default: master)
// **compare** (default: HEAD)
fn main() {
    let destructive_changes_xml = destructivator::destructive_changes_xml();
    println!("{}", destructive_changes_xml);
}
