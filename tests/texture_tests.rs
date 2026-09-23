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

#[test]
fn channel_counts_beyond_i32_are_errors() {
    let too_many = usize::try_from(i64::from(i32::MAX) + 1).expect("64-bit usize");
    let checkerboard = Texture::new_checkerboard(
        4.0,
        None,
        [4, 4],
        too_many,
        TextureChannelEncoding::UInt8,
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    )
    .expect_err("oversized channel count must fail");
    assert_eq!(checkerboard.code(), -1);

    let noise = Texture::new_scalar_noise(
        0.5,
        None,
        [4, 4],
        too_many,
        TextureChannelEncoding::UInt8,
        false,
    )
    .expect_err("oversized channel count must fail");
    assert_eq!(noise.code(), -1);
}
