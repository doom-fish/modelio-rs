mod common;

use modelio::prelude::*;

#[test]
fn asset_resolvers_handle_fixture_paths() {
    let fixture = common::fixture_obj();
    let fixture_dir_uri = common::fixture_dir_uri();

    let path_resolver = PathAssetResolver::new(&fixture_dir_uri).expect("path resolver");
    let resolver = path_resolver.as_asset_resolver();
    assert!(resolver
        .can_resolve_asset_named("triangle.obj")
        .expect("can resolve"));
    let resolved_path = resolver
        .resolve_asset_named("triangle.obj")
        .expect("resolve")
        .expect("resolved path");
    assert!(resolved_path.contains("triangle.obj"));
    assert!(path_resolver.path().expect("path").contains("file://"));

    let asset = Asset::from_url(&fixture).expect("asset");
    let relative_resolver = RelativeAssetResolver::new(&asset).expect("relative resolver");
    assert!(relative_resolver
        .as_asset_resolver()
        .can_resolve_asset_named("triangle.obj")
        .expect("relative resolution"));
    assert!(relative_resolver.asset().is_some());
    relative_resolver.set_asset(None);
    assert!(relative_resolver.asset().is_none());
    relative_resolver.set_asset(Some(&asset));
    assert!(relative_resolver.asset().is_some());

    let bundle_resolver =
        BundleAssetResolver::new(env!("CARGO_MANIFEST_DIR")).expect("bundle resolver");
    bundle_resolver
        .set_path(env!("CARGO_MANIFEST_DIR"))
        .expect("bundle path");
    assert!(bundle_resolver.path().is_some());

    let material = Material::new("ResolverMaterial", true).expect("material");
    material.load_textures_using_resolver(&path_resolver.as_asset_resolver());
}

#[test]
fn custom_asset_resolver_callback_round_trip() {
    let resolver = AssetResolver::new(|event| match event {
        AssetResolverEvent::CanResolveAssetNamed(name) => {
            AssetResolverResponse::Bool(name == "embedded.usdz")
        }
        AssetResolverEvent::ResolveAssetNamed(name) => AssetResolverResponse::Url(
            (name == "embedded.usdz").then(|| format!("file:///virtual/{name}")),
        ),
    })
    .expect("custom resolver");

    assert!(resolver
        .can_resolve_asset_named("embedded.usdz")
        .expect("can resolve embedded"));
    assert!(!resolver
        .can_resolve_asset_named("missing.usdz")
        .expect("can resolve missing"));
    assert_eq!(
        resolver
            .resolve_asset_named("embedded.usdz")
            .expect("resolve embedded")
            .as_deref(),
        Some("file:///virtual/embedded.usdz")
    );
    assert!(resolver
        .resolve_asset_named("missing.usdz")
        .expect("resolve missing")
        .is_none());
}
