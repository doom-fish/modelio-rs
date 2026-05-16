use modelio::prelude::*;

#[test]
fn packed_joint_animation_binds_to_skeleton_component() {
    let joint_paths = ["/root", "/root/joint"];
    let skeleton = Skeleton::new("Rig", &joint_paths).expect("skeleton");
    let animation = PackedJointAnimation::new("Walk", &joint_paths).expect("animation");
    animation
        .translations()
        .expect("translations")
        .set_float3_array(&[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], 0.0);

    let component = AnimationBindComponent::new().expect("bind component");
    component.set_skeleton(&skeleton);
    component.set_packed_joint_animation(&animation);
    component
        .set_joint_paths(&joint_paths)
        .expect("joint paths");

    let info = component.info().expect("component info");
    assert!(info.has_skeleton);
    assert!(info.has_joint_animation);
    assert_eq!(info.joint_paths.expect("joint paths").len(), 2);
}
