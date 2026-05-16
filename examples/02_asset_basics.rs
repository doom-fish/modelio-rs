use std::path::PathBuf;

use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/triangle.obj");
    let asset = Asset::from_url(&fixture)?;
    let info = asset.info()?;

    println!("asset_count={}", info.count);
    println!("asset_bounds={:?}", asset.bounding_box());
    println!(
        "first_kind={:?}",
        asset.object_at(0).map(|object| object.kind())
    );
    println!("✅ modelio asset OK");
    Ok(())
}
