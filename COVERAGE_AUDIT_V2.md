# modelio-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 98
VERIFIED: 98
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100

Audit methodology: Enumerated SDK public surface by parsing all Objective-C headers in ModelIO.framework/Headers/ (23 header files). Extracted all @interface classes, @protocol definitions, typedef NS_ENUM/NS_OPTIONS enums, and FOUNDATION_EXPORT/MDL_EXPORT constants. Cross-referenced each symbol against the crate's Rust wrapper implementations in src/ and swift-bridge/Sources/. All 66 classes, 11 protocols, 17 enums, and 4 constant groups are fully wrapped and exposed via the crate's public API.

## 🟢 VERIFIED

| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| MDLAnimatedValue | class | MDLAnimatedValueTypes.h | AnimatedValue (src/animated_value_types.rs) |
| MDLAnimatedScalar | class | MDLAnimatedValueTypes.h | AnimatedScalar (src/animated_value_types.rs) |
| MDLAnimatedScalarArray | class | MDLAnimatedValueTypes.h | AnimatedScalarArray (src/animated_value_types.rs) |
| MDLAnimatedVector2 | class | MDLAnimatedValueTypes.h | AnimatedVector2 (src/animated_value_types.rs) |
| MDLAnimatedVector3 | class | MDLAnimatedValueTypes.h | AnimatedVector3 (src/animated_value_types.rs) |
| MDLAnimatedVector3Array | class | MDLAnimatedValueTypes.h | AnimatedVector3Array (src/animated_value_types.rs) |
| MDLAnimatedVector4 | class | MDLAnimatedValueTypes.h | AnimatedVector4 (src/animated_value_types.rs) |
| MDLAnimatedQuaternion | class | MDLAnimatedValueTypes.h | AnimatedQuaternion (src/animated_value_types.rs) |
| MDLAnimatedQuaternionArray | class | MDLAnimatedValueTypes.h | AnimatedQuaternionArray (src/animated_value_types.rs) |
| MDLAnimatedMatrix4x4 | class | MDLAnimatedValueTypes.h | AnimatedMatrix4x4 (src/animated_value_types.rs) |
| MDLSkeleton | class | MDLAnimation.h | Skeleton (src/skeleton.rs) |
| MDLPackedJointAnimation | class | MDLAnimation.h | PackedJointAnimation (src/animation.rs) |
| MDLAnimationBindComponent | class | MDLAnimation.h | AnimationBindComponent (src/animation.rs) |
| MDLJointAnimation | protocol | MDLAnimation.h | JointAnimation (src/protocols.rs) |
| MDLAsset | class | MDLAsset.h | Asset (src/asset.rs) |
| MDLLightProbeIrradianceDataSource | protocol | MDLAsset.h | LightProbeIrradianceDataSource (src/light_probe.rs) |
| MDLAssetResolver | protocol | MDLAssetResolver.h | AssetResolver (src/asset_resolver.rs) |
| MDLRelativeAssetResolver | class | MDLAssetResolver.h | RelativeAssetResolver (src/asset_resolver.rs) |
| MDLPathAssetResolver | class | MDLAssetResolver.h | PathAssetResolver (src/asset_resolver.rs) |
| MDLBundleAssetResolver | class | MDLAssetResolver.h | BundleAssetResolver (src/asset_resolver.rs) |
| MDLCamera | class | MDLCamera.h | Camera (src/camera.rs) |
| MDLCameraProjection | enum | MDLCamera.h | CameraProjection (src/types.rs) |
| MDLStereoscopicCamera | class | MDLCamera.h | StereoscopicCamera (src/camera.rs) |
| MDLLight | class | MDLLight.h | Light (src/light.rs) |
| MDLLightType | enum | MDLLight.h | LightType (src/types.rs) |
| MDLPhysicallyPlausibleLight | class | MDLLight.h | PhysicallyPlausibleLight (src/physically_plausible_light.rs) |
| MDLAreaLight | class | MDLLight.h | AreaLight (src/physically_plausible_light.rs) |
| MDLPhotometricLight | class | MDLLight.h | PhotometricLight (src/physically_plausible_light.rs) |
| MDLLightProbe | class | MDLLight.h | LightProbe (src/light_probe.rs) |
| MDLMaterial | class | MDLMaterial.h | Material (src/material.rs) |
| MDLMaterialSemantic | enum | MDLMaterial.h | MaterialSemantic (src/types.rs) |
| MDLMaterialPropertyType | enum | MDLMaterial.h | MaterialPropertyType (src/types.rs) |
| MDLMaterialTextureWrapMode | enum | MDLMaterial.h | MaterialTextureWrapMode (src/types.rs) |
| MDLMaterialTextureFilterMode | enum | MDLMaterial.h | MaterialTextureFilterMode (src/types.rs) |
| MDLMaterialMipMapFilterMode | enum | MDLMaterial.h | MaterialMipMapFilterMode (src/types.rs) |
| MDLMaterialFace | enum | MDLMaterial.h | MaterialFace (src/types.rs) |
| MDLTextureFilter | class | MDLMaterial.h | TextureFilter (src/material.rs) |
| MDLTextureSampler | class | MDLMaterial.h | TextureSampler (src/material.rs) |
| MDLMaterialProperty | class | MDLMaterial.h | MaterialProperty (src/material.rs) |
| MDLMaterialPropertyConnection | class | MDLMaterial.h | MaterialPropertyConnection (src/material.rs) |
| MDLMaterialPropertyNode | class | MDLMaterial.h | MaterialPropertyNode (src/material.rs) |
| MDLMaterialPropertyGraph | class | MDLMaterial.h | MaterialPropertyGraph (src/material.rs) |
| MDLScatteringFunction | class | MDLMaterial.h | ScatteringFunction (src/material.rs) |
| MDLPhysicallyPlausibleScatteringFunction | class | MDLMaterial.h | PhysicallyPlausibleScatteringFunction (src/material.rs) |
| MDLMesh | class | MDLMesh.h | Mesh (src/mesh.rs) |
| MDLVertexAttributeData | class | MDLMesh.h | VertexAttributeData (src/mesh.rs) |
| MDLMeshBuffer | protocol | MDLMeshBuffer.h | MeshBuffer (src/protocols.rs) |
| MDLMeshBufferType | enum | MDLMeshBuffer.h | MeshBufferType (src/types.rs) |
| MDLMeshBufferMap | class | MDLMeshBuffer.h | MeshBufferMap (src/mesh_buffer.rs) |
| MDLMeshBufferData | class | MDLMeshBuffer.h | MeshBufferData (src/mesh_buffer.rs) |
| MDLMeshBufferZone | protocol | MDLMeshBuffer.h | MeshBufferZone (src/protocols.rs) |
| MDLMeshBufferZoneDefault | class | MDLMeshBuffer.h | MeshBufferZoneDefault (src/mesh_buffer.rs) |
| MDLMeshBufferAllocator | protocol | MDLMeshBuffer.h | MeshBufferAllocator (src/protocols.rs) |
| MDLMeshBufferDataAllocator | class | MDLMeshBuffer.h | MeshBufferDataAllocator (src/mesh_buffer.rs) |
| MDLObject | class | MDLObject.h | Object (src/object.rs) |
| MDLNamed | protocol | MDLObject.h | Named (src/protocols.rs) |
| MDLComponent | protocol | MDLObject.h | Component (src/protocols.rs) |
| MDLObjectContainerComponent | protocol | MDLObject.h | ObjectContainerComponent (src/protocols.rs) |
| MDLObjectContainer | class | MDLObject.h | ObjectContainer (src/object.rs) |
| MDLSubmesh | class | MDLSubmesh.h | Submesh (src/submesh.rs) |
| MDLSubmeshTopology | class | MDLSubmesh.h | SubmeshTopology (src/submesh.rs) |
| MDLTexture | class | MDLTexture.h | Texture (src/texture.rs) |
| MDLTextureChannelEncoding | enum | MDLTexture.h | TextureChannelEncoding (src/types.rs) |
| MDLURLTexture | class | MDLTexture.h | URLTexture (src/texture.rs) |
| MDLCheckerboardTexture | class | MDLTexture.h | CheckerboardTexture (src/texture.rs) |
| MDLSkyCubeTexture | class | MDLTexture.h | SkyCubeTexture (src/texture.rs) |
| MDLColorSwatchTexture | class | MDLTexture.h | ColorSwatchTexture (src/texture.rs) |
| MDLNoiseTexture | class | MDLTexture.h | NoiseTexture (src/texture.rs) |
| MDLNormalMapTexture | class | MDLTexture.h | NormalMapTexture (src/texture.rs) |
| MDLTransform | class | MDLTransform.h | Transform (src/transform.rs) |
| MDLTransformComponent | protocol | MDLTransform.h | TransformComponent (src/protocols.rs) |
| MDLTransformStack | class | MDLTransformStack.h | TransformStack (src/transform.rs) |
| MDLTransformOpRotationOrder | enum | MDLTransformStack.h | TransformOpRotationOrder (src/types.rs) |
| MDLTransformOp | protocol | MDLTransformStack.h | TransformOp (src/protocols.rs) |
| MDLTransformRotateXOp | class | MDLTransformStack.h | TransformRotateXOp (src/transform.rs) |
| MDLTransformRotateYOp | class | MDLTransformStack.h | TransformRotateYOp (src/transform.rs) |
| MDLTransformRotateZOp | class | MDLTransformStack.h | TransformRotateZOp (src/transform.rs) |
| MDLTransformRotateOp | class | MDLTransformStack.h | TransformRotateOp (src/transform.rs) |
| MDLTransformTranslateOp | class | MDLTransformStack.h | TransformTranslateOp (src/transform.rs) |
| MDLTransformScaleOp | class | MDLTransformStack.h | TransformScaleOp (src/transform.rs) |
| MDLTransformMatrixOp | class | MDLTransformStack.h | TransformMatrixOp (src/transform.rs) |
| MDLTransformOrientOp | class | MDLTransformStack.h | TransformOrientOp (src/transform.rs) |
| MDLVertexAttribute | class | MDLVertexDescriptor.h | VertexAttribute (src/vertex_attribute.rs) |
| MDLVertexDescriptor | class | MDLVertexDescriptor.h | VertexDescriptor (src/vertex_attribute.rs) |
| MDLVertexFormat | enum | MDLVertexDescriptor.h | VertexFormat (src/types.rs) |
| MDLVertexBufferLayout | class | MDLVertexDescriptor.h | VertexBufferLayout (src/vertex_attribute.rs) |
| MDLVoxelArray | class | MDLVoxelArray.h | VoxelArray (src/voxel_array.rs) |
| MDLGeometryType | enum | MDLVoxelArray.h | GeometryType (src/types.rs) |
| MDLProbePlacement | enum | MDLVoxelArray.h | ProbePlacement (src/types.rs) |
| MDLIndexBitDepth | enum | MDLVoxelArray.h | IndexBitDepth (src/types.rs) |
| MDLDataPrecision | enum | MDLVoxelArray.h | DataPrecision (src/types.rs) |
| MDLUtility | class | MDLUtility.h | Utility (src/utility.rs) |
| kUTTypeAlembic | constant | MDLTypes.h | ut_type::alembic() (src/sdk_constants.rs) |
| kUTType3dObject | constant | MDLTypes.h | ut_type::object_3d() (src/sdk_constants.rs) |
| kUTTypePolygon | constant | MDLTypes.h | ut_type::polygon() (src/sdk_constants.rs) |
| kUTTypeStereolithography | constant | MDLTypes.h | ut_type::stereolithography() (src/sdk_constants.rs) |
| kUTTypeUniversalSceneDescription | constant | MDLTypes.h | ut_type::universal_scene_description() (src/sdk_constants.rs) |
| kUTTypeUniversalSceneDescriptionMobile | constant | MDLTypes.h | ut_type::universal_scene_description_mobile() (src/sdk_constants.rs) |
| MDLVertexAttribute* (anisotropy, binormal, bitangent, color, edge_crease, joint_indices, joint_weights, normal, occlusion_value, position, shading_basis_u, shading_basis_v, subdivision_stencil, tangent, texture_coordinate) | constants (15) | MDLVertexDescriptor.h | vertex_attribute_name::*() (src/sdk_constants.rs) |
| MDLAnimatedValueInterpolation | enum | MDLAnimatedValueTypes.h | AnimatedValueInterpolation (src/types.rs) |
| MDLMatrix4x4Array | class | MDLAnimatedValueTypes.h | Matrix4x4Array (src/value_types.rs) |

## 🔴 GAPS

_None._

## ⏭️ EXEMPT

_None._
