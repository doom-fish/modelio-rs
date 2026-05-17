# modelio-rs

Safe Rust bindings for Apple's [ModelIO](https://developer.apple.com/documentation/modelio) framework on macOS. The published Cargo package is `modelio-rs`; the Rust library target is `modelio`.

> **Status:** v0.2.2 brings the SDK audit to 117/117 top-level ModelIO symbols (100%) and adds material graphs/samplers, stereoscopic cameras, area/photometric lights, object containers, matrix arrays, voxel extras, SDK constants, and USDZ conversion helpers.

## Highlights

- `Asset` for loading, exporting, and walking ModelIO assets
- `Mesh` for procedural primitives, vertex buffer reads, submesh inspection, and vertex descriptor access
- `Material`, `MaterialProperty`, `TextureFilter`, `TextureSampler`, and material-graph wrappers for physically plausible materials, standalone properties, and sampler/graph authoring
- `Light`, `AreaLight`, `PhotometricLight`, `PhysicallyPlausibleLight`, `Camera`, and `StereoscopicCamera` for scene-lighting and camera control surfaces
- `Object` and `ObjectContainer` for hierarchy creation, child traversal, path lookup, and transform-component attachment
- `Transform`, `TransformStack`, and typed transform ops for explicit transform authoring
- `MeshBufferData`, `MeshBufferDataAllocator`, and related wrappers for buffer-authoring workflows
- `AssetResolver` plus path, bundle, and relative resolvers for asset lookup control
- `VoxelArray` for sparse voxel grids, asset/mesh voxelization, boolean ops, and coarse/smooth mesh generation
- `Texture` for URL-backed textures, checkerboards, procedural gradients/noise/sky cubes, metadata, writes, and texel extraction
- `LightProbe` and `LightProbeIrradianceDataSource` for probe generation and placement helpers
- `Skeleton`, `Matrix4x4Array`, `PackedJointAnimation`, `AnimationBindComponent`, and animated value wrappers for rigging and animation workflows
- `VertexDescriptor`, `VertexAttribute`, `VertexBufferLayout`, runtime SDK constant accessors, and `Utility` for vertex layout inspection and auxiliary SDK helpers

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

- `Asset::new`, `from_url`, `export_to_url`, `place_light_probes`
- `Asset::count`, `object_at`, `object_at_path`, `mesh_at`, `meshes`
- `Asset::frame_interval`, `start_time`, `end_time`, `up_axis`
- `Object::new`, `name`, `path`, `hidden`, `add_child`, `child_at`, `at_path`, `transform_component`, `children_container`
- `ObjectContainer::new`, `count`, `object_at`, `add_object`
- `PathAssetResolver`, `BundleAssetResolver`, `RelativeAssetResolver`, and `AssetResolver::resolve_asset_named`

### Meshes + submeshes + vertex data

- `Mesh::new_box`, `new_plane`, `new_ellipsoid`, `new_sphere`, `new_cylinder`, `new_icosahedron`
- `Mesh::vertex_count`, `vertex_buffers`, `submeshes`, `bounding_box`, `vertex_descriptor`
- `MeshBuffer::map`, `fill_data`, `allocator`, `zone`, `as_data_buffer`
- `MeshBufferData`, `MeshBufferDataAllocator`, `MeshBufferZoneDefault`, and `MeshBufferMap`
- `VertexAttributeData::info`, `bytes`
- `Submesh::index_count`, `index_type`, `geometry_type`, `index_buffer`, `material`, `set_material`, `topology`
- `SubmeshTopology::new`, crease/index buffer accessors, and face-count mutation
- `VertexDescriptor::info`, `attributes`, `attribute_named`, `copy`, `layouts`
- `VertexAttribute::new`, `info`, `set_initialization_value`
- `VertexBufferLayout::new`, `stride`, `set_stride`

### Materials + textures

- `Material::new`, `info`, `material_face`, `property_with_semantic`, `property_named`, `load_textures_using_resolver`
- `MaterialProperty::new`, `info`, `set_float`, `set_color`, `set_string`, `set_url`, `texture`, `texture_sampler`
- `TextureFilter::new`, wrap/filter setters, and `info`
- `TextureSampler::new`, texture/filter/transform setters, and `info`
- `MaterialPropertyConnection`, `MaterialPropertyNode`, and `MaterialPropertyGraph` for graph construction/evaluation
- `Texture::from_url`, `new_checkerboard`, `new_color_temperature_gradient`, `new_color_gradient`, `new_vector_noise`, `new_scalar_noise`, `new_cellular_noise`, `new_normal_map`, `new_sky_cube`
- `Texture::info`, `write_to_url`, `update_sky_cube`, `texel_data_top_left`, `texel_data_bottom_left`

### Lights + cameras

- `Light::new`, `info`, `set_light_type`, `set_color_space`, `irradiance_at_point`
- `AreaLight::new` and `PhotometricLight::new` / `new_with_ies_profile` plus info accessors
- `LightProbe::new`, `reflective_texture`, `irradiance_texture`, `generate_spherical_harmonics_from_irradiance`
- `LightProbeIrradianceDataSource::new`, `bounding_box`, `spherical_harmonics_level`
- `PhysicallyPlausibleLight::new`, `info`, `set_color_temperature`, `set_lumens`, cone-angle and attenuation setters
- `Camera::new`, `info`, `set_projection`, `set_field_of_view`, `look_at`, `look_at_from`, `ray_to`, `frame_bounding_box`
- `StereoscopicCamera::new`, `info`, and optical separation accessors

### Transforms + animation

- `Transform::new`, `from_component`, `from_matrix`, `translation`, `rotation`, `scale`, and timed setters
- `TransformComponent::matrix`, `key_times`, `local_transform_at_time`, `global_transform_with_object`
- `TransformStack::add_translate_op`, `add_rotate_*_op`, `add_scale_op`, `add_matrix_op`, `add_orient_op`, `transform_ops`
- `TransformOp` plus typed op wrappers with animated-value accessors

### Voxels + animation

- `VoxelArray::new`, `from_asset`, `info`, `set_voxel`, `voxel_exists`, `voxel_indices`, `voxels_within_extent`
- `VoxelArray::set_voxels_for_mesh`, `union_with`, `intersect_with`, `difference_with`, `coarse_mesh`, `mesh`, allocator-aware mesh extraction
- `Skeleton::new`, `info`, `joint_bind_transforms`, `joint_rest_transforms`, `joint_*_transform_array`
- `Matrix4x4Array::new`, `info`, `set_float_matrices`, `float_matrices`, `clear`
- `PackedJointAnimation::new`, `info`, `translations`, `rotations`, `scales`
- `AnimationBindComponent::new`, `info`, `set_skeleton`, `set_packed_joint_animation`, `set_joint_paths`
- `AnimatedScalar`, `AnimatedVector2/3/4`, `AnimatedQuaternion`, `AnimatedMatrix4x4`
- `AnimatedScalarArray`, `AnimatedVector3Array`, `AnimatedQuaternionArray`

### SDK helpers

- `ut_type::*` and `vertex_attribute_name::*` expose ModelIO SDK NSString constants at runtime
- `Utility::convert_to_usdz` wraps `MDLUtility`'s USDZ conversion helper

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
- `15_transform_stack_basics`
- `16_mesh_buffer_allocator`
- `17_asset_resolver_light_probe`

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
