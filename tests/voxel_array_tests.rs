mod common;

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

    let allocator = MeshBufferDataAllocator::new().expect("allocator");
    assert!(
        voxels
            .coarse_mesh_with_allocator(Some(&allocator.as_mesh_buffer_allocator()))
            .expect("coarse mesh with allocator")
            .vertex_count()
            > 0
    );
    assert!(
        voxels
            .mesh_with_allocator(Some(&allocator.as_mesh_buffer_allocator()))
            .expect("smooth mesh with allocator")
            .vertex_count()
            > 0
    );
}

#[test]
fn voxel_array_can_be_built_from_assets_and_meshes() {
    let asset = Asset::from_url(common::fixture_obj()).expect("fixture asset");
    let voxels = VoxelArray::from_asset(&asset, 8, 0.05).expect("voxelized asset");
    assert!(voxels.count() > 0);

    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)
        .expect("box mesh");
    let empty = VoxelArray::new(
        &[],
        BoundingBox {
            min: [-1.0, -1.0, -1.0],
            max: [1.0, 1.0, 1.0],
        },
        0.25,
    )
    .expect("empty voxel array");
    empty.set_voxels_for_mesh(&mesh, 6, 0.05);
    assert!(empty.count() > 0);
}
