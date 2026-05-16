# modelio-rs

Safe Rust bindings for Apple's [ModelIO](https://developer.apple.com/documentation/modelio) framework — 3D assets, meshes, materials, textures, and primitive generators on macOS. The published Cargo package is `modelio-rs`; the Rust library target is `modelio`.

> **Status:** v0.1.0 ships the practical ModelIO surface for asset loading, mesh inspection, submesh/material traversal, texture access, mesh-buffer reads, vertex-attribute reads, and procedural primitive creation.

## Highlights

- `Asset` for loading `.obj`, `.usdz`, and other ModelIO-supported assets from disk
- `Mesh` primitive generators for boxes, planes, ellipsoids / spheres, cylinders, and icosahedra
- `Submesh`, `Material`, `MaterialProperty`, and `Texture` wrappers for safe inspection
- `MeshBuffer::bytes()` and `VertexAttributeData::bytes()` for copying mapped ModelIO buffers into Rust-owned memory
- Strongly-typed enums for geometry types, index bit depths, mesh buffer kinds, material semantics, texture encodings, and ModelIO vertex-format constants

## Quick start

```rust,no_run
use modelio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)?;
    let bbox = mesh.bounding_box();

    println!("vertices={}", mesh.vertex_count());
    println!("submeshes={}", mesh.submesh_count());
    println!("bounds min={:?} max={:?}", bbox.min, bbox.max);
    Ok(())
}
```

## Surface overview

### Asset loading

- `Asset::from_url`
- `Asset::can_import_file_extension`
- `Asset::count`, `mesh_at`, `meshes`, `bounding_box`, `url`

### Mesh inspection + generation

- `Mesh::new_box`, `new_plane`, `new_ellipsoid`, `new_sphere`, `new_cylinder`, `new_icosahedron`
- `Mesh::vertex_count`, `vertex_buffer_count`, `submesh_count`, `bounding_box`
- `Mesh::vertex_buffer`, `submesh`, `vertex_attribute_data_named`

### Materials + textures

- `Submesh::material`, `index_buffer`, `index_count`, `index_type`, `geometry_type`
- `Material::property`, `property_named`, `property_with_semantic`, `properties`
- `MaterialProperty::info`, `texture`
- `Texture::from_url`, `info`, `texel_data_top_left`, `texel_data_bottom_left`

## Smoke example

Run the primitive smoke example with:

```bash
cargo run --example 01_primitive_smoke
```

It generates a procedural box mesh, validates that vertices and indices were produced, prints the mesh bounds, and then prints `✅ modelio primitive OK`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
