use modelio::prelude::*;

#[test]
fn checkerboard_texture_exposes_info_and_bytes() {
    let texture = Texture::new_checkerboard(
        4.0,
        Some("checker"),
        [4, 4],
        4,
        TextureChannelEncoding::UInt8,
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    )
    .expect("checkerboard texture");

    let info = texture.info().expect("texture info");
    assert_eq!(info.dimensions, [4, 4]);
    assert_eq!(texture.texel_data_top_left().len(), 64);
}
