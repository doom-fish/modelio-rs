use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let material = Material::new("ExampleMaterial", true)?;
    let roughness = material
        .property_with_semantic(MaterialSemantic::Roughness)
        .expect("roughness property");
    roughness.set_float(0.35);

    let base_color = material
        .property_with_semantic(MaterialSemantic::BaseColor)
        .expect("base color property");
    base_color.set_color([0.8, 0.2, 0.1, 1.0]);

    println!("material_info={:?}", material.info()?);
    println!("roughness={:?}", roughness.info()?.float_value);
    println!("✅ modelio material OK");
    Ok(())
}
