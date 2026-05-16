use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let joint_paths = ["/root", "/root/joint"];
    let skeleton = Skeleton::new("Skeleton", &joint_paths)?;
    let animation = PackedJointAnimation::new("Walk", &joint_paths)?;
    animation
        .translations()?
        .set_float3_array(&[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], 0.0);

    let component = AnimationBindComponent::new()?;
    component.set_skeleton(&skeleton);
    component.set_packed_joint_animation(&animation);
    component.set_joint_paths(&joint_paths)?;
    component.set_geometry_bind_transform([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]);

    println!("animation_bind_component={:?}", component.info()?);
    println!("packed_animation={:?}", animation.info()?);
    println!("✅ modelio animation OK");
    Ok(())
}
