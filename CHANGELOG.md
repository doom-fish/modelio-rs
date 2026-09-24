# Changelog

All notable changes to `modelio-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-09-24

### Security

- `VertexAttributeData::bytes` no longer reads past the end of the vertex
  buffer. It copied the whole buffer size starting at the attribute, so
  interleaved attributes over-read by their offset (12 and 24 bytes for the
  normals and texture coordinates of a position/normal/UV mesh). It now
  returns the bytes from the attribute's first element to the end of the
  buffer.
- `MeshBuffer::fill_data` rejects fills that go past the end of the buffer.
  ModelIO's `fillData:offset:` does not check and overran the heap.
- A buffer returned by a custom `MeshBufferAllocator` that is shorter than the
  requested length (or data) is replaced by one from the default allocator,
  so ModelIO can no longer write past it.
- The light-probe irradiance data source always hands ModelIO
  `(level + 1)² × 3` coefficients, whatever the closure returns.

### Fixed

- `Asset::from_url` returns ModelIO's load error for missing, corrupt or
  unsupported files instead of `Ok` with an empty asset.
- `Asset::object_at_path` no longer relies on retaining a nil object that the
  SDK declares non-null.
- Swift integer conversions no longer abort the process on large caller
  values: `Int32(channelCount)` in `Texture::new_checkerboard` and
  `Texture::new_scalar_noise`, and the offset, stride, length, capacity,
  element-count, spherical-harmonics-level and texture-size conversions.
  Constructors return an invalid-argument error; setters clamp.
- The irradiance closure runs once per sample instead of twice.
- A panic in the irradiance closure no longer unwinds into ModelIO.
- A zone passed to the allocator's from-zone callbacks is released instead
  of leaked when the buffer type is invalid.

### Changed

- **Breaking:** `MeshBuffer::fill_data` returns `Result<()>`.
- **Breaking:** `VertexAttribute::new` and `VertexAttribute::set_format` take
  a `VertexFormat`. The `vertex_format` constants are `VertexFormat` values;
  the `*_BITS` and `PACKED_BIT` masks stay `u32`.
- **Breaking (behaviour):** an irradiance closure that returns a number of
  coefficients other than `(level + 1)² × 3`, or panics, contributes zeros for
  that sample.
- Callback panics are contained by `doom_fish_utils::panic_safe`.
- `doom-fish-utils` requirement is `>=0.4.1, <0.5`; `rust-version` is 1.82.

### Added

- `VertexFormat` with `from_raw`, `as_raw`, `byte_size`, `component_count` and
  `is_packed`, plus `format_enum()` on `VertexAttributeInfo` and
  `VertexAttributeDescriptorInfo`.
- `VertexDescriptor::new`, `VertexDescriptor::add_or_replace_attribute` and
  `Mesh::set_vertex_descriptor`.
- `Asset::from_url_with_options` with the vertex descriptor, buffer allocator
  and preserve-topology options of ModelIO's error-reporting initializer.
- Descriptors given to `Mesh::set_vertex_descriptor` or
  `Asset::from_url_with_options` are validated first: every attribute with a
  format needs a known format and a layout whose stride fits
  `offset + format size`.

## [0.3.1] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.3.0] - 2026-05-19

### Added

- Added callback-backed `MDLAssetResolver`, `MDLMeshBufferAllocator`, `MDLTransformComponent`, and `MDLTransformOp` wrappers via `AssetResolver`, `MeshBufferAllocator`, `TransformComponent`, and `TransformOp` event/response APIs.
- Added first-class `ScatteringFunction` and `PhysicallyPlausibleScatteringFunction` wrappers plus `Material::new_with_scattering_function` and `Material::scattering_function`.
- Added integration coverage for custom resolvers, custom mesh-buffer allocators, scattering functions, and callback-backed transform components/ops.

### Changed

- Updated the coverage docs and SDK audit to call out the six first-class ModelIO wrappers promoted in `v0.3.0`.

## [0.2.4] - 2026-05-18

### Changed

- Added rustdoc comments across the public Model I/O wrapper surface, raising measured public-item documentation coverage to 100%.

## [0.2.3] - 2026-05-17

### Changed

- Added SAFETY comments to all unsafe blocks for comprehensive unsafe audit documentation.

## [0.2.2] - 2026-05-17

### Added

- Added `TextureFilter`, `TextureSampler`, `MaterialPropertyConnection`, `MaterialPropertyNode`, `MaterialPropertyGraph`, and standalone `MaterialProperty::new` support.
- Added `ObjectContainer`, `SubmeshTopology`, `VertexBufferLayout`, `Matrix4x4Array`, `StereoscopicCamera`, `AreaLight`, `PhotometricLight`, `Utility`, and runtime SDK constant helpers.
- Extended `VoxelArray` with asset/mesh voxelization and allocator-aware coarse/smooth mesh extraction.
- Added integration coverage for material graphs/samplers, SDK constants, matrix arrays, and USDZ conversion.

### Changed

- Raised the SDK audit from 76/117 to 117/117 top-level ModelIO symbols covered (100%).
- Reworked the coverage docs to reflect complete top-level ModelIO symbol coverage in v0.2.2.

## [0.2.1] - 2026-05-16

### Added

- Added `Transform`, `TransformComponent`, `TransformStack`, `TransformOp`, and typed transform-op wrappers for ModelIO transform authoring.
- Added `MeshBufferData`, `MeshBufferMap`, `MeshBufferAllocator`, `MeshBufferDataAllocator`, `MeshBufferZone`, and `MeshBufferZoneDefault` safe Rust APIs.
- Added `AssetResolver`, `PathAssetResolver`, `BundleAssetResolver`, and `RelativeAssetResolver` wrappers.
- Added `LightProbe`, `LightProbeIrradianceDataSource`, and `ProbePlacement`, plus `Asset::place_light_probes`.
- Extended `Texture` with color-swatch, noise, normal-map, and sky-cube factories.
- Added examples and integration tests covering transform stacks, mesh-buffer allocation, asset resolvers, procedural textures, and light probes.

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
