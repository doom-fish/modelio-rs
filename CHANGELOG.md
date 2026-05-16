# Changelog

## [0.1.0] - 2026-05-16

### Added

- Initial `modelio-rs` release for macOS asset loading and procedural mesh generation.
- `Asset`, `Mesh`, `Submesh`, `MeshBuffer`, `VertexAttributeData`, `Material`, `MaterialProperty`, and `Texture` wrappers.
- Swift bridge for `ModelIO.framework` object ownership, mesh generation, material inspection, and texture extraction.
- Safe Rust enums and constants for ModelIO geometry kinds, material semantics, texture channel encodings, and vertex formats.
- Smoke example `examples/01_primitive_smoke.rs` covering procedural box generation and buffer inspection.
- Header-audit test `tests/api_coverage.rs` validating the targeted v0.1 surface against the active SDK.
