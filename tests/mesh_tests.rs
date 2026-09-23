mod common;

use modelio::prelude::*;

#[test]
fn mesh_primitives_have_geometry_and_descriptors() {
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)
        .expect("box mesh");

    assert!(mesh.vertex_count() > 0);
    assert!(mesh.submesh_count() > 0);
    assert!(mesh.vertex_descriptor().is_some());
    assert!(!mesh.vertex_buffers().is_empty());
}

#[test]
fn vertex_attribute_bytes_stay_inside_the_interleaved_buffer() {
    let asset = Asset::from_url(common::fixture_obj()).expect("load fixture asset");
    let mesh = asset.mesh_at(0).expect("fixture mesh");
    let descriptor = mesh.vertex_descriptor().expect("vertex descriptor");
    let names = [
        vertex_attribute_name::position().expect("position name"),
        vertex_attribute_name::normal().expect("normal name"),
        vertex_attribute_name::texture_coordinate().expect("texture coordinate name"),
    ];

    let mut offsets = Vec::new();
    let mut lengths = Vec::new();
    for name in &names {
        let data = mesh
            .vertex_attribute_data_named(name)
            .expect("attribute lookup")
            .expect("attribute data");
        let info = data.info().expect("attribute data info");
        let attribute = descriptor
            .attribute_named(name)
            .expect("descriptor lookup")
            .expect("descriptor attribute")
            .info()
            .expect("descriptor attribute info");
        let buffer = mesh
            .vertex_buffer(attribute.buffer_index)
            .expect("vertex buffer")
            .bytes()
            .expect("vertex buffer bytes");
        assert_eq!(info.buffer_size, buffer.len());

        let bytes = data.bytes().expect("attribute bytes");
        assert_eq!(bytes.len(), info.buffer_size - attribute.offset);
        assert_eq!(bytes.as_slice(), &buffer[attribute.offset..]);
        offsets.push(attribute.offset);
        lengths.push(bytes.len());
    }
    assert_eq!(offsets, [0, 12, 24]);
    assert_eq!(lengths, [96, 84, 72]);
}
