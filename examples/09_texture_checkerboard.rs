use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let texture = Texture::new_checkerboard(
        4.0,
        Some("checker"),
        [4, 4],
        4,
        TextureChannelEncoding::UInt8,
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    )?;

    println!("texture_info={:?}", texture.info()?);
    println!("texel_bytes={}", texture.texel_data_top_left().len());
    println!("✅ modelio texture OK");
    Ok(())
}
