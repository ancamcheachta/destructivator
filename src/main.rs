extern crate destructivator;

fn main() {
    let destructive_changes_xml = destructivator::destructive_changes_xml(None, None, None);
    println!("{}", destructive_changes_xml);
}
