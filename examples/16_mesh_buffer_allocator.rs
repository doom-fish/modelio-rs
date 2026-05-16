use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let allocator = MeshBufferDataAllocator::new()?;
    let zone = allocator.new_default_zone(64)?;
    let buffer = allocator
        .as_mesh_buffer_allocator()
        .new_buffer_with_data(&[1, 2, 3, 4], MeshBufferType::Vertex)?;
    let map = buffer.map()?;

    println!("zone capacity={} bytes={:?}", zone.capacity(), map.bytes());
    Ok(())
}
