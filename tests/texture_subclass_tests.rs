use modelio::prelude::*;

#[test]
fn texture_subclass_factories_create_textures() {
    let color_temperature = Texture::new_color_temperature_gradient(2000.0, 6500.0, Some("temp"), [2, 8])
        .expect("color temperature gradient");
    assert_eq!(color_temperature.info().expect("temp info").dimensions, [2, 8]);

    let color_gradient = Texture::new_color_gradient(
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        Some("gradient"),
        [2, 8],
    )
    .expect("color gradient");
    assert_eq!(color_gradient.info().expect("gradient info").dimensions, [2, 8]);

    let vector_noise = Texture::new_vector_noise(
        0.2,
        Some("vector"),
        [4, 4],
        TextureChannelEncoding::UInt8,
    )
    .expect("vector noise");
    assert_eq!(vector_noise.info().expect("vector info").dimensions, [4, 4]);

    let scalar_noise = Texture::new_scalar_noise(
        0.2,
        Some("scalar"),
        [4, 4],
        4,
        TextureChannelEncoding::UInt8,
        true,
    )
    .expect("scalar noise");
    assert_eq!(scalar_noise.info().expect("scalar info").dimensions, [4, 4]);

    let cellular_noise = Texture::new_cellular_noise(
        0.75,
        Some("cellular"),
        [4, 4],
        TextureChannelEncoding::UInt8,
    )
    .expect("cellular noise");
    assert_eq!(cellular_noise.info().expect("cellular info").dimensions, [4, 4]);

    let checkerboard = Texture::new_checkerboard(
        4.0,
        Some("checker"),
        [4, 4],
        4,
        TextureChannelEncoding::UInt8,
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    )
    .expect("checkerboard");
    let normal_map = Texture::new_normal_map(&checkerboard, Some("normal"), 0.5, 0.75)
        .expect("normal map");
    assert_eq!(normal_map.info().expect("normal info").dimensions, [4, 4]);

    let sky = Texture::new_sky_cube(
        Some("sky"),
        [4, 4],
        TextureChannelEncoding::UInt8,
        0.5,
        0.8,
        0.1,
        0.2,
    )
    .expect("sky cube");
    sky.update_sky_cube();
    assert!(sky.info().expect("sky info").is_cube);

    let sky_azimuth = Texture::new_sky_cube_with_azimuth(
        Some("sky-azimuth"),
        [4, 4],
        TextureChannelEncoding::UInt8,
        0.5,
        0.8,
        1.0,
        0.1,
        0.2,
    )
    .expect("sky cube with azimuth");
    sky_azimuth.update_sky_cube();
    assert!(sky_azimuth.info().expect("sky azimuth info").is_cube);
}
