#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mdl_object_retain(handle: *mut c_void) -> *mut c_void;
    pub fn mdl_object_release(handle: *mut c_void);

    pub fn mdl_asset_new_with_url(
        path: *const c_char,
        out_asset: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn mdl_asset_can_import_file_extension(path_extension: *const c_char) -> i32;
    pub fn mdl_asset_count(handle: *mut c_void) -> u64;
    pub fn mdl_asset_bounding_box(
        handle: *mut c_void,
        out_min_x: *mut f32,
        out_min_y: *mut f32,
        out_min_z: *mut f32,
        out_max_x: *mut f32,
        out_max_y: *mut f32,
        out_max_z: *mut f32,
    );
    pub fn mdl_asset_url_string(handle: *mut c_void) -> *mut c_char;
    pub fn mdl_asset_mesh_at_index(handle: *mut c_void, index: u64) -> *mut c_void;
    pub fn mdl_asset_load_textures(handle: *mut c_void);

    pub fn mdl_mesh_new_box(
        extent_x: f32,
        extent_y: f32,
        extent_z: f32,
        segment_x: u32,
        segment_y: u32,
        segment_z: u32,
        inward_normals: i32,
        geometry_type: i32,
        out_mesh: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn mdl_mesh_new_ellipsoid(
        extent_x: f32,
        extent_y: f32,
        extent_z: f32,
        segment_x: u32,
        segment_y: u32,
        inward_normals: i32,
        hemisphere: i32,
        geometry_type: i32,
        out_mesh: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn mdl_mesh_new_cylinder(
        extent_x: f32,
        extent_y: f32,
        extent_z: f32,
        segment_x: u32,
        segment_y: u32,
        inward_normals: i32,
        top_cap: i32,
        bottom_cap: i32,
        geometry_type: i32,
        out_mesh: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn mdl_mesh_new_plane(
        extent_x: f32,
        extent_y: f32,
        extent_z: f32,
        segment_x: u32,
        segment_y: u32,
        geometry_type: i32,
        out_mesh: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn mdl_mesh_new_icosahedron(
        extent_x: f32,
        extent_y: f32,
        extent_z: f32,
        inward_normals: i32,
        geometry_type: i32,
        out_mesh: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn mdl_mesh_vertex_count(handle: *mut c_void) -> u64;
    pub fn mdl_mesh_vertex_buffer_count(handle: *mut c_void) -> u64;
    pub fn mdl_mesh_vertex_buffer_at(handle: *mut c_void, index: u64) -> *mut c_void;
    pub fn mdl_mesh_submesh_count(handle: *mut c_void) -> u64;
    pub fn mdl_mesh_submesh_at(handle: *mut c_void, index: u64) -> *mut c_void;
    pub fn mdl_mesh_bounding_box(
        handle: *mut c_void,
        out_min_x: *mut f32,
        out_min_y: *mut f32,
        out_min_z: *mut f32,
        out_max_x: *mut f32,
        out_max_y: *mut f32,
        out_max_z: *mut f32,
    );
    pub fn mdl_mesh_vertex_attribute_data(
        handle: *mut c_void,
        attribute_name: *const c_char,
    ) -> *mut c_void;

    pub fn mdl_mesh_buffer_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn mdl_mesh_buffer_copy_bytes(
        handle: *mut c_void,
        out_bytes: *mut u8,
        capacity: u64,
    ) -> u64;

    pub fn mdl_submesh_index_count(handle: *mut c_void) -> u64;
    pub fn mdl_submesh_index_type(handle: *mut c_void) -> u32;
    pub fn mdl_submesh_geometry_type(handle: *mut c_void) -> i32;
    pub fn mdl_submesh_name_string(handle: *mut c_void) -> *mut c_char;
    pub fn mdl_submesh_index_buffer(handle: *mut c_void) -> *mut c_void;
    pub fn mdl_submesh_material(handle: *mut c_void) -> *mut c_void;

    pub fn mdl_material_count(handle: *mut c_void) -> u64;
    pub fn mdl_material_name_string(handle: *mut c_void) -> *mut c_char;
    pub fn mdl_material_property_at(handle: *mut c_void, index: u64) -> *mut c_void;
    pub fn mdl_material_property_named(
        handle: *mut c_void,
        property_name: *const c_char,
    ) -> *mut c_void;
    pub fn mdl_material_property_with_semantic(handle: *mut c_void, semantic: u32) -> *mut c_void;
    pub fn mdl_material_property_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn mdl_material_property_texture(handle: *mut c_void) -> *mut c_void;

    pub fn mdl_url_texture_new(
        path: *const c_char,
        name: *const c_char,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn mdl_texture_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn mdl_texture_texel_data_length(handle: *mut c_void, top_left_origin: i32) -> u64;
    pub fn mdl_texture_copy_texel_data(
        handle: *mut c_void,
        top_left_origin: i32,
        out_bytes: *mut u8,
        capacity: u64,
    ) -> u64;

    pub fn mdl_vertex_attribute_data_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn mdl_vertex_attribute_data_copy_bytes(
        handle: *mut c_void,
        out_bytes: *mut u8,
        capacity: u64,
    ) -> u64;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const NULL_RESULT: i32 = -2;
    pub const FRAMEWORK: i32 = -3;
}
