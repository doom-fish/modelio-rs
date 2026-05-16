use modelio::prelude::*;

#[test]
fn voxel_array_tracks_voxels_and_generates_meshes() {
    let voxels = VoxelArray::new(
        &[],
        BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 1.0],
        },
        1.0,
    )
    .expect("voxel array");

    voxels.set_voxel([0, 0, 0, 0]);

    assert!(voxels.voxel_exists([0, 0, 0, 0], false, false, false, false));
    assert_eq!(voxels.voxel_indices(), vec![[0, 0, 0, 0]]);
    assert!(voxels.coarse_mesh().expect("coarse mesh").vertex_count() > 0);
}
