use modelio::prelude::*;

#[test]
fn skeleton_exposes_joint_paths_and_default_transforms() {
    let skeleton = Skeleton::new("Rig", &["/root", "/root/joint"]).expect("skeleton");
    let info = skeleton.info().expect("skeleton info");

    assert_eq!(info.joint_count, 2);
    let bind_array = skeleton
        .joint_bind_transform_array()
        .expect("bind transform array");
    let rest_array = skeleton
        .joint_rest_transform_array()
        .expect("rest transform array");

    assert_eq!(bind_array.info().expect("bind info").element_count, 2);
    assert_eq!(rest_array.info().expect("rest info").element_count, 2);
    assert_eq!(bind_array.float_matrices().expect("bind matrices").len(), 2);
    assert_eq!(rest_array.float_matrices().expect("rest matrices").len(), 2);
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
