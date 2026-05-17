# ModelIO coverage

Crate: `modelio-rs`  
Version target: `0.2.2`

This document tracks **top-level ModelIO public symbol coverage** against the active macOS SDK: public classes, protocols, enums, structs, and exported SDK constants. v0.2.2 reaches **117 / 117 verified symbols (100%)** in that audit.

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
| `MDLAssetResolver.h` | ✅ | Generic resolver surface plus path, bundle, and relative resolvers |
| `MDLMesh.h` + `MDLMeshBuffer.h` | ✅ | Primitive meshes, vertex buffers, mesh-buffer allocators/zones/maps, and vertex-attribute data |
| `MDLMaterial.h` | ✅ | `Material`, standalone `MaterialProperty` creation, sampler/filter wrappers, and material-graph types |
| `MDLLight.h` | ✅ | `Light`, `LightProbe`, `PhysicallyPlausibleLight`, `AreaLight`, and `PhotometricLight` |
| `MDLCamera.h` | ✅ | `Camera` and `StereoscopicCamera` |
| `MDLObject.h` | ✅ | `Object`, hierarchy traversal, component attachment, and `ObjectContainer` |
| `MDLTransform.h` + `MDLTransformStack.h` | ✅ | `Transform`, `TransformComponent`, `TransformStack`, and typed transform ops |
| `MDLVoxelArray.h` | ✅ | Sparse voxel grids, asset/mesh voxelization, boolean ops, and allocator-aware mesh extraction |
| `MDLTexture.h` | ✅ | URL textures, checkerboards, gradients, noise, normal maps, sky cubes, metadata, writes, texel reads |
| `MDLAnimation.h` + `MDLAnimatedValueTypes.h` | ✅ | Skeletons, packed joint animation, bind components, animated scalars/vectors/quaternions/matrices, and protocol markers |
| `MDLSubmesh.h` + `MDLVertexDescriptor.h` | ✅ | `Submesh`, `SubmeshTopology`, `VertexAttribute`, `VertexDescriptor`, and `VertexBufferLayout` |
| `MDLValueTypes.h` + `MDLUtility.h` + SDK constants | ✅ | `Matrix4x4Array`, `Utility`, `ut_type::*`, and `vertex_attribute_name::*` |

## New in v0.2.2

- Closed the 41-symbol gap list from `COVERAGE_AUDIT.md`
- Added material sampler/filter and graph wrappers
- Added stereoscopic camera, area light, photometric light, object container, submesh topology, and vertex-buffer layout wrappers
- Added `Matrix4x4Array`, SDK constant helpers, and `Utility::convert_to_usdz`
- Added asset/mesh voxelization helpers and allocator-aware mesh extraction

## Validation

Coverage is validated by:

- `tests/api_coverage.rs` against the active macOS SDK headers
- Integration smoke tests for materials, cameras, lights, voxels, skeletons, constants, and utility helpers
