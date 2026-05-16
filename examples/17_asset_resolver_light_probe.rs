use std::path::PathBuf;

use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let resolver = PathAssetResolver::new(&format!("file://{}", fixture_dir.display()))?;
    println!(
        "triangle.obj -> {:?}",
        resolver
            .as_asset_resolver()
            .resolve_asset_named("triangle.obj")?
    );

    let sky = Texture::new_sky_cube(
        Some("sky"),
        [4, 4],
        TextureChannelEncoding::UInt8,
        0.5,
        0.8,
        0.1,
        0.2,
    )?;
    sky.update_sky_cube();

    let data_source = LightProbeIrradianceDataSource::new(
        BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 1.0],
        },
        1,
        |_| vec![0.0; 12],
    )?;
    let probes = Asset::place_light_probes(0.5, ProbePlacement::UniformGrid, &data_source)?;

    println!("generated {} light probes (sky cube? {})", probes.len(), sky.info()?.is_cube);
    Ok(())
}
