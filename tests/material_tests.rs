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

#[test]
fn material_sampler_filter_and_graph_surfaces_round_trip() {
    let material = Material::new("GraphMaterial", true).expect("material");
    let material_base_color = material
        .property_with_semantic(MaterialSemantic::BaseColor)
        .expect("material base color");
    assert!(material_base_color.name().is_some());

    let base_color = MaterialProperty::new("GraphBaseColor", MaterialSemantic::BaseColor)
        .expect("base color property");
    let roughness = MaterialProperty::new("GraphRoughness", MaterialSemantic::Roughness)
        .expect("roughness property");
    assert_eq!(base_color.name().as_deref(), Some("GraphBaseColor"));
    assert_eq!(roughness.name().as_deref(), Some("GraphRoughness"));

    let filter = TextureFilter::new().expect("texture filter");
    filter.set_s_wrap_mode(MaterialTextureWrapMode::Repeat);
    filter.set_t_wrap_mode(MaterialTextureWrapMode::Mirror);
    filter.set_r_wrap_mode(MaterialTextureWrapMode::Clamp);
    filter.set_min_filter(MaterialTextureFilterMode::Linear);
    filter.set_mag_filter(MaterialTextureFilterMode::Nearest);
    filter.set_mip_filter(MaterialMipMapFilterMode::Linear);
    let filter_info = filter.info().expect("filter info");
    assert_eq!(
        filter_info.s_wrap_mode_enum(),
        Some(MaterialTextureWrapMode::Repeat)
    );
    assert_eq!(
        filter_info.t_wrap_mode_enum(),
        Some(MaterialTextureWrapMode::Mirror)
    );
    assert_eq!(
        filter_info.min_filter_enum(),
        Some(MaterialTextureFilterMode::Linear)
    );
    assert_eq!(
        filter_info.mag_filter_enum(),
        Some(MaterialTextureFilterMode::Nearest)
    );
    assert_eq!(
        filter_info.mip_filter_enum(),
        Some(MaterialMipMapFilterMode::Linear)
    );

    let texture = Texture::new_checkerboard(
        4.0,
        Some("checker"),
        [16, 16],
        4,
        TextureChannelEncoding::UInt8,
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
    )
    .expect("checkerboard texture");
    let transform = Transform::from_matrix([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.25, 0.5, 0.0, 1.0,
    ])
    .expect("transform");

    let sampler = TextureSampler::new().expect("texture sampler");
    sampler.set_texture(Some(&texture));
    sampler.set_hardware_filter(Some(&filter));
    sampler.set_transform(Some(&transform));
    let sampler_info = sampler.info().expect("sampler info");
    assert!(sampler_info.has_texture);
    assert!(sampler_info.has_hardware_filter);
    assert!(sampler_info.has_transform);
    assert!(sampler.texture().is_some());
    assert!(sampler.hardware_filter().is_some());
    assert!(sampler.transform().is_some());

    base_color.set_texture_sampler(Some(&sampler));
    assert!(base_color.texture().is_some());
    assert!(base_color.texture_sampler().is_some());

    let connection = MaterialPropertyConnection::new(&base_color, &roughness).expect("connection");
    assert!(connection.output().is_some());
    assert!(connection.input().is_some());

    let node = MaterialPropertyNode::new(&[&roughness], &[&base_color]).expect("node");
    assert_eq!(node.inputs().expect("node inputs").len(), 1);
    assert_eq!(node.outputs().expect("node outputs").len(), 1);

    // ModelIO promotes a graph's external inputs/outputs from its child nodes.
    // Using an empty node keeps this smoke test from tripping its duplicate-assignment exception.
    let graph_node = MaterialPropertyNode::new(&[], &[]).expect("graph node");
    let graph = MaterialPropertyGraph::new(&[&graph_node], &[]).expect("graph");
    graph.evaluate();
    assert_eq!(graph.nodes().expect("graph nodes").len(), 1);
    assert_eq!(graph.connections().expect("graph connections").len(), 0);
}

#[test]
fn scattering_function_wrappers_round_trip() {
    let scattering = ScatteringFunction::new().expect("scattering function");
    scattering.set_name("Lambert").expect("scattering name");
    assert_eq!(scattering.name().as_deref(), Some("Lambert"));

    let base_color = scattering.base_color().expect("base color");
    base_color.set_color([0.6, 0.5, 0.4, 1.0]);
    assert_eq!(
        base_color.info().expect("base color info").color,
        Some([0.6, 0.5, 0.4, 1.0])
    );

    let pbr = PhysicallyPlausibleScatteringFunction::new().expect("pbr scattering");
    assert_eq!(pbr.version(), 1);
    pbr.metallic().expect("metallic").set_float(0.75);
    pbr.clearcoat().expect("clearcoat").set_float(0.25);

    let material = Material::new_with_scattering_function("CustomPbr", &pbr.as_scattering_function())
        .expect("material with scattering");
    let material_scattering = material.scattering_function().expect("material scattering");
    let pbr_scattering = material_scattering
        .as_physically_plausible_scattering_function()
        .expect("physically plausible scattering");
    assert_eq!(
        pbr_scattering
            .metallic()
            .expect("material metallic")
            .info()
            .expect("material metallic info")
            .float_value,
        Some(0.75)
    );
    assert_eq!(
        pbr_scattering
            .clearcoat()
            .expect("material clearcoat")
            .info()
            .expect("material clearcoat info")
            .float_value,
        Some(0.25)
    );
}
