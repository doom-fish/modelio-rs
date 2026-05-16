use modelio::prelude::*;

#[test]
fn physically_plausible_material_properties_round_trip() {
    let material = Material::new("MaterialTest", true).expect("material");
    material.set_material_face(MaterialFace::DoubleSided);

    let roughness = material
        .property_with_semantic(MaterialSemantic::Roughness)
        .expect("roughness");
    roughness.set_float(0.42);

    let base_color = material
        .property_with_semantic(MaterialSemantic::BaseColor)
        .expect("base color");
    base_color.set_color([0.1, 0.2, 0.3, 1.0]);

    assert_eq!(material.material_face(), Some(MaterialFace::DoubleSided));
    assert_eq!(roughness.info().expect("info").float_value, Some(0.42));
    assert_eq!(
        base_color.info().expect("info").color,
        Some([0.1, 0.2, 0.3, 1.0])
    );
}
