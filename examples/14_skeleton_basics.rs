use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let skeleton = Skeleton::new("Rig", &["/root", "/root/joint"])?;

    println!("skeleton_info={:?}", skeleton.info()?);
    println!(
        "joint_bind_transforms={}",
        skeleton.joint_bind_transforms()?.len()
    );
    println!(
        "joint_rest_transforms={}",
        skeleton.joint_rest_transforms()?.len()
    );
    println!("✅ modelio skeleton OK");
    Ok(())
}
