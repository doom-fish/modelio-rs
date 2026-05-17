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
            "@interface MDLAnimatedValue",
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

#[test]
fn transform_mesh_buffer_resolver_and_light_probe_surface_is_present() {
    let asset_header = read_header("MDLAsset");
    assert_contains_all(
        &asset_header,
        &[
            "@protocol MDLLightProbeIrradianceDataSource <NSObject>",
            "+ (NSArray<MDLLightProbe *> *)placeLightProbesWithDensity:",
        ],
    );

    let resolver_header = read_header("MDLAssetResolver");
    assert_contains_all(
        &resolver_header,
        &[
            "@protocol MDLAssetResolver <NSObject>",
            "@interface MDLRelativeAssetResolver",
            "@interface MDLPathAssetResolver",
            "@interface MDLBundleAssetResolver",
        ],
    );

    let transform_header = read_header("MDLTransform");
    assert_contains_all(
        &transform_header,
        &[
            "@protocol MDLTransformComponent <MDLComponent>",
            "@interface MDLTransform : NSObject <NSCopying, MDLTransformComponent>",
            "- (vector_float3)translationAtTime:",
            "- (void)setScale:(vector_float3)scale forTime:",
        ],
    );

    let transform_stack_header = read_header("MDLTransformStack");
    assert_contains_all(
        &transform_stack_header,
        &[
            "typedef NS_ENUM(NSUInteger, MDLTransformOpRotationOrder)",
            "@protocol MDLTransformOp",
            "@interface MDLTransformRotateXOp",
            "@interface MDLTransformOrientOp",
            "-(MDLTransformTranslateOp*) addTranslateOp:",
            "-(MDLAnimatedValue*) animatedValueWithName:",
        ],
    );

    let mesh_buffer_header = read_header("MDLMeshBuffer");
    assert_contains_all(
        &mesh_buffer_header,
        &[
            "@interface MDLMeshBufferMap : NSObject",
            "@interface MDLMeshBufferData : NSObject <MDLMeshBuffer>",
            "@protocol MDLMeshBufferAllocator <NSObject>",
            "@interface MDLMeshBufferDataAllocator: NSObject <MDLMeshBufferAllocator>",
            "@interface MDLMeshBufferZoneDefault : NSObject <MDLMeshBufferZone>",
        ],
    );

    let texture_header = read_header("MDLTexture");
    assert_contains_all(
        &texture_header,
        &[
            "@interface MDLSkyCubeTexture : MDLTexture",
            "@interface MDLColorSwatchTexture : MDLTexture",
            "@interface MDLNoiseTexture : MDLTexture",
            "@interface MDLNormalMapTexture : MDLTexture",
        ],
    );

    let light_header = read_header("MDLLight");
    assert_contains_all(
        &light_header,
        &[
            "@interface MDLLightProbe : MDLLight",
            "lightProbeWithTextureSize:",
            "@property (nonatomic, retain, nullable, readonly) MDLTexture *reflectiveTexture;",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "mdl_asset_resolver_can_resolve_named",
            "mdl_relative_asset_resolver_new",
            "mdl_mesh_buffer_allocator_new_zone",
            "mdl_mesh_buffer_zone_default_new",
            "mdl_color_swatch_texture_new_temperature_gradient",
            "mdl_normal_map_texture_new",
            "mdl_sky_cube_texture_new_with_azimuth",
            "mdl_transform_component_matrix",
            "mdl_transform_stack_add_orient_op",
            "mdl_transform_rotate_op_animated_value",
            "mdl_light_probe_new",
            "mdl_light_probe_irradiance_data_source_new",
            "mdl_asset_place_light_probes",
        ],
    );
}

#[test]
fn extended_surface_and_sdk_constants_are_present() {
    let animation_header = read_header("MDLAnimation");
    assert_contains_all(&animation_header, &["@protocol MDLJointAnimation"]);

    let camera_header = read_header("MDLCamera");
    assert_contains_all(&camera_header, &["@interface MDLStereoscopicCamera"]);

    let light_header = read_header("MDLLight");
    assert_contains_all(
        &light_header,
        &["@interface MDLAreaLight", "@interface MDLPhotometricLight"],
    );

    let material_header = read_header("MDLMaterial");
    assert_contains_all(
        &material_header,
        &[
            "@interface MDLTextureFilter",
            "@interface MDLTextureSampler",
            "@interface MDLMaterialPropertyConnection",
            "@interface MDLMaterialPropertyNode",
            "@interface MDLMaterialPropertyGraph",
            "MDLMaterialTextureWrapMode",
            "MDLMaterialTextureFilterMode",
            "MDLMaterialMipMapFilterMode",
        ],
    );

    let object_header = read_header("MDLObject");
    assert_contains_all(&object_header, &["@interface MDLObjectContainer"]);

    let submesh_header = read_header("MDLSubmesh");
    assert_contains_all(&submesh_header, &["@interface MDLSubmeshTopology"]);

    let types_header = read_header("MDLTypes");
    assert_contains_all(
        &types_header,
        &[
            "@protocol MDLComponent <NSObject>",
            "@protocol MDLNamed",
            "@protocol MDLObjectContainerComponent <MDLComponent, NSFastEnumeration>",
            "kUTTypeAlembic",
            "kUTType3dObject",
            "kUTTypePolygon",
            "kUTTypeStereolithography",
            "kUTTypeUniversalSceneDescription",
            "kUTTypeUniversalSceneDescriptionMobile",
        ],
    );

    let utility_header = read_header("MDLUtility");
    assert_contains_all(&utility_header, &["@interface MDLUtility"]);

    let value_types_header = read_header("MDLValueTypes");
    assert_contains_all(&value_types_header, &["@interface MDLMatrix4x4Array"]);

    let vertex_header = read_header("MDLVertexDescriptor");
    assert_contains_all(
        &vertex_header,
        &[
            "@interface MDLVertexBufferLayout",
            "MDLVertexAttributePosition",
            "MDLVertexAttributeNormal",
            "MDLVertexAttributeTextureCoordinate",
        ],
    );

    let bridge = read_bridge();
    assert_contains_all(
        &bridge,
        &[
            "mdl_material_property_new",
            "mdl_named_name_string",
            "mdl_texture_filter_new",
            "mdl_texture_sampler_new",
            "mdl_material_property_connection_new",
            "mdl_material_property_node_new",
            "mdl_material_property_graph_new",
            "mdl_object_container_new",
            "mdl_submesh_topology_new",
            "mdl_sdk_constant_string",
            "mdl_matrix4x4_array_new",
            "mdl_stereoscopic_camera_new",
            "mdl_area_light_new",
            "mdl_photometric_light_new",
            "mdl_utility_convert_to_usdz",
        ],
    );
}
