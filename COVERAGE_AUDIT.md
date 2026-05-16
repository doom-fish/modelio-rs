# modelio-rs coverage audit (vs MacOSX26.2.sdk)

Method: unique top-level ModelIO public classes/protocols/enums/structs/constants from the macOS 26.2 SDK headers, excluding no top-level macOS-unavailable symbols and finding no top-level deprecated symbols to exempt.

SDK_PUBLIC_SYMBOLS: 117
VERIFIED: 45
GAPS: 72
EXEMPT: 0
COVERAGE_PCT: 38.5%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MDLAnimatedValueInterpolation` | enum | `MDLAnimatedValueTypes.h` | AnimatedValueInterpolation (`src/types.rs`) |
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
| `MDLCameraProjection` | enum | `MDLCamera.h` | CameraProjection (`src/types.rs`) |
| `MDLCamera` | class | `MDLCamera.h` | Camera (`src/camera.rs`) |
| `MDLLightType` | enum | `MDLLight.h` | LightType (`src/types.rs`) |
| `MDLLight` | class | `MDLLight.h` | Light (`src/light.rs`) |
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
| `MDLObject` | class | `MDLObject.h` | Object (`src/object.rs`) |
| `MDLSubmesh` | class | `MDLSubmesh.h` | Submesh (`src/submesh.rs`) |
| `MDLTextureChannelEncoding` | enum | `MDLTexture.h` | TextureChannelEncoding (`src/types.rs`) |
| `MDLCheckerboardTexture` | class | `MDLTexture.h` | Texture::new_checkerboard (`src/texture.rs`) |
| `MDLTexture` | class | `MDLTexture.h` | Texture (`src/texture.rs`) |
| `MDLURLTexture` | class | `MDLTexture.h` | Texture::from_url (`src/texture.rs`) |
| `MDLDataPrecision` | enum | `MDLTypes.h` | DataPrecision (`src/types.rs`) |
| `MDLGeometryType` | enum | `MDLTypes.h` | GeometryType (`src/types.rs`) |
| `MDLIndexBitDepth` | enum | `MDLTypes.h` | IndexBitDepth (`src/types.rs`) |
| `MDLAxisAlignedBoundingBox` | struct | `MDLTypes.h` | BoundingBox (`src/types.rs`) |
| `MDLVertexFormat` | enum | `MDLVertexDescriptor.h` | vertex_format module (`src/types.rs`) |
| `MDLVertexAttribute` | class | `MDLVertexDescriptor.h` | VertexAttribute (`src/vertex_attribute.rs`) |
| `MDLVertexDescriptor` | class | `MDLVertexDescriptor.h` | VertexDescriptor (`src/vertex_attribute.rs`) |
| `MDLVoxelArray` | class | `MDLVoxelArray.h` | VoxelArray (`src/voxel_array.rs`) |
| `MDLVoxelIndexExtent` | struct | `MDLVoxelArray.h` | VoxelIndexExtent (`src/types.rs`) |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `MDLAnimatedValue` | class | `MDLAnimatedValueTypes.h` | Concrete animated wrappers exist, but the abstract base class is not exposed. |
| `MDLJointAnimation` | protocol | `MDLAnimation.h` | Only the concrete PackedJointAnimation class is wrapped; the protocol is not. |
| `MDLLightProbeIrradianceDataSource` | protocol | `MDLAsset.h` | No callback/protocol bridge for light-probe irradiance baking. |
| `MDLBundleAssetResolver` | class | `MDLAssetResolver.h` | No asset-resolver protocol/class bridge. |
| `MDLPathAssetResolver` | class | `MDLAssetResolver.h` | No asset-resolver protocol/class bridge. |
| `MDLRelativeAssetResolver` | class | `MDLAssetResolver.h` | No asset-resolver protocol/class bridge. |
| `MDLAssetResolver` | protocol | `MDLAssetResolver.h` | No asset-resolver protocol/class bridge. |
| `MDLStereoscopicCamera` | class | `MDLCamera.h` | No stereoscopic camera wrapper. |
| `MDLAreaLight` | class | `MDLLight.h` | Only Light and PhysicallyPlausibleLight are wrapped. |
| `MDLLightProbe` | class | `MDLLight.h` | Only Light and PhysicallyPlausibleLight are wrapped. |
| `MDLPhotometricLight` | class | `MDLLight.h` | Only Light and PhysicallyPlausibleLight are wrapped. |
| `MDLMaterialMipMapFilterMode` | enum | `MDLMaterial.h` | Texture-sampler filter/wrap enums are not re-exported. |
| `MDLMaterialTextureFilterMode` | enum | `MDLMaterial.h` | Texture-sampler filter/wrap enums are not re-exported. |
| `MDLMaterialTextureWrapMode` | enum | `MDLMaterial.h` | Texture-sampler filter/wrap enums are not re-exported. |
| `MDLMaterialPropertyConnection` | class | `MDLMaterial.h` | Material graph authoring is not wrapped. |
| `MDLMaterialPropertyGraph` | class | `MDLMaterial.h` | Material graph authoring is not wrapped. |
| `MDLMaterialPropertyNode` | class | `MDLMaterial.h` | Material graph authoring is not wrapped. |
| `MDLTextureFilter` | class | `MDLMaterial.h` | Texture sampler/filter objects are not wrapped. |
| `MDLTextureSampler` | class | `MDLMaterial.h` | Texture sampler/filter objects are not wrapped. |
| `MDLMeshBufferData` | class | `MDLMeshBuffer.h` | Mesh buffers are exposed read-only; authoring support types are not wrapped. |
| `MDLMeshBufferDataAllocator` | class | `MDLMeshBuffer.h` | Mesh buffers are exposed read-only; authoring support types are not wrapped. |
| `MDLMeshBufferMap` | class | `MDLMeshBuffer.h` | Mesh buffers are exposed read-only; authoring support types are not wrapped. |
| `MDLMeshBufferZoneDefault` | class | `MDLMeshBuffer.h` | Mesh buffers are exposed read-only; authoring support types are not wrapped. |
| `MDLMeshBufferAllocator` | protocol | `MDLMeshBuffer.h` | No allocator/zone protocol bridge for mesh-buffer authoring. |
| `MDLMeshBufferZone` | protocol | `MDLMeshBuffer.h` | No allocator/zone protocol bridge for mesh-buffer authoring. |
| `MDLObjectContainer` | class | `MDLObject.h` | Object child helpers exist, but the default container class is not exposed. |
| `MDLSubmeshTopology` | class | `MDLSubmesh.h` | No topology helper wrapper. |
| `MDLColorSwatchTexture` | class | `MDLTexture.h` | Only URL and checkerboard texture subclasses are wrapped. |
| `MDLNoiseTexture` | class | `MDLTexture.h` | Only URL and checkerboard texture subclasses are wrapped. |
| `MDLNormalMapTexture` | class | `MDLTexture.h` | Only URL and checkerboard texture subclasses are wrapped. |
| `MDLSkyCubeTexture` | class | `MDLTexture.h` | Only URL and checkerboard texture subclasses are wrapped. |
| `MDLTransform` | class | `MDLTransform.h` | No transform object wrapper. |
| `MDLTransformComponent` | protocol | `MDLTransform.h` | Object/Camera transform components are not surfaced as a public wrapper. |
| `MDLTransformOpRotationOrder` | enum | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `MDLTransformMatrixOp` | class | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `MDLTransformOrientOp` | class | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `MDLTransformRotateOp` | class | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `MDLTransformRotateXOp` | class | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `MDLTransformRotateYOp` | class | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `MDLTransformRotateZOp` | class | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `MDLTransformScaleOp` | class | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `MDLTransformStack` | class | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `MDLTransformTranslateOp` | class | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `MDLTransformOp` | protocol | `MDLTransformStack.h` | Transform-stack authoring APIs are not wrapped. |
| `kUTType3dObject` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `kUTTypeAlembic` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `kUTTypePolygon` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `kUTTypeStereolithography` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `kUTTypeUniversalSceneDescription` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `kUTTypeUniversalSceneDescriptionMobile` | constant | `MDLTypes.h` | UTType constant is not re-exported. |
| `MDLProbePlacement` | enum | `MDLTypes.h` | Probe-placement enum is unused because light-probe APIs are not wrapped. |
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
