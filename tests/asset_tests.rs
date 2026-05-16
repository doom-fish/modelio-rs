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
