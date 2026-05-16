use modelio::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mesh = Mesh::new_box([1.0, 2.0, 3.0], [1, 1, 1], false, GeometryType::Triangles)?;

    let bbox = mesh.bounding_box();
    let vertex_count = mesh.vertex_count();
    let submesh_count = mesh.submesh_count();
    let first_submesh = mesh
        .submesh(0)
        .ok_or_else(|| std::io::Error::other("missing generated submesh"))?;
    let index_buffer = first_submesh
        .index_buffer()
        .ok_or_else(|| std::io::Error::other("missing generated index buffer"))?;
    let index_bytes = index_buffer.bytes()?;

    if vertex_count == 0 {
        return Err(std::io::Error::other("generated mesh had zero vertices").into());
    }
    if submesh_count == 0 {
        return Err(std::io::Error::other("generated mesh had zero submeshes").into());
    }
    if index_bytes.is_empty() {
        return Err(std::io::Error::other("generated submesh had no index data").into());
    }

    println!("vertex count: {vertex_count}");
    println!("submesh count: {submesh_count}");
    println!("bounding box: min={:?} max={:?}", bbox.min, bbox.max);
    println!("✅ modelio primitive OK");
    Ok(())
}
