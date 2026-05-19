# ModelIO coverage

Crate: `modelio-rs`  
Version target: `0.3.0`

This document tracks **top-level ModelIO public symbol coverage** against the active macOS SDK: public classes, protocols, enums, structs, and exported SDK constants. `v0.3.0` reaches **117 / 117 verified symbols (100%)** in that audit.

> Status in this document refers to top-level symbol coverage, not every Objective-C convenience selector. The Rust API intentionally wraps the safe, high-value ModelIO surface rather than mirroring every overload 1:1.

## Summary

| Metric | Value |
| --- | --- |
| SDK public symbols audited | 117 |
| Verified wrappers | 117 |
| Remaining gaps | 0 |
| Coverage | 100% |

## Covered logical areas

| Header / area | Status | Notes |
| --- | --- | --- |
| `MDLAsset.h` | ✅ | `Asset` loading/export, object traversal, timing metadata, and light-probe placement |
| `MDLAssetResolver.h` | ✅ | Callback-backed generic resolver surface plus path, bundle, and relative resolvers |
| `MDLMesh.h` + `MDLMeshBuffer.h` | ✅ | Primitive meshes, vertex buffers, callback-backed mesh-buffer allocators/zones/maps, and vertex-attribute data |
| `MDLMaterial.h` | ✅ | `Material`, `ScatteringFunction`, `PhysicallyPlausibleScatteringFunction`, standalone `MaterialProperty` creation, sampler/filter wrappers, and material-graph types |
| `MDLLight.h` | ✅ | `Light`, `LightProbe`, `PhysicallyPlausibleLight`, `AreaLight`, and `PhotometricLight` |
| `MDLCamera.h` | ✅ | `Camera` and `StereoscopicCamera` |
| `MDLObject.h` | ✅ | `Object`, hierarchy traversal, component attachment, and `ObjectContainer` |
| `MDLTransform.h` + `MDLTransformStack.h` | ✅ | `Transform`, callback-backed `TransformComponent`/`TransformOp`, `TransformStack`, and typed transform ops |
| `MDLVoxelArray.h` | ✅ | Sparse voxel grids, asset/mesh voxelization, boolean ops, and allocator-aware mesh extraction |
| `MDLTexture.h` | ✅ | URL textures, checkerboards, gradients, noise, normal maps, sky cubes, metadata, writes, texel reads |
| `MDLAnimation.h` + `MDLAnimatedValueTypes.h` | ✅ | Skeletons, packed joint animation, bind components, animated scalars/vectors/quaternions/matrices, and protocol markers |
| `MDLSubmesh.h` + `MDLVertexDescriptor.h` | ✅ | `Submesh`, `SubmeshTopology`, `VertexAttribute`, `VertexDescriptor`, and `VertexBufferLayout` |
| `MDLValueTypes.h` + `MDLUtility.h` + SDK constants | ✅ | `Matrix4x4Array`, `Utility`, `ut_type::*`, and `vertex_attribute_name::*` |

## New in v0.3.0

- Added callback-backed `AssetResolver`, `MeshBufferAllocator`, `TransformComponent`, and `TransformOp` protocol wrappers
- Added first-class `ScatteringFunction` and `PhysicallyPlausibleScatteringFunction` wrappers plus material accessors/constructors for scattering functions
- Added integration coverage for custom resolvers, custom mesh-buffer allocators, scattering functions, and callback-backed transform components/ops

## New in v0.2.2

- Closed the 41-symbol gap list from `COVERAGE_AUDIT.md`
- Added material sampler/filter and graph wrappers
- Added stereoscopic camera, area light, photometric light, object container, submesh topology, and vertex-buffer layout wrappers
- Added `Matrix4x4Array`, SDK constant helpers, and `Utility::convert_to_usdz`
- Added asset/mesh voxelization helpers and allocator-aware mesh extraction

## Validation

Coverage is validated by:

- `tests/api_coverage.rs` against the active macOS SDK headers
- Integration smoke tests for materials, resolvers, mesh buffers, transforms, cameras, lights, voxels, skeletons, constants, and utility helpers
