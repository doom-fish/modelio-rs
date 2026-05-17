use modelio::prelude::*;

#[test]
fn camera_look_at_and_ray_generation_work() {
    let camera = Camera::new().expect("camera");
    camera.set_projection(CameraProjection::Perspective);
    camera.set_field_of_view(45.0);
    camera.look_at_from([0.0, 0.0, -1.0], [0.0, 0.0, 3.0]);

    let ray = camera.ray_to([10, 10], [20, 20]);
    assert!(ray[2] < 0.0);
    assert_eq!(
        camera.info().expect("info").projection_enum(),
        Some(CameraProjection::Perspective)
    );

    let stereo = StereoscopicCamera::new().expect("stereoscopic camera");
    stereo.set_inter_pupillary_distance(64.0);
    stereo.set_left_vergence(1.0);
    stereo.set_right_vergence(1.0);
    stereo.set_overlap(0.1);
    assert_eq!(
        stereo.info().expect("stereo info").inter_pupillary_distance,
        64.0
    );
}
