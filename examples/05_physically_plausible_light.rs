use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let light = PhysicallyPlausibleLight::new()?;
    light.set_color_temperature(3200.0);
    light.set_lumens(1500.0);
    light.set_inner_cone_angle(15.0);
    light.set_outer_cone_angle(40.0);

    println!("physically_plausible_light={:?}", light.info()?);
    println!("✅ modelio physically plausible light OK");
    Ok(())
}
