use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let camera = Camera::new()?;
    camera.set_projection(CameraProjection::Perspective);
    camera.set_field_of_view(45.0);
    camera.look_at_from([0.0, 0.0, -1.0], [0.0, 0.0, 3.0]);

    println!("camera_info={:?}", camera.info()?);
    println!("camera_ray={:?}", camera.ray_to([10, 10], [20, 20]));
    println!("✅ modelio camera OK");
    Ok(())
}
