use modelio::prelude::*;

#[test]
fn vertex_attribute_and_descriptor_surfaces_are_available() {
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)
        .expect("box mesh");
    let descriptor = mesh.vertex_descriptor().expect("vertex descriptor");
    let descriptor_info = descriptor.info().expect("descriptor info");
    assert!(descriptor_info.attribute_count > 0);

    let attribute =
        VertexAttribute::new("custom", vertex_format::FLOAT3, 0, 0).expect("custom attribute");
    attribute.set_initialization_value([0.0, 0.0, 0.0, 1.0]);
    assert_eq!(attribute.info().expect("attribute info").buffer_index, 0);
}
