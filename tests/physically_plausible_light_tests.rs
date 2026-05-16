use modelio::prelude::*;

fn assert_close(left: f32, right: f32) {
    assert!((left - right).abs() < 1e-6, "left={left} right={right}");
}

#[test]
fn physically_plausible_light_mutates_photometric_properties() {
    let light = PhysicallyPlausibleLight::new().expect("light");
    light.set_color_temperature(3200.0);
    light.set_lumens(2000.0);
    light.set_inner_cone_angle(10.0);
    light.set_outer_cone_angle(30.0);
    light.set_attenuation_start_distance(1.0);
    light.set_attenuation_end_distance(5.0);

    let info = light.info().expect("info");
    assert_close(info.lumens, 2000.0);
    assert_close(info.inner_cone_angle, 10.0);
    assert_close(info.outer_cone_angle, 30.0);
    assert_close(info.attenuation_start_distance, 1.0);
    assert_close(info.attenuation_end_distance, 5.0);
}
