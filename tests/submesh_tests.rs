use modelio::prelude::*;

#[test]
fn submesh_sets_material_and_exposes_index_buffers() {
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)
        .expect("box mesh");
    let submesh = mesh.submesh(0).expect("submesh");
    let material = Material::new("SubmeshMaterial", true).expect("material");
    submesh.set_material(Some(&material));

    assert!(submesh.index_count() > 0);
    assert!(!submesh
        .index_buffer()
        .expect("index buffer")
        .bytes()
        .expect("bytes")
        .is_empty());
    assert!(submesh
        .index_buffer_as_type(IndexBitDepth::UInt32)
        .is_some());
    assert!(submesh.material().is_some());

    let topology = SubmeshTopology::new(&submesh).expect("submesh topology");
    assert!(topology.face_count() > 0);
    submesh.set_topology(Some(&topology));
    assert!(submesh.topology().is_some());
}
