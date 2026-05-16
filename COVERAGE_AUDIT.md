# modelio-rs coverage audit (vs MacOSX26.2.sdk)

Method: unique top-level ModelIO public classes/protocols/enums/structs/constants from the macOS 26.2 SDK headers, excluding no top-level macOS-unavailable symbols and finding no top-level deprecated symbols to exempt.

SDK_PUBLIC_SYMBOLS: 117
VERIFIED: 76
GAPS: 41
EXEMPT: 0
COVERAGE_PCT: 65.0%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MDLAnimatedValueInterpolation` | enum | `MDLAnimatedValueTypes.h` | AnimatedValueInterpolation (`src/types.rs`) |
| `MDLAnimatedValue` | class | `MDLAnimatedValueTypes.h` | AnimatedValue (`src/animated_value_types.rs`) |
| `MDLAnimatedMatrix4x4` | class | `MDLAnimatedValueTypes.h` | AnimatedMatrix4x4 (`src/animated_value_types.rs`) |
| `MDLAnimatedQuaternion` | class | `MDLAnimatedValueTypes.h` | AnimatedQuaternion (`src/animated_value_types.rs`) |
| `MDLAnimatedQuaternionArray` | class | `MDLAnimatedValueTypes.h` | AnimatedQuaternionArray (`src/animated_value_types.rs`) |
| `MDLAnimatedScalar` | class | `MDLAnimatedValueTypes.h` | AnimatedScalar (`src/animated_value_types.rs`) |
| `MDLAnimatedScalarArray` | class | `MDLAnimatedValueTypes.h` | AnimatedScalarArray (`src/animated_value_types.rs`) |
| `MDLAnimatedVector2` | class | `MDLAnimatedValueTypes.h` | AnimatedVector2 (`src/animated_value_types.rs`) |
| `MDLAnimatedVector3` | class | `MDLAnimatedValueTypes.h` | AnimatedVector3 (`src/animated_value_types.rs`) |
| `MDLAnimatedVector3Array` | class | `MDLAnimatedValueTypes.h` | AnimatedVector3Array (`src/animated_value_types.rs`) |
| `MDLAnimatedVector4` | class | `MDLAnimatedValueTypes.h` | AnimatedVector4 (`src/animated_value_types.rs`) |
| `MDLAnimationBindComponent` | class | `MDLAnimation.h` | AnimationBindComponent (`src/animation.rs`) |
| `MDLPackedJointAnimation` | class | `MDLAnimation.h` | PackedJointAnimation (`src/animation.rs`) |
| `MDLSkeleton` | class | `MDLAnimation.h` | Skeleton (`src/skeleton.rs`) |
| `MDLAsset` | class | `MDLAsset.h` | Asset (`src/asset.rs`) |
| `MDLLightProbeIrradianceDataSource` | protocol | `MDLAsset.h` | LightProbeIrradianceDataSource (`src/light_probe.rs`) |
| `MDLAssetResolver` | protocol | `MDLAssetResolver.h` | AssetResolver (`src/asset_resolver.rs`) |
| `MDLBundleAssetResolver` | class | `MDLAssetResolver.h` | BundleAssetResolver (`src/asset_resolver.rs`) |
| `MDLPathAssetResolver` | class | `MDLAssetResolver.h` | PathAssetResolver (`src/asset_resolver.rs`) |
| `MDLRelativeAssetResolver` | class | `MDLAssetResolver.h` | RelativeAssetResolver (`src/asset_resolver.rs`) |
| `MDLCameraProjection` | enum | `MDLCamera.h` | CameraProjection (`src/types.rs`) |
| `MDLCamera` | class | `MDLCamera.h` | Camera (`src/camera.rs`) |
| `MDLLightType` | enum | `MDLLight.h` | LightType (`src/types.rs`) |
| `MDLLight` | class | `MDLLight.h` | Light (`src/light.rs`) |
| `MDLLightProbe` | class | `MDLLight.h` | LightProbe (`src/light_probe.rs`) |
| `MDLPhysicallyPlausibleLight` | class | `MDLLight.h` | PhysicallyPlausibleLight (`src/physically_plausible_light.rs`) |
| `MDLMaterialFace` | enum | `MDLMaterial.h` | MaterialFace (`src/types.rs`) |
| `MDLMaterialPropertyType` | enum | `MDLMaterial.h` | MaterialPropertyType (`src/types.rs`) |
| `MDLMaterialSemantic` | enum | `MDLMaterial.h` | MaterialSemantic (`src/types.rs`) |
| `MDLMaterial` | class | `MDLMaterial.h` | Material (`src/material.rs`) |
| `MDLMaterialProperty` | class | `MDLMaterial.h` | MaterialProperty (`src/material.rs`) |
| `MDLPhysicallyPlausibleScatteringFunction` | class | `MDLMaterial.h` | Material::new(name, true) (`src/material.rs`, `Material.swift`) |
| `MDLScatteringFunction` | class | `MDLMaterial.h` | Material::new(name, false) (`src/material.rs`, `Material.swift`) |
| `MDLMesh` | class | `MDLMesh.h` | Mesh (`src/mesh.rs`) |
| `MDLVertexAttributeData` | class | `MDLMesh.h` | VertexAttributeData (`src/mesh.rs`) |
| `MDLMeshBufferType` | enum | `MDLMeshBuffer.h` | MeshBufferType (`src/types.rs`) |
| `MDLMeshBuffer` | protocol | `MDLMeshBuffer.h` | MeshBuffer (`src/mesh.rs`) |
| `MDLMeshBufferAllocator` | protocol | `MDLMeshBuffer.h` | MeshBufferAllocator (`src/mesh_buffer.rs`) |
| `MDLMeshBufferData` | class | `MDLMeshBuffer.h` | MeshBufferData (`src/mesh_buffer.rs`) |
| `MDLMeshBufferDataAllocator` | class | `MDLMeshBuffer.h` | MeshBufferDataAllocator (`src/mesh_buffer.rs`) |
| `MDLMeshBufferMap` | class | `MDLMeshBuffer.h` | MeshBufferMap (`src/mesh_buffer.rs`) |
| `MDLMeshBufferZone` | protocol | `MDLMeshBuffer.h` | MeshBufferZone (`src/mesh_buffer.rs`) |
| `MDLMeshBufferZoneDefault` | class | `MDLMeshBuffer.h` | MeshBufferZoneDefault (`src/mesh_buffer.rs`) |
| `MDLObject` | class | `MDLObject.h` | Object (`src/object.rs`) |
| `MDLSubmesh` | class | `MDLSubmesh.h` | Submesh (`src/submesh.rs`) |
| `MDLTextureChannelEncoding` | enum | `MDLTexture.h` | TextureChannelEncoding (`src/types.rs`) |
| `MDLCheckerboardTexture` | class | `MDLTexture.h` | Texture::new_checkerboard (`src/texture.rs`) |
| `MDLColorSwatchTexture` | class | `MDLTexture.h` | `Texture::new_color_temperature_gradient` / `Texture::new_color_gradient` (`src/texture.rs`) |
| `MDLNoiseTexture` | class | `MDLTexture.h` | `Texture::new_vector_noise` / `Texture::new_scalar_noise` / `Texture::new_cellular_noise` (`src/texture.rs`) |
| `MDLNormalMapTexture` | class | `MDLTexture.h` | `Texture::new_normal_map` (`src/texture.rs`) |
| `MDLSkyCubeTexture` | class | `MDLTexture.h` | `Texture::new_sky_cube` / `Texture::new_sky_cube_with_azimuth` (`src/texture.rs`) |
| `MDLTexture` | class | `MDLTexture.h` | Texture (`src/texture.rs`) |
| `MDLURLTexture` | class | `MDLTexture.h` | Texture::from_url (`src/texture.rs`) |
| `MDLTransform` | class | `MDLTransform.h` | Transform (`src/transform.rs`) |
| `MDLTransformComponent` | protocol | `MDLTransform.h` | TransformComponent (`src/transform.rs`) |
| `MDLDataPrecision` | enum | `MDLTypes.h` | DataPrecision (`src/types.rs`) |
| `MDLGeometryType` | enum | `MDLTypes.h` | GeometryType (`src/types.rs`) |
| `MDLIndexBitDepth` | enum | `MDLTypes.h` | IndexBitDepth (`src/types.rs`) |
| `MDLProbePlacement` | enum | `MDLTypes.h` | ProbePlacement (`src/types.rs`) |
| `MDLAxisAlignedBoundingBox` | struct | `MDLTypes.h` | BoundingBox (`src/types.rs`) |
| `MDLTransformOpRotationOrder` | enum | `MDLTransformStack.h` | TransformOpRotationOrder (`src/types.rs`) |
| `MDLTransformMatrixOp` | class | `MDLTransformStack.h` | TransformMatrixOp (`src/transform.rs`) |
| `MDLTransformOrientOp` | class | `MDLTransformStack.h` | TransformOrientOp (`src/transform.rs`) |
| `MDLTransformOp` | protocol | `MDLTransformStack.h` | TransformOp (`src/transform.rs`) |
| `MDLTransformRotateOp` | class | `MDLTransformStack.h` | TransformRotateOp (`src/transform.rs`) |
| `MDLTransformRotateXOp` | class | `MDLTransformStack.h` | TransformRotateXOp (`src/transform.rs`) |
| `MDLTransformRotateYOp` | class | `MDLTransformStack.h` | TransformRotateYOp (`src/transform.rs`) |
| `MDLTransformRotateZOp` | class | `MDLTransformStack.h` | TransformRotateZOp (`src/transform.rs`) |
| `MDLTransformScaleOp` | class | `MDLTransformStack.h` | TransformScaleOp (`src/transform.rs`) |
| `MDLTransformStack` | class | `MDLTransformStack.h` | TransformStack (`src/transform.rs`) |
| `MDLTransformTranslateOp` | class | `MDLTransformStack.h` | TransformTranslateOp (`src/transform.rs`) |
| `MDLVertexFormat` | enum | `MDLVertexDescriptor.h` | vertex_format module (`src/types.rs`) |
| `MDLVertexAttribute` | class | `MDLVertexDescriptor.h` | VertexAttribute (`src/vertex_attribute.rs`) |
| `MDLVertexDescriptor` | class | `MDLVertexDescriptor.h` | VertexDescriptor (`src/vertex_attribute.rs`) |
| `MDLVoxelArray` | class | `MDLVoxelArray.h` | VoxelArray (`src/voxel_array.rs`) |
| `MDLVoxelIndexExtent` | struct | `MDLVoxelArray.h` | VoxelIndexExtent (`src/types.rs`) |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `MDLJointAnimation` | protocol | `MDLAnimation.h` | Only the concrete PackedJointAnimation class is wrapped; the protocol is not. |
| `MDLStereoscopicCamera` | class | `MDLCamera.h` | No stereoscopic camera wrapper. |
| `MDLAreaLight` | class | `MDLLight.h` | Only Light, LightProbe, and PhysicallyPlausibleLight are wrapped. |
| `MDLPhotometricLight` | class | `MDLLight.h` | Only Light, LightProbe, and PhysicallyPlausibleLight are wrapped. |
| `MDLMaterialMipMapFilterMode` | enum | `MDLMaterial.h` | Texture-sampler filter/wrap enums are not re-exported. |
| `MDLMaterialTextureFilterMode` | enum | `MDLMaterial.h` | Texture-sampler filter/wrap enums are not re-exported. |
| `MDLMaterialTextureWrapMode` | enum | `MDLMaterial.h` | Texture-sampler filter/wrap enums are not re-exported. |
| `MDLMaterialPropertyConnection` | class | `MDLMaterial.h` | Material graph authoring is not wrapped. |
| `MDLMaterialPropertyGraph` | class | `MDLMaterial.h` | Material graph authoring is not wrapped. |
| `MDLMaterialPropertyNode` | class | `MDLMaterial.h` | Material graph authoring is not wrapped. |
| `MDLTextureFilter` | class | `MDLMaterial.h` | Texture sampler/filter objects are not wrapped. |
| `MDLTextureSampler` | class | `MDLMaterial.h` | Texture sampler/filter objects are not wrapped. |
| `MDLObjectContainer` | class | `MDLObject.h` | Object child helpers exist, but the default container class is not exposed. |
| `MDLSubmeshTopology` | class | `MDLSubmesh.h` | No topology helper wrapper. |
| `kUTType3dObject` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `kUTTypeAlembic` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `kUTTypePolygon` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `kUTTypeStereolithography` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `kUTTypeUniversalSceneDescription` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `kUTTypeUniversalSceneDescriptionMobile` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `MDLComponent` | protocol | `MDLTypes.h` | Shared protocol is not surfaced as a Rust trait/object. |
| `MDLNamed` | protocol | `MDLTypes.h` | Shared protocol is not surfaced as a Rust trait/object. |
| `MDLObjectContainerComponent` | protocol | `MDLTypes.h` | Object child helpers exist, but the container protocol is not exposed. |
| `MDLUtility` | class | `MDLUtility.h` | No wrapper for the USDZ conversion helper. |
| `MDLMatrix4x4Array` | class | `MDLValueTypes.h` | Skeleton APIs copy matrices into Rust Vec values; the array object is not wrapped. |
| `MDLVertexAttributeAnisotropy` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeBinormal` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeBitangent` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeColor` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeEdgeCrease` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeJointIndices` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeJointWeights` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeNormal` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeOcclusionValue` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributePosition` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeShadingBasisU` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeShadingBasisV` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeSubdivisionStencil` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeTangent` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexAttributeTextureCoordinate` | constant | `MDLVertexDescriptor.h` | Default vertex-attribute NSString constant is not re-exported. |
| `MDLVertexBufferLayout` | class | `MDLVertexDescriptor.h` | Descriptor layout details are exposed as info only; layout objects are not wrapped. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |

_None._
