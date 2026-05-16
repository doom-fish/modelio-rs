use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let light = Light::new()?;
    light.set_light_type(LightType::Ambient);
    light.set_color_space("kCGColorSpaceSRGB")?;

    println!("light_info={:?}", light.info()?);
    println!(
        "irradiance={:?}",
        light.irradiance_at_point([0.0, 0.0, 0.0])
    );
    println!("✅ modelio light OK");
    Ok(())
}
