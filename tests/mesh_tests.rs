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
