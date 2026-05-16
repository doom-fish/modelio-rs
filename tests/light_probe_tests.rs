use modelio::prelude::*;

#[test]
fn light_probe_and_irradiance_data_source_surface_is_available() {
    let sky = Texture::new_sky_cube(
        Some("sky"),
        [4, 4],
        TextureChannelEncoding::UInt8,
        0.5,
        0.8,
        0.1,
        0.2,
    )
    .expect("sky texture");
    sky.update_sky_cube();

    let probe = LightProbe::new(Some(&sky), Some(&sky)).expect("light probe");
    assert!(probe.reflective_texture().is_some());
    assert!(probe.irradiance_texture().is_some());
    probe.generate_spherical_harmonics_from_irradiance(1);
    let _ = probe.spherical_harmonics_coefficients();
    assert!(probe.as_light().info().expect("light info").light_type_enum().is_some());

    let data_source = LightProbeIrradianceDataSource::new(
        BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 1.0],
        },
        1,
        |position| vec![position[0], position[1], position[2], 1.0].into_iter().cycle().take(12).collect(),
    )
    .expect("irradiance data source");
    assert!(data_source
        .bounding_box()
        .max
        .iter()
        .zip([1.0, 1.0, 1.0])
        .all(|(left, right)| (*left - right).abs() < f32::EPSILON));
    assert_eq!(data_source.spherical_harmonics_level(), 1);
    data_source.set_spherical_harmonics_level(2);
    assert_eq!(data_source.spherical_harmonics_level(), 2);

    let probes = Asset::place_light_probes(0.5, ProbePlacement::UniformGrid, &data_source)
        .expect("place light probes");
    assert!(!probes.is_empty());
    assert!(probes[0]
        .as_light()
        .irradiance_at_point([0.0, 0.0, 0.0])
        .iter()
        .all(|value| value.is_finite()));
}
