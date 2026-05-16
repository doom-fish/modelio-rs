use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)?;
    let descriptor = mesh.vertex_descriptor().expect("vertex descriptor");
    let attribute = VertexAttribute::new("custom_position", vertex_format::FLOAT3, 0, 0)?;
    attribute.set_initialization_value([0.0, 0.0, 0.0, 1.0]);

    println!("descriptor_info={:?}", descriptor.info()?);
    println!("custom_attribute={:?}", attribute.info()?);
    println!("✅ modelio vertex attribute OK");
    Ok(())
}
