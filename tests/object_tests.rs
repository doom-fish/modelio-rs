use modelio::prelude::*;

#[test]
fn object_hierarchy_round_trips_children_and_paths() {
    let parent = Object::new().expect("parent");
    parent.set_name("parent").expect("set parent name");

    let child = Object::new().expect("child");
    child.set_name("child").expect("set child name");
    parent.add_child(&child);

    let first_child = parent.child_at(0).expect("first child");
    let child_path = first_child.path().expect("child path");
    let resolved = parent.at_path(&child_path).expect("resolve child path");

    assert_eq!(parent.child_count(), 1);
    assert_eq!(first_child.name().as_deref(), Some("child"));
    assert!(resolved.is_some());
}
