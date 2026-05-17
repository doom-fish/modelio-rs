mod common;

use modelio::prelude::*;

#[test]
fn sdk_constants_and_matrix_arrays_are_exposed() {
    assert!(ut_type::alembic().is_some());
    assert!(ut_type::object_3d().is_some());
    assert!(ut_type::polygon().is_some());
    assert!(ut_type::stereolithography().is_some());
    assert!(ut_type::universal_scene_description().is_some());
    assert!(ut_type::universal_scene_description_mobile().is_some());

    assert!(vertex_attribute_name::position().is_some());
    assert!(vertex_attribute_name::normal().is_some());
    assert!(vertex_attribute_name::texture_coordinate().is_some());
    assert!(vertex_attribute_name::joint_indices().is_some());

    let array = Matrix4x4Array::new(2).expect("matrix array");
    let first = [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];
    let second = [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 2.0, 3.0, 1.0,
    ];

    array.set_float_matrices(&[first, second]);
    assert_eq!(array.info().expect("matrix info").element_count, 2);
    assert_eq!(
        array.float_matrices().expect("float matrices"),
        vec![first, second]
    );

    array.clear();
    let cleared = array.float_matrices().expect("cleared matrices");
    assert_eq!(cleared, vec![[0.0; 16], [0.0; 16]]);
}

#[test]
fn utility_converts_fixture_to_usdz_when_available() {
    let output = common::output_dir("utility").join("triangle.usdz");
    if output.exists() {
        std::fs::remove_file(&output).expect("remove stale usdz");
    }

    match Utility::convert_to_usdz(common::fixture_obj(), &output) {
        Ok(()) => assert!(output.exists()),
        Err(error) => {
            assert!(
                error.to_string().contains("macOS 15") || error.to_string().contains("unavailable"),
                "unexpected utility error: {error}"
            );
        }
    }
}
