# modelio-rs

Safe Rust bindings for Apple's [ModelIO](https://developer.apple.com/documentation/modelio) framework on macOS. The published Cargo package is `modelio-rs`; the Rust library target is `modelio`.

> **Status:** v0.2.0 extends the crate from mesh-centric loading and inspection to assets, objects, materials, lights, cameras, voxel grids, textures, skeletal animation, animated value types, submeshes, and vertex descriptors.

## Highlights

- `Asset` for loading, exporting, and walking ModelIO assets
- `Mesh` for procedural primitives, vertex buffer reads, submesh inspection, and vertex descriptor access
- `Material` and `MaterialProperty` for physically plausible materials and property mutation
- `Light`, `PhysicallyPlausibleLight`, and `Camera` for scene-lighting and camera control surfaces
- `Object` for hierarchy creation, child traversal, and path lookup
- `VoxelArray` for sparse voxel grids, boolean ops, and coarse mesh generation
- `Texture` for URL-backed textures, checkerboards, metadata, writes, and texel extraction
- `Skeleton`, `PackedJointAnimation`, `AnimationBindComponent`, and animated value wrappers for basic rigging and animation workflows
- `VertexDescriptor` and `VertexAttribute` wrappers for vertex layout inspection and authoring

## Quick start

```rust,no_run
use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)?;
    let bbox = mesh.bounding_box();

    println!("vertices={}", mesh.vertex_count());
    println!("submeshes={}", mesh.submesh_count());
    println!("bounds min={:?} max={:?}", bbox.min, bbox.max);
    Ok(())
}
```

## Surface overview

### Assets + objects

- `Asset::new`, `from_url`, `export_to_url`
- `Asset::count`, `object_at`, `object_at_path`, `mesh_at`, `meshes`
- `Asset::frame_interval`, `start_time`, `end_time`, `up_axis`
- `Object::new`, `name`, `path`, `hidden`, `add_child`, `child_at`, `at_path`

### Meshes + submeshes + vertex data

- `Mesh::new_box`, `new_plane`, `new_ellipsoid`, `new_sphere`, `new_cylinder`, `new_icosahedron`
- `Mesh::vertex_count`, `vertex_buffers`, `submeshes`, `bounding_box`, `vertex_descriptor`
- `VertexAttributeData::info`, `bytes`
- `Submesh::index_count`, `index_type`, `geometry_type`, `index_buffer`, `material`, `set_material`
- `VertexDescriptor::info`, `attributes`, `attribute_named`, `copy`
- `VertexAttribute::new`, `info`, `set_initialization_value`

### Materials + textures

- `Material::new`, `info`, `material_face`, `property_with_semantic`
- `MaterialProperty::info`, `set_float`, `set_color`, `set_string`, `set_url`, `texture`
- `Texture::from_url`, `new_checkerboard`, `info`, `write_to_url`
- `Texture::texel_data_top_left`, `texel_data_bottom_left`

### Lights + cameras

- `Light::new`, `info`, `set_light_type`, `set_color_space`, `irradiance_at_point`
- `PhysicallyPlausibleLight::new`, `info`, `set_color_temperature`, `set_lumens`, cone-angle and attenuation setters
- `Camera::new`, `info`, `set_projection`, `set_field_of_view`, `look_at`, `look_at_from`, `ray_to`, `frame_bounding_box`

### Voxels + animation

- `VoxelArray::new`, `info`, `set_voxel`, `voxel_exists`, `voxel_indices`, `voxels_within_extent`
- `VoxelArray::union_with`, `intersect_with`, `difference_with`, `coarse_mesh`, `mesh`
- `Skeleton::new`, `info`, `joint_bind_transforms`, `joint_rest_transforms`
- `PackedJointAnimation::new`, `info`, `translations`, `rotations`, `scales`
- `AnimationBindComponent::new`, `info`, `set_skeleton`, `set_packed_joint_animation`, `set_joint_paths`
- `AnimatedScalar`, `AnimatedVector2/3/4`, `AnimatedQuaternion`, `AnimatedMatrix4x4`
- `AnimatedScalarArray`, `AnimatedVector3Array`, `AnimatedQuaternionArray`

## Examples

The crate ships numbered examples covering every logical area:

- `01_primitive_smoke`
- `02_asset_basics`
- `03_material_properties`
- `04_light_defaults`
- `05_physically_plausible_light`
- `06_camera_controls`
- `07_object_hierarchy`
- `08_voxel_array_boolean`
- `09_texture_checkerboard`
- `10_animation_bind_component`
- `11_animated_value_types`
- `12_submesh_material`
- `13_vertex_attribute_descriptor`
- `14_skeleton_basics`

Run one with:

```bash
cargo run --example 09_texture_checkerboard
```

Run them all with:

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Tests

Integration smoke tests live under `tests/` with one file per logical area plus `tests/api_coverage.rs`, which validates the Swift bridge against the active macOS SDK headers.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
