mod common;

use modelio::prelude::*;

#[test]
fn asset_loads_fixture_and_exports() {
    let fixture = common::fixture_obj();
    assert!(Asset::can_import_file_extension("obj"));

    let asset = Asset::from_url(&fixture).expect("load fixture asset");
    assert!(asset.count() > 0);
    assert!(asset.object_at(0).is_some());

    if Asset::can_export_file_extension("obj") {
        let export_path = common::output_dir("asset").join("triangle_copy.obj");
        asset.export_to_url(&export_path).expect("export asset");
        assert!(export_path.exists());
    }
}

#[test]
fn empty_asset_adds_mesh_object() {
    let asset = Asset::new().expect("new asset");
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)
        .expect("box mesh");
    asset.add_object(&mesh.as_object());

    assert_eq!(asset.count(), 1);
    assert_eq!(asset.object_at(0).expect("object").kind(), ObjectKind::Mesh);
}

#[test]
fn from_url_reports_missing_corrupt_and_unknown_files() {
    let missing = Asset::from_url(common::output_dir("asset").join("missing.obj"))
        .expect_err("missing file must fail");
    assert!(!missing.message().is_empty());

    let corrupt_path = common::output_dir("asset").join("corrupt.obj");
    std::fs::write(&corrupt_path, b"this is not a mesh\n").expect("write corrupt fixture");
    let corrupt = Asset::from_url(&corrupt_path).expect_err("corrupt file must fail");
    assert!(!corrupt.message().is_empty());

    let unknown_path = common::output_dir("asset").join("unknown.xyz");
    std::fs::write(&unknown_path, b"hello").expect("write unknown fixture");
    let unknown = Asset::from_url(&unknown_path).expect_err("unknown extension must fail");
    assert!(!unknown.message().is_empty());
}

#[test]
fn from_url_with_options_conforms_meshes_to_the_descriptor() {
    let descriptor = VertexDescriptor::new().expect("vertex descriptor");
    let position = VertexAttribute::new(
        &vertex_attribute_name::position().expect("position name"),
        vertex_format::FLOAT3,
        0,
        0,
    )
    .expect("position attribute");
    descriptor.add_or_replace_attribute(&position);
    descriptor.layout_at(0).expect("layout 0").set_stride(12);

    let asset = Asset::from_url_with_options(common::fixture_obj(), Some(&descriptor), None, true)
        .expect("load fixture asset");
    let mesh = asset.mesh_at(0).expect("fixture mesh");
    assert_eq!(mesh.vertex_count(), 3);
    assert_eq!(
        mesh.vertex_buffer(0).expect("vertex buffer").info().expect("info").length,
        36
    );

    descriptor.layout_at(0).expect("layout 0").set_stride(8);
    let error = Asset::from_url_with_options(common::fixture_obj(), Some(&descriptor), None, false)
        .expect_err("inconsistent descriptor must be rejected");
    assert_eq!(error.code(), -1);
}
