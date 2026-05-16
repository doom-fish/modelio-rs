use modelio::prelude::*;

#[test]
fn light_reports_irradiance_and_updates_type() {
    let light = Light::new().expect("light");
    light.set_light_type(LightType::Ambient);
    light
        .set_color_space("kCGColorSpaceSRGB")
        .expect("color space");

    let info = light.info().expect("info");
    let irradiance = light.irradiance_at_point([0.0, 0.0, 0.0]);

    assert_eq!(info.light_type_enum(), Some(LightType::Ambient));
    assert!(irradiance.iter().all(|value| value.is_finite()));
}
