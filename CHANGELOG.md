# Changelog

## [0.2.0] - 2026-05-16

### Added

- Added `Object`, `Light`, `PhysicallyPlausibleLight`, `Camera`, `VoxelArray`, `Skeleton`, `PackedJointAnimation`, `AnimationBindComponent`, animated value wrappers, `VertexDescriptor`, and `VertexAttribute` safe Rust APIs.
- Extended `Asset`, `Material`, `Texture`, `Mesh`, and `Submesh` with export, hierarchy, material-mutation, checkerboard-texture, vertex-descriptor, and submesh-material surfaces.
- Split the Swift bridge into logical files per ModelIO area and expanded the C ABI to cover scene objects, cameras, lights, voxels, animation, and vertex descriptors.
- Added fixture-backed examples and integration tests for every logical area plus an expanded SDK header audit.
- Added `COVERAGE.md` documenting the current framework coverage and deferred ModelIO areas.

## [0.1.0] - 2026-05-16

### Added

- Initial `modelio-rs` release for macOS asset loading and procedural mesh generation.
- `Asset`, `Mesh`, `Submesh`, `MeshBuffer`, `VertexAttributeData`, `Material`, `MaterialProperty`, and `Texture` wrappers.
- Swift bridge for `ModelIO.framework` object ownership, mesh generation, material inspection, and texture extraction.
- Safe Rust enums and constants for ModelIO geometry kinds, material semantics, texture channel encodings, and vertex formats.
- Smoke example `examples/01_primitive_smoke.rs` covering procedural box generation and buffer inspection.
- Header-audit test `tests/api_coverage.rs` validating the targeted v0.1 surface against the active SDK.
