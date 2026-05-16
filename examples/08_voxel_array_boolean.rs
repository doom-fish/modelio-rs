use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let bounding_box = BoundingBox {
        min: [0.0, 0.0, 0.0],
        max: [1.0, 1.0, 1.0],
    };
    let voxels = VoxelArray::new(&[], bounding_box, 1.0)?;
    voxels.set_voxel([0, 0, 0, 0]);

    println!("voxel_info={:?}", voxels.info()?);
    println!(
        "voxel_exists={}",
        voxels.voxel_exists([0, 0, 0, 0], false, false, false, false)
    );
    println!(
        "coarse_mesh_vertices={}",
        voxels.coarse_mesh().expect("coarse mesh").vertex_count()
    );
    println!("✅ modelio voxel array OK");
    Ok(())
}
