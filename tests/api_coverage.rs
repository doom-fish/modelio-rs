use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim())
}

fn read(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn read_header(name: &str) -> String {
    read(&sdk_root().join(format!(
        "System/Library/Frameworks/ModelIO.framework/Headers/{name}.h"
    )))
}

fn read_bridge() -> String {
    let bridge_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("swift-bridge/Sources/ModelIOBridge");
    let mut paths = std::fs::read_dir(&bridge_dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "swift"))
        .collect::<Vec<_>>();
    paths.sort();
    paths
        .iter()
        .map(|path| read(path))
        .collect::<Vec<_>>()
        .join("\n")
}

fn assert_contains_all(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(haystack.contains(needle), "missing `{needle}`");
    }
}

#[test]
fn mdl_asset_surface_is_present() {
    let header = read_header("MDLAsset");
    assert_contains_all(
        &header,
        &[
            "initWithURL:",
            "canImportFileExtension:",
            "@property (nonatomic, readonly) MDLAxisAlignedBoundingBox boundingBox;",
            "@property (nonatomic, readonly, retain, nullable) NSURL *URL;",
            "@property (nonatomic, readonly) NSUInteger count;",
            "- (MDLObject *)objectAtIndex:(NSUInteger)index;",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "mdl_asset_new_with_url",
            "mdl_asset_can_import_file_extension",
            "mdl_asset_count",
            "mdl_asset_bounding_box",
            "mdl_asset_url_string",
            "mdl_asset_mesh_at_index",
        ],
    );
}

#[test]
fn mdl_mesh_surface_is_present() {
    let header = read_header("MDLMesh");
    assert_contains_all(
        &header,
        &[
            "vertexAttributeDataForAttributeNamed:",
            "@property (nonatomic, readwrite) NSUInteger vertexCount;",
            "@property (nonatomic, readwrite, retain) NSArray<id<MDLMeshBuffer>> *vertexBuffers;",
            "@property (nonatomic, copy, nullable) NSMutableArray<MDLSubmesh*> *submeshes;",
            "@property (nonatomic, readonly) MDLAxisAlignedBoundingBox boundingBox;",
            "initBoxWithExtent:",
            "initSphereWithExtent:",
            "initCylinderWithExtent:",
            "initPlaneWithExtent:",
            "initIcosahedronWithExtent:",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "mdl_mesh_new_box",
            "mdl_mesh_new_ellipsoid",
            "mdl_mesh_new_cylinder",
            "mdl_mesh_new_plane",
            "mdl_mesh_new_icosahedron",
            "mdl_mesh_vertex_attribute_data",
            "mdl_mesh_vertex_buffer_at",
            "mdl_mesh_submesh_at",
        ],
    );
}

#[test]
fn mdl_material_surface_is_present() {
    let header = read_header("MDLMaterial");
    assert_contains_all(
        &header,
        &[
            "@property (nonatomic, assign) MDLMaterialSemantic semantic;",
            "@property (nonatomic, assign) MDLMaterialPropertyType type;",
            "@property (nonatomic, copy, nullable) NSString *stringValue;",
            "@property (nonatomic, copy, nullable) NSURL *URLValue;",
            "@property (nonatomic, retain, nullable) MDLTextureSampler *textureSamplerValue;",
            "@property (nullable, nonatomic) CGColorRef color;",
            "@property (nonatomic, assign) float floatValue;",
            "@property (nonatomic, assign) vector_float2 float2Value;",
            "@property (nonatomic, assign) vector_float3 float3Value;",
            "@property (nonatomic, assign) vector_float4 float4Value;",
            "@property (nonatomic, assign) matrix_float4x4 matrix4x4;",
            "@property (nonatomic, assign) float luminance;",
            "- (nullable MDLMaterialProperty*)propertyNamed:(NSString*)name;",
            "- (nullable MDLMaterialProperty*)propertyWithSemantic:(MDLMaterialSemantic)semantic;",
            "@property (nonatomic, readonly) NSUInteger count;",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "mdl_material_count",
            "mdl_material_property_named",
            "mdl_material_property_with_semantic",
            "mdl_material_property_info_json",
            "mdl_material_property_texture",
        ],
    );
}

#[test]
fn mdl_submesh_and_texture_surface_is_present() {
    let submesh_header = read_header("MDLSubmesh");
    assert_contains_all(
        &submesh_header,
        &[
            "@property (nonatomic, readonly, retain) id<MDLMeshBuffer> indexBuffer;",
            "@property (nonatomic, readonly) NSUInteger indexCount;",
            "@property (nonatomic, readonly) MDLIndexBitDepth indexType;",
            "@property (nonatomic, readonly) MDLGeometryType geometryType;",
            "@property (nonatomic, retain, nullable) MDLMaterial *material;",
        ],
    );

    let texture_header = read_header("MDLTexture");
    assert_contains_all(
        &texture_header,
        &[
            "- (instancetype)initWithURL:(NSURL*)URL name:(nullable NSString*)name;",
            "- (nullable NSData *)texelDataWithTopLeftOrigin;",
            "- (nullable NSData *)texelDataWithBottomLeftOrigin;",
            "@property (nonatomic, readonly) vector_int2 dimensions;",
            "@property (nonatomic, readonly) NSInteger rowStride;",
            "@property (nonatomic, readonly) NSUInteger channelCount;",
            "@property (nonatomic, readonly) MDLTextureChannelEncoding channelEncoding;",
            "@property (nonatomic) BOOL isCube;",
            "@property (nonatomic) BOOL hasAlphaValues;",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "mdl_submesh_index_buffer",
            "mdl_submesh_material",
            "mdl_url_texture_new",
            "mdl_texture_info_json",
            "mdl_texture_copy_texel_data",
        ],
    );
}
