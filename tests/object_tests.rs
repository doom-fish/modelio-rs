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

    let container = ObjectContainer::new().expect("object container");
    container.add_object(&child);
    assert_eq!(container.count(), 1);
    assert_eq!(
        container.object_at(0).and_then(|object| object.name()),
        Some("child".into())
    );
    parent.set_children_container(Some(&container));
    assert_eq!(
        parent
            .children_container()
            .expect("children container")
            .count(),
        1
    );
}
