# ModelIO coverage audit

Crate: `modelio-rs`  
Version target: `0.2.0`

Legend:

- ✅ implemented
- 🟡 partial — surface exists but some header APIs remain unwrapped
- ⏭️ skipped — not wrapped in this release

## Core logical areas

| Header | API | Status | Notes |
| --- | --- | --- | --- |
| `MDLAsset.h` | `MDLAsset` construction, import/export predicates, export, count, object access, add/remove object, time range, up axis, bounding boxes, URL, texture loading | ✅ | Safe wrapper: `src/asset.rs`; bridge: `swift-bridge/Sources/ModelIOBridge/Asset.swift` |
| `MDLMesh.h` | Primitive generators (box/ellipsoid/sphere/cylinder/plane/icosahedron), vertex counts, vertex buffers, submeshes, bounding box, vertex descriptor, attribute-data access | 🟡 | Mesh modifiers, cone/capsule/subdivision generators, light-baking helpers remain deferred |
| `MDLMaterial.h` | `MDLMaterial` creation, `materialFace`, property lookup, property mutation for float/vector/matrix/string/url/color/luminance, texture extraction | 🟡 | Material graphs, explicit property constructors, texture-sampler mutation, resolver-driven texture loading remain deferred |
| `MDLLight.h` | `MDLLight` creation, light type/color space, irradiance sampling | ✅ | Wrapped in `src/light.rs` and `Light.swift` |
| `MDLLight.h` | `MDLPhysicallyPlausibleLight` creation, color temperature, color, lumens, cone angles, attenuation distances | ✅ | Wrapped in `src/physically_plausible_light.rs` and `PhysicallyPlausibleLight.swift` |
| `MDLCamera.h` | `MDLCamera` creation, projection, basic optical properties, `lookAt`, `frameBoundingBox`, `rayTo`, bokeh kernel texture | �� | Stereoscopic camera APIs and many less-common optical knobs remain deferred |
| `MDLObject.h` | `MDLObject` creation, name/path/hidden, child hierarchy, path lookup, bounding box, kind inspection | 🟡 | Component protocol plumbing and transform authoring remain deferred |
| `MDLVoxelArray.h` | Construction from voxel indices, count, existence queries, voxel enumeration, boolean ops, signed-shell conversion, coarse/smooth mesh creation, spatial queries | 🟡 | Asset/mesh voxelization initializers and deprecated narrow-band/shell overloads remain deferred |
| `MDLTexture.h` | URL textures, checkerboard textures, metadata, texel reads, write-to-URL | 🟡 | Cube/irradiance/sky/noise/normal-map texture factories remain deferred |
| `MDLAnimation.h` | `MDLPackedJointAnimation`, `MDLAnimationBindComponent` creation, translation/rotation/scale access, skeleton binding, joint-path mutation | ✅ | Wrapped in `src/animation.rs` and `Animation.swift` |
| `MDLAnimatedValueTypes.h` | Scalar/vector/quaternion/matrix animated value creation, interpolation, clear, per-time sampling, scalar/vector3/quaternion arrays | 🟡 | Double-precision APIs and array reset helpers remain deferred |
| `MDLSubmesh.h` | Index counts/types, geometry type, name, index buffer conversion, material assignment | 🟡 | Topology surfaces and reindexing constructors remain deferred |
| `MDLVertexDescriptor.h` | `MDLVertexAttribute` creation/mutation, `MDLVertexDescriptor` copy/info/lookup/reset/packed layout helpers | ✅ | Default attribute name constants are surfaced as raw strings via SDK rather than dedicated Rust constants |
| `MDLAnimation.h` | `MDLSkeleton` creation, joint paths, bind/rest transform extraction | ✅ | Wrapped in `src/skeleton.rs` and `Skeleton.swift` |

## Additional headers and deferred areas

| Header | API | Status | Reason |
| --- | --- | --- | --- |
| `MDLAssetResolver.h` | Asset-resolver protocols and relative resolver types | ⏭️ skipped | Requires resolver object callbacks or custom protocol implementations across the bridge |
| `MDLLight.h` | `MDLAreaLight`, `MDLPhotometricLight`, `MDLLightProbe`, light-probe baking helpers | ⏭️ skipped | Heavier asset/light baking workflows and texture/probe helpers are outside this release |
| `MDLCamera.h` | `MDLStereoscopicCamera` | ⏭️ skipped | Specialized stereoscopic camera surface not required for v0.2.0 |
| `MDLMaterial.h` | `MDLMaterialPropertyConnection`, `MDLMaterialPropertyNode`, `MDLMaterialPropertyGraph`, scattering-function subclasses | ⏭️ skipped | Material graph authoring and graph evaluation need a larger typed surface |
| `MDLMesh.h` | Mesh ambient-occlusion/light-map baking helpers | ⏭️ skipped | Bake routines are environment-sensitive and not suitable for the headless smoke suite |
| `MDLMeshBuffer.h` | Mesh-buffer allocators/zones/raw mutable buffer creation | ⏭️ skipped | Existing crate exposes safe read access but not allocator authoring |
| `MDLTransform.h` / `MDLTransformStack.h` | Full transform component APIs | ⏭️ skipped | Current object/camera wrappers use ModelIO defaults without exposing transform authoring yet |
| `MDLUtility.h`, `MDLValueTypes.h` | Miscellaneous utilities and typed value-array helpers beyond skeleton extraction | ⏭️ skipped | Deferred until a broader math/value-helpers pass |
| Deprecated voxel constructors | Narrow-band / shell overloads deprecated in macOS 10.12 | ⏭️ skipped | Deferred in favor of current non-deprecated initializer |

## Files added or extended by logical area

| Area | Swift bridge | Rust module | Example | Test |
| --- | --- | --- | --- | --- |
| `MDLAsset` | `Asset.swift` | `src/asset.rs` | `examples/02_asset_basics.rs` | `tests/asset_tests.rs` |
| `MDLMesh` | `Mesh.swift` | `src/mesh.rs` | `examples/01_primitive_smoke.rs` | `tests/mesh_tests.rs` |
| `MDLMaterial` | `Material.swift` | `src/material.rs` | `examples/03_material_properties.rs` | `tests/material_tests.rs` |
| `MDLLight` | `Light.swift` | `src/light.rs` | `examples/04_light_defaults.rs` | `tests/light_tests.rs` |
| `MDLPhysicallyPlausibleLight` | `PhysicallyPlausibleLight.swift` | `src/physically_plausible_light.rs` | `examples/05_physically_plausible_light.rs` | `tests/physically_plausible_light_tests.rs` |
| `MDLCamera` | `Camera.swift` | `src/camera.rs` | `examples/06_camera_controls.rs` | `tests/camera_tests.rs` |
| `MDLObject` | `Object.swift` | `src/object.rs` | `examples/07_object_hierarchy.rs` | `tests/object_tests.rs` |
| `MDLVoxelArray` | `VoxelArray.swift` | `src/voxel_array.rs` | `examples/08_voxel_array_boolean.rs` | `tests/voxel_array_tests.rs` |
| `MDLTexture` | `Texture.swift` | `src/texture.rs` | `examples/09_texture_checkerboard.rs` | `tests/texture_tests.rs` |
| `MDLAnimation` | `Animation.swift` | `src/animation.rs` | `examples/10_animation_bind_component.rs` | `tests/animation_tests.rs` |
| `MDLAnimatedValueTypes` | `AnimatedValueTypes.swift` | `src/animated_value_types.rs` | `examples/11_animated_value_types.rs` | `tests/animated_value_types_tests.rs` |
| `MDLSubmesh` | `Submesh.swift` | `src/submesh.rs` | `examples/12_submesh_material.rs` | `tests/submesh_tests.rs` |
| `MDLVertexAttribute` | `VertexAttribute.swift` | `src/vertex_attribute.rs` | `examples/13_vertex_attribute_descriptor.rs` | `tests/vertex_attribute_tests.rs` |
| `MDLSkeleton` | `Skeleton.swift` | `src/skeleton.rs` | `examples/14_skeleton_basics.rs` | `tests/skeleton_tests.rs` |
