use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let parent = Object::new()?;
    parent.set_name("parent")?;

    let child = Object::new()?;
    child.set_name("child")?;
    parent.add_child(&child);

    let resolved = parent
        .child_at(0)
        .and_then(|first_child| first_child.path())
        .and_then(|path| parent.at_path(&path).ok().flatten());

    println!("object_info={:?}", parent.info()?);
    println!("resolved_kind={:?}", resolved.map(|object| object.kind()));
    println!("✅ modelio object OK");
    Ok(())
}
