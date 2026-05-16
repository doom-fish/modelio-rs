use modelio::prelude::*;

#[test]
fn skeleton_exposes_joint_paths_and_default_transforms() {
    let skeleton = Skeleton::new("Rig", &["/root", "/root/joint"]).expect("skeleton");
    let info = skeleton.info().expect("skeleton info");

    assert_eq!(info.joint_count, 2);
    assert_eq!(
        skeleton
            .joint_bind_transforms()
            .expect("bind transforms")
            .len(),
        2
    );
    assert_eq!(
        skeleton
            .joint_rest_transforms()
            .expect("rest transforms")
            .len(),
        2
    );
}
