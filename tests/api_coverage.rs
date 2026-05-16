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
fn asset_mesh_material_and_texture_surface_is_present() {
    let asset_header = read_header("MDLAsset");
    assert_contains_all(
        &asset_header,
        &[
            "canImportFileExtension:",
            "canExportFileExtension:",
            "exportAssetToURL:",
            "- (void)addObject:",
            "- (MDLObject *)objectAtIndex:",
        ],
    );

    let mesh_header = read_header("MDLMesh");
    assert_contains_all(
        &mesh_header,
        &[
            "vertexDescriptor",
            "vertexAttributeDataForAttributeNamed:",
            "initBoxWithExtent:",
            "initCylinderWithExtent:",
        ],
    );

    let material_header = read_header("MDLMaterial");
    assert_contains_all(
        &material_header,
        &[
            "initWithName:(NSString*)name scatteringFunction:",
            "materialFace",
            "floatValue",
            "color",
        ],
    );

    let texture_header = read_header("MDLTexture");
    assert_contains_all(
        &texture_header,
        &[
            "initWithURL:(NSURL*)URL name:",
            "MDLCheckerboardTexture",
            "texelDataWithTopLeftOrigin",
            "writeToURL:",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "mdl_asset_new_empty",
            "mdl_asset_export_to_url",
            "mdl_mesh_vertex_descriptor",
            "mdl_material_new",
            "mdl_checkerboard_texture_new",
        ],
    );
}

#[test]
fn object_light_and_camera_surface_is_present() {
    let object_header = read_header("MDLObject");
    assert_contains_all(
        &object_header,
        &[
            "@property (nonatomic, readonly, copy) NSArray<id<MDLComponent>> *components;",
            "- (void)addChild:(MDLObject *)child;",
            "- (MDLObject*)objectAtPath:",
            "@property (nonatomic) BOOL hidden;",
        ],
    );

    let light_header = read_header("MDLLight");
    assert_contains_all(
        &light_header,
        &[
            "irradianceAtPoint:",
            "@property (nonatomic, readwrite) MDLLightType lightType;",
            "@interface MDLPhysicallyPlausibleLight",
            "setColorByTemperature:",
        ],
    );

    let camera_header = read_header("MDLCamera");
    assert_contains_all(
        &camera_header,
        &["projectionMatrix", "frameBoundingBox:", "lookAt:", "rayTo:"],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "mdl_object_new",
            "mdl_object_add_child",
            "mdl_light_new",
            "mdl_physically_plausible_light_new",
            "mdl_camera_new",
            "mdl_camera_ray_to",
        ],
    );
}

#[test]
fn voxel_animation_and_animated_value_surface_is_present() {
    let voxel_header = read_header("MDLVoxelArray");
    assert_contains_all(
        &voxel_header,
        &[
            "initWithData:(NSData*)voxelData",
            "voxelExistsAtIndex:",
            "voxelIndices",
            "coarseMesh",
        ],
    );

    let animation_header = read_header("MDLAnimation");
    assert_contains_all(
        &animation_header,
        &[
            "@interface MDLSkeleton",
            "@interface MDLPackedJointAnimation",
            "@interface MDLAnimationBindComponent",
        ],
    );

    let animated_header = read_header("MDLAnimatedValueTypes");
    assert_contains_all(
        &animated_header,
        &[
            "@interface MDLAnimatedScalar",
            "@interface MDLAnimatedVector3",
            "@interface MDLAnimatedQuaternion",
            "@interface MDLAnimatedMatrix4x4",
            "@interface MDLAnimatedScalarArray",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "mdl_voxel_array_new_with_indices",
            "mdl_packed_joint_animation_new",
            "mdl_animation_bind_component_new",
            "mdl_animated_scalar_new",
            "mdl_animated_vector3_array_new",
            "mdl_animated_quaternion_array_new",
        ],
    );
}

#[test]
fn submesh_and_vertex_attribute_surface_is_present() {
    let submesh_header = read_header("MDLSubmesh");
    assert_contains_all(
        &submesh_header,
        &[
            "@property (nonatomic, readonly, retain) id<MDLMeshBuffer> indexBuffer;",
            "indexBufferAsIndexType:",
            "@property (nonatomic, retain, nullable) MDLMaterial *material;",
        ],
    );

    let vertex_header = read_header("MDLVertexDescriptor");
    assert_contains_all(
        &vertex_header,
        &[
            "@interface MDLVertexAttribute",
            "initWithName:(NSString *)name",
            "@interface MDLVertexDescriptor",
            "attributeNamed:",
            "setPackedOffsets",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "mdl_submesh_index_buffer_as_type",
            "mdl_submesh_set_material",
            "mdl_vertex_attribute_new",
            "mdl_vertex_descriptor_new_copy",
            "mdl_vertex_descriptor_attribute_named",
        ],
    );
}
