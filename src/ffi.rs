#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_retain(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_release(handle: *mut c_void);

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_new_empty(
        out_asset: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_new_with_url(
        path: *const c_char,
        out_asset: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_can_import_file_extension(path_extension: *const c_char) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_can_export_file_extension(path_extension: *const c_char) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_bounding_box(
        handle: *mut c_void,
        out_min_x: *mut f32,
        out_min_y: *mut f32,
        out_min_z: *mut f32,
        out_max_x: *mut f32,
        out_max_y: *mut f32,
        out_max_z: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_bounding_box_at_time(
        handle: *mut c_void,
        time: f64,
        out_min_x: *mut f32,
        out_min_y: *mut f32,
        out_min_z: *mut f32,
        out_max_x: *mut f32,
        out_max_y: *mut f32,
        out_max_z: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_url_string(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_object_at_index(handle: *mut c_void, index: u64) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_mesh_at_index(handle: *mut c_void, index: u64) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_object_at_path(handle: *mut c_void, path: *const c_char) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_add_object(handle: *mut c_void, object: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_remove_object(handle: *mut c_void, object: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_load_textures(handle: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_export_to_url(
        handle: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_frame_interval(handle: *mut c_void) -> f64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_set_frame_interval(handle: *mut c_void, value: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_start_time(handle: *mut c_void) -> f64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_set_start_time(handle: *mut c_void, value: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_end_time(handle: *mut c_void) -> f64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_set_end_time(handle: *mut c_void, value: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_up_axis(
        handle: *mut c_void,
        out_x: *mut f32,
        out_y: *mut f32,
        out_z: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_set_up_axis(handle: *mut c_void, x: f32, y: f32, z: f32);

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
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
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
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
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
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
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
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
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_new_icosahedron(
        extent_x: f32,
        extent_y: f32,
        extent_z: f32,
        inward_normals: i32,
        geometry_type: i32,
        out_mesh: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_vertex_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_vertex_buffer_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_vertex_buffer_at(handle: *mut c_void, index: u64) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_submesh_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_submesh_at(handle: *mut c_void, index: u64) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_bounding_box(
        handle: *mut c_void,
        out_min_x: *mut f32,
        out_min_y: *mut f32,
        out_min_z: *mut f32,
        out_max_x: *mut f32,
        out_max_y: *mut f32,
        out_max_z: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_vertex_descriptor(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_vertex_attribute_data(
        handle: *mut c_void,
        attribute_name: *const c_char,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_copy_bytes(
        handle: *mut c_void,
        out_bytes: *mut u8,
        capacity: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_attribute_data_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_attribute_data_copy_bytes(
        handle: *mut c_void,
        out_bytes: *mut u8,
        capacity: u64,
    ) -> u64;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_index_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_index_type(handle: *mut c_void) -> u32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_geometry_type(handle: *mut c_void) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_name_string(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_set_name(handle: *mut c_void, name: *const c_char);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_index_buffer(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_index_buffer_as_type(handle: *mut c_void, index_type: u32) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_material(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_set_topology(handle: *mut c_void, topology: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_set_material(handle: *mut c_void, material: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_new(
        submesh: *mut c_void,
        out_topology: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_face_topology(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_set_face_topology(handle: *mut c_void, buffer: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_face_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_set_face_count(handle: *mut c_void, count: u64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_vertex_crease_indices(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_set_vertex_crease_indices(handle: *mut c_void, buffer: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_vertex_creases(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_set_vertex_creases(handle: *mut c_void, buffer: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_vertex_crease_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_set_vertex_crease_count(handle: *mut c_void, count: u64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_edge_crease_indices(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_set_edge_crease_indices(handle: *mut c_void, buffer: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_edge_creases(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_set_edge_creases(handle: *mut c_void, buffer: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_edge_crease_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_submesh_topology_set_edge_crease_count(handle: *mut c_void, count: u64);

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_new(
        name: *const c_char,
        physically_plausible: i32,
        out_material: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_new_with_scattering_function(
        name: *const c_char,
        scattering_function: *mut c_void,
        out_material: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_scattering_function(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_scattering_function_new(
        out_scattering_function: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_scattering_function_property(handle: *mut c_void, code: u32) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_scattering_function_is_physically_plausible(handle: *mut c_void) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_scattering_function_new(
        out_scattering_function: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_scattering_function_version(handle: *mut c_void) -> i64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_scattering_function_property(
        handle: *mut c_void,
        code: u32,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_name_string(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_set_name(handle: *mut c_void, name: *const c_char);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_material_face(handle: *mut c_void) -> u32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_set_material_face(handle: *mut c_void, face: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_remove_all_properties(handle: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_load_textures_using_resolver(handle: *mut c_void, resolver: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_at(handle: *mut c_void, index: u64) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_named(
        handle: *mut c_void,
        property_name: *const c_char,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_with_semantic(handle: *mut c_void, semantic: u32) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_new(
        name: *const c_char,
        semantic: u32,
        out_property: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_named_name_string(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_named_set_name(handle: *mut c_void, name: *const c_char);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_texture(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_texture_sampler(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_set_texture_sampler(handle: *mut c_void, sampler: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_filter_new(
        out_filter: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_filter_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_filter_set_s_wrap_mode(handle: *mut c_void, raw_value: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_filter_set_t_wrap_mode(handle: *mut c_void, raw_value: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_filter_set_r_wrap_mode(handle: *mut c_void, raw_value: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_filter_set_min_filter(handle: *mut c_void, raw_value: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_filter_set_mag_filter(handle: *mut c_void, raw_value: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_filter_set_mip_filter(handle: *mut c_void, raw_value: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_sampler_new(
        out_sampler: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_sampler_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_sampler_texture(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_sampler_set_texture(handle: *mut c_void, texture: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_sampler_hardware_filter(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_sampler_set_hardware_filter(handle: *mut c_void, filter: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_sampler_transform(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_sampler_set_transform(handle: *mut c_void, transform: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_connection_new(
        output: *mut c_void,
        input: *mut c_void,
        out_connection: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_connection_output(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_connection_input(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_node_new(
        inputs: *const *mut c_void,
        input_count: u64,
        outputs: *const *mut c_void,
        output_count: u64,
        out_node: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_node_inputs(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_node_outputs(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_graph_new(
        nodes: *const *mut c_void,
        node_count: u64,
        connections: *const *mut c_void,
        connection_count: u64,
        out_graph: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_graph_evaluate(handle: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_graph_nodes(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_graph_connections(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_set_float(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_set_float2(handle: *mut c_void, x: f32, y: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_set_float3(handle: *mut c_void, x: f32, y: f32, z: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_set_float4(handle: *mut c_void, x: f32, y: f32, z: f32, w: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_set_matrix4x4(handle: *mut c_void, values: *const f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_set_string(handle: *mut c_void, value: *const c_char);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_set_url(handle: *mut c_void, value: *const c_char);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_set_color(
        handle: *mut c_void,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_material_property_set_luminance(handle: *mut c_void, value: f32);

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_url_texture_new(
        path: *const c_char,
        name: *const c_char,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_checkerboard_texture_new(
        divisions: f32,
        name: *const c_char,
        width: i32,
        height: i32,
        channel_count: u64,
        channel_encoding: i32,
        color1_r: f32,
        color1_g: f32,
        color1_b: f32,
        color1_a: f32,
        color2_r: f32,
        color2_g: f32,
        color2_b: f32,
        color2_a: f32,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_write_to_url(
        handle: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_texel_data_length(handle: *mut c_void, top_left_origin: i32) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_texture_copy_texel_data(
        handle: *mut c_void,
        top_left_origin: i32,
        out_bytes: *mut u8,
        capacity: u64,
    ) -> u64;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_new(out_object: *mut *mut c_void, out_error_message: *mut *mut c_char)
        -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_kind(handle: *mut c_void) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_name_string(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_set_name(handle: *mut c_void, name: *const c_char);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_path_string(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_hidden(handle: *mut c_void) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_set_hidden(handle: *mut c_void, hidden: i32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_add_child(handle: *mut c_void, child: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_children_container(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_set_children_container(handle: *mut c_void, container: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_child_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_child_at(handle: *mut c_void, index: u64) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_at_path(handle: *mut c_void, path: *const c_char) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_bounding_box_at_time(
        handle: *mut c_void,
        time: f64,
        out_min_x: *mut f32,
        out_min_y: *mut f32,
        out_min_z: *mut f32,
        out_max_x: *mut f32,
        out_max_y: *mut f32,
        out_max_z: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_container_new(
        out_container: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_container_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_container_object_at(handle: *mut c_void, index: u64) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_container_add_object(handle: *mut c_void, object: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_container_remove_object(handle: *mut c_void, object: *mut c_void);

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_new(out_light: *mut *mut c_void, out_error_message: *mut *mut c_char) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_set_light_type(handle: *mut c_void, raw_value: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_set_color_space(handle: *mut c_void, color_space: *const c_char);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_irradiance_at_point(
        handle: *mut c_void,
        x: f32,
        y: f32,
        z: f32,
        out_r: *mut f32,
        out_g: *mut f32,
        out_b: *mut f32,
        out_a: *mut f32,
    );

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_light_new(
        out_light: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_light_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_light_set_color_temperature(
        handle: *mut c_void,
        temperature: f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_light_set_color(
        handle: *mut c_void,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_light_set_lumens(handle: *mut c_void, lumens: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_light_set_inner_cone_angle(handle: *mut c_void, angle: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_light_set_outer_cone_angle(handle: *mut c_void, angle: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_light_set_attenuation_start_distance(
        handle: *mut c_void,
        distance: f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_physically_plausible_light_set_attenuation_end_distance(
        handle: *mut c_void,
        distance: f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_area_light_new(
        out_light: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_area_light_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_area_light_set_area_radius(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_area_light_set_super_elliptic_power(handle: *mut c_void, x: f32, y: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_area_light_set_aspect(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_photometric_light_new(
        out_light: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_photometric_light_new_with_ies_profile(
        path: *const c_char,
        out_light: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_photometric_light_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_photometric_light_generate_spherical_harmonics_from_light(
        handle: *mut c_void,
        level: u64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_photometric_light_generate_cubemap_from_light(
        handle: *mut c_void,
        texture_size: u64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_photometric_light_generate_texture(
        handle: *mut c_void,
        texture_size: u64,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_photometric_light_light_cube_map(handle: *mut c_void) -> *mut c_void;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_new(out_camera: *mut *mut c_void, out_error_message: *mut *mut c_char)
        -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_set_projection(handle: *mut c_void, raw_value: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_set_near_visibility_distance(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_set_far_visibility_distance(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_set_world_to_meters_conversion_scale(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_set_focal_length(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_set_focus_distance(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_set_field_of_view(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_look_at(handle: *mut c_void, x: f32, y: f32, z: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_look_at_from(
        handle: *mut c_void,
        focus_x: f32,
        focus_y: f32,
        focus_z: f32,
        camera_x: f32,
        camera_y: f32,
        camera_z: f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_frame_bounding_box(
        handle: *mut c_void,
        min_x: f32,
        min_y: f32,
        min_z: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
        set_near_and_far: i32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_ray_to(
        handle: *mut c_void,
        pixel_x: i32,
        pixel_y: i32,
        viewport_width: i32,
        viewport_height: i32,
        out_x: *mut f32,
        out_y: *mut f32,
        out_z: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_camera_bokeh_kernel(handle: *mut c_void, width: i32, height: i32) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_stereoscopic_camera_new(
        out_camera: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_stereoscopic_camera_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_stereoscopic_camera_set_inter_pupillary_distance(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_stereoscopic_camera_set_left_vergence(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_stereoscopic_camera_set_right_vergence(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_stereoscopic_camera_set_overlap(handle: *mut c_void, value: f32);

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_new_with_asset(
        asset: *mut c_void,
        divisions: i32,
        patch_radius: f32,
        out_voxel_array: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_new_with_indices(
        values: *const i32,
        count: u64,
        min_x: f32,
        min_y: f32,
        min_z: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
        voxel_extent: f32,
        out_voxel_array: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_set_voxels_for_mesh(
        handle: *mut c_void,
        mesh: *mut c_void,
        divisions: i32,
        patch_radius: f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_set_voxel(handle: *mut c_void, x: i32, y: i32, z: i32, shell: i32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_voxel_exists(
        handle: *mut c_void,
        x: i32,
        y: i32,
        z: i32,
        shell: i32,
        allow_any_x: i32,
        allow_any_y: i32,
        allow_any_z: i32,
        allow_any_shell: i32,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_copy_indices(
        handle: *mut c_void,
        out_values: *mut i32,
        capacity_indices: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_copy_voxels_within_extent(
        handle: *mut c_void,
        min_x: i32,
        min_y: i32,
        min_z: i32,
        min_shell: i32,
        max_x: i32,
        max_y: i32,
        max_z: i32,
        max_shell: i32,
        out_values: *mut i32,
        capacity_indices: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_union(handle: *mut c_void, other: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_intersect(handle: *mut c_void, other: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_difference(handle: *mut c_void, other: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_index_of_spatial_location(
        handle: *mut c_void,
        x: f32,
        y: f32,
        z: f32,
        out_values: *mut i32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_spatial_location_of_index(
        handle: *mut c_void,
        x: i32,
        y: i32,
        z: i32,
        shell: i32,
        out_x: *mut f32,
        out_y: *mut f32,
        out_z: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_voxel_bounding_box_at_index(
        handle: *mut c_void,
        x: i32,
        y: i32,
        z: i32,
        shell: i32,
        out_min_x: *mut f32,
        out_min_y: *mut f32,
        out_min_z: *mut f32,
        out_max_x: *mut f32,
        out_max_y: *mut f32,
        out_max_z: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_convert_to_signed_shell_field(handle: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_set_shell_field_interior_thickness(handle: *mut c_void, value: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_set_shell_field_exterior_thickness(handle: *mut c_void, value: f32);
    #[allow(dead_code)]
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_coarse_mesh(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_coarse_mesh_with_allocator(
        handle: *mut c_void,
        allocator: *mut c_void,
    ) -> *mut c_void;
    #[allow(dead_code)]
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_mesh(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_voxel_array_mesh_with_allocator(
        handle: *mut c_void,
        allocator: *mut c_void,
    ) -> *mut c_void;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_matrix4x4_array_new(
        element_count: u64,
        out_array: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_matrix4x4_array_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_matrix4x4_array_clear(handle: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_matrix4x4_array_set_float_matrices(
        handle: *mut c_void,
        values: *const f32,
        count: u64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_matrix4x4_array_copy_float_matrices(
        handle: *mut c_void,
        out_values: *mut f32,
        capacity_matrices: u64,
    ) -> u64;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_value_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_value_clear(handle: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_value_set_interpolation(handle: *mut c_void, raw_value: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_scalar_new(
        out_value: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_scalar_set_float(handle: *mut c_void, value: f32, time: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_scalar_float_value(handle: *mut c_void, time: f64) -> f32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector2_new(
        out_value: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector2_set_float2(handle: *mut c_void, x: f32, y: f32, time: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector2_copy_float2_value(
        handle: *mut c_void,
        time: f64,
        out_x: *mut f32,
        out_y: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector3_new(
        out_value: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector3_set_float3(handle: *mut c_void, x: f32, y: f32, z: f32, time: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector3_copy_float3_value(
        handle: *mut c_void,
        time: f64,
        out_x: *mut f32,
        out_y: *mut f32,
        out_z: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector4_new(
        out_value: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector4_set_float4(
        handle: *mut c_void,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        time: f64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector4_copy_float4_value(
        handle: *mut c_void,
        time: f64,
        out_x: *mut f32,
        out_y: *mut f32,
        out_z: *mut f32,
        out_w: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_quaternion_new(
        out_value: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_quaternion_set_float(
        handle: *mut c_void,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        time: f64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_quaternion_copy_float_value(
        handle: *mut c_void,
        time: f64,
        out_values: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_matrix4x4_new(
        out_value: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_matrix4x4_set_float(handle: *mut c_void, values: *const f32, time: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_matrix4x4_copy_float_value(
        handle: *mut c_void,
        time: f64,
        out_values: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_scalar_array_new(
        element_count: u64,
        out_value: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_scalar_array_set_float(
        handle: *mut c_void,
        values: *const f32,
        count: u64,
        time: f64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_scalar_array_copy_float_at_time(
        handle: *mut c_void,
        time: f64,
        out_values: *mut f32,
        capacity: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector3_array_new(
        element_count: u64,
        out_value: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector3_array_set_float(
        handle: *mut c_void,
        values: *const f32,
        count: u64,
        time: f64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_vector3_array_copy_float_at_time(
        handle: *mut c_void,
        time: f64,
        out_values: *mut f32,
        capacity_elements: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_quaternion_array_new(
        element_count: u64,
        out_value: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_quaternion_array_set_float(
        handle: *mut c_void,
        values: *const f32,
        count: u64,
        time: f64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animated_quaternion_array_copy_float_at_time(
        handle: *mut c_void,
        time: f64,
        out_values: *mut f32,
        capacity_elements: u64,
    ) -> u64;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_packed_joint_animation_new(
        name: *const c_char,
        joint_paths: *const *const c_char,
        joint_path_count: u64,
        out_animation: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_packed_joint_animation_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_packed_joint_animation_translations(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_packed_joint_animation_rotations(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_packed_joint_animation_scales(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animation_bind_component_new(
        out_component: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animation_bind_component_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animation_bind_component_set_skeleton(handle: *mut c_void, skeleton: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animation_bind_component_skeleton(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animation_bind_component_set_packed_joint_animation(
        handle: *mut c_void,
        animation: *mut c_void,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animation_bind_component_packed_joint_animation(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animation_bind_component_set_joint_paths(
        handle: *mut c_void,
        joint_paths: *const *const c_char,
        joint_path_count: u64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_animation_bind_component_set_geometry_bind_transform(
        handle: *mut c_void,
        values: *const f32,
    );

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_skeleton_new(
        name: *const c_char,
        joint_paths: *const *const c_char,
        joint_path_count: u64,
        out_skeleton: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_skeleton_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_skeleton_joint_bind_transform_array(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_skeleton_joint_rest_transform_array(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_skeleton_copy_joint_bind_transforms(
        handle: *mut c_void,
        out_values: *mut f32,
        capacity_matrices: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_skeleton_copy_joint_rest_transforms(
        handle: *mut c_void,
        out_values: *mut f32,
        capacity_matrices: u64,
    ) -> u64;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_attribute_new(
        name: *const c_char,
        format: u32,
        offset: u64,
        buffer_index: u64,
        out_attribute: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_attribute_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_attribute_set_name(handle: *mut c_void, name: *const c_char);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_attribute_set_format(handle: *mut c_void, raw_value: u32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_attribute_set_offset(handle: *mut c_void, offset: u64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_attribute_set_buffer_index(handle: *mut c_void, buffer_index: u64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_attribute_set_time(handle: *mut c_void, time: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_attribute_set_initialization_value(
        handle: *mut c_void,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_descriptor_new_copy(
        handle: *mut c_void,
        out_descriptor: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_buffer_layout_new(
        stride: u64,
        out_layout: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_buffer_layout_stride(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_buffer_layout_set_stride(handle: *mut c_void, stride: u64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_descriptor_info_json(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_descriptor_attribute_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_descriptor_attribute_at(handle: *mut c_void, index: u64) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_descriptor_attribute_named(
        handle: *mut c_void,
        name: *const c_char,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_descriptor_layout_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_descriptor_layout_at(handle: *mut c_void, index: u64) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_descriptor_reset(handle: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_descriptor_set_packed_offsets(handle: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_vertex_descriptor_set_packed_strides(handle: *mut c_void);

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_array_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_array_object_at(handle: *mut c_void, index: u64) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_sdk_constant_string(code: u32) -> *mut c_char;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_resolver_new_with_callback(
        callback_context: *mut c_void,
        out_resolver: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_resolver_can_resolve_named(handle: *mut c_void, name: *const c_char) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_resolver_resolve_named(
        handle: *mut c_void,
        name: *const c_char,
    ) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_path_asset_resolver_new(
        path: *const c_char,
        out_resolver: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_path_asset_resolver_path(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_path_asset_resolver_set_path(handle: *mut c_void, path: *const c_char);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_bundle_asset_resolver_new(
        path: *const c_char,
        out_resolver: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_bundle_asset_resolver_path(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_bundle_asset_resolver_set_path(handle: *mut c_void, path: *const c_char);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_relative_asset_resolver_new(
        asset: *mut c_void,
        out_resolver: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_relative_asset_resolver_asset(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_relative_asset_resolver_set_asset(handle: *mut c_void, asset: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_utility_convert_to_usdz(
        input_url: *const c_char,
        output_url: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_fill_data(
        handle: *mut c_void,
        bytes: *const u8,
        count: u64,
        offset: u64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_map(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_allocator(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_zone(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_is_data(handle: *mut c_void) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_data_new(
        length: u64,
        buffer_type: u32,
        out_buffer: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_data_new_with_bytes(
        bytes: *const u8,
        count: u64,
        buffer_type: u32,
        out_buffer: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_data_copy_data(
        handle: *mut c_void,
        out_bytes: *mut u8,
        capacity: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_map_copy_bytes(
        handle: *mut c_void,
        length: u64,
        out_bytes: *mut u8,
        capacity: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_map_write_bytes(
        handle: *mut c_void,
        length: u64,
        bytes: *const u8,
        count: u64,
        offset: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_allocator_new_with_callback(
        callback_context: *mut c_void,
        out_allocator: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_allocator_new_zone(
        handle: *mut c_void,
        capacity: u64,
        out_zone: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_allocator_new_zone_for_buffers_with_size(
        handle: *mut c_void,
        sizes: *const u64,
        types: *const u32,
        count: u64,
        out_zone: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_allocator_new_buffer(
        handle: *mut c_void,
        length: u64,
        buffer_type: u32,
        out_buffer: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_allocator_new_buffer_with_data(
        handle: *mut c_void,
        bytes: *const u8,
        count: u64,
        buffer_type: u32,
        out_buffer: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_allocator_new_buffer_from_zone_length(
        handle: *mut c_void,
        zone: *mut c_void,
        length: u64,
        buffer_type: u32,
        out_buffer: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_allocator_new_buffer_from_zone_data(
        handle: *mut c_void,
        zone: *mut c_void,
        bytes: *const u8,
        count: u64,
        buffer_type: u32,
        out_buffer: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_data_allocator_new(
        out_allocator: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_zone_capacity(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_zone_allocator(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_zone_is_default(handle: *mut c_void) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_mesh_buffer_zone_default_new(
        out_zone: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_color_swatch_texture_new_temperature_gradient(
        color_temperature1: f32,
        color_temperature2: f32,
        name: *const c_char,
        width: i32,
        height: i32,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_color_swatch_texture_new_color_gradient(
        color1_r: f32,
        color1_g: f32,
        color1_b: f32,
        color1_a: f32,
        color2_r: f32,
        color2_g: f32,
        color2_b: f32,
        color2_a: f32,
        name: *const c_char,
        width: i32,
        height: i32,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_noise_texture_new_vector(
        smoothness: f32,
        name: *const c_char,
        width: i32,
        height: i32,
        channel_encoding: i32,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_noise_texture_new_scalar(
        smoothness: f32,
        name: *const c_char,
        width: i32,
        height: i32,
        channel_count: u64,
        channel_encoding: i32,
        grayscale: i32,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_noise_texture_new_cellular(
        frequency: f32,
        name: *const c_char,
        width: i32,
        height: i32,
        channel_encoding: i32,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_normal_map_texture_new(
        source_texture: *mut c_void,
        name: *const c_char,
        smoothness: f32,
        contrast: f32,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_sky_cube_texture_new(
        name: *const c_char,
        channel_encoding: i32,
        width: i32,
        height: i32,
        turbidity: f32,
        sun_elevation: f32,
        upper_atmosphere_scattering: f32,
        ground_albedo: f32,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_sky_cube_texture_new_with_azimuth(
        name: *const c_char,
        channel_encoding: i32,
        width: i32,
        height: i32,
        turbidity: f32,
        sun_elevation: f32,
        sun_azimuth: f32,
        upper_atmosphere_scattering: f32,
        ground_albedo: f32,
        out_texture: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_sky_cube_texture_update(handle: *mut c_void);

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_transform_component(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_new_with_callback(
        callback_context: *mut c_void,
        out_component: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_object_set_transform_component(handle: *mut c_void, component: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_matrix(handle: *mut c_void, out_values: *mut f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_set_matrix(handle: *mut c_void, values: *const f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_resets_transform(handle: *mut c_void) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_set_resets_transform(handle: *mut c_void, resets_transform: i32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_minimum_time(handle: *mut c_void) -> f64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_maximum_time(handle: *mut c_void) -> f64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_key_time_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_copy_key_times(
        handle: *mut c_void,
        out_values: *mut f64,
        capacity: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_local_transform_at_time(
        handle: *mut c_void,
        time: f64,
        out_values: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_global_transform_with_object(
        object: *mut c_void,
        time: f64,
        out_values: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_is_transform(handle: *mut c_void) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_component_is_transform_stack(handle: *mut c_void) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_new(
        out_transform: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_new_with_component(
        component: *mut c_void,
        out_transform: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_new_with_component_resets_transform(
        component: *mut c_void,
        resets_transform: i32,
        out_transform: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_new_with_matrix(
        values: *const f32,
        out_transform: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_new_with_matrix_resets_transform(
        values: *const f32,
        resets_transform: i32,
        out_transform: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_set_identity(handle: *mut c_void);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_translation_at_time(handle: *mut c_void, time: f64, out_values: *mut f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_rotation_at_time(handle: *mut c_void, time: f64, out_values: *mut f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_shear_at_time(handle: *mut c_void, time: f64, out_values: *mut f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_scale_at_time(handle: *mut c_void, time: f64, out_values: *mut f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_set_matrix_for_time(handle: *mut c_void, values: *const f32, time: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_set_translation_for_time(
        handle: *mut c_void,
        x: f32,
        y: f32,
        z: f32,
        time: f64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_set_rotation_for_time(
        handle: *mut c_void,
        x: f32,
        y: f32,
        z: f32,
        time: f64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_set_shear_for_time(handle: *mut c_void, x: f32, y: f32, z: f32, time: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_set_scale_for_time(handle: *mut c_void, x: f32, y: f32, z: f32, time: f64);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_rotation_matrix_at_time(
        handle: *mut c_void,
        time: f64,
        out_values: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_translation(handle: *mut c_void, out_values: *mut f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_set_translation(handle: *mut c_void, x: f32, y: f32, z: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_rotation(handle: *mut c_void, out_values: *mut f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_set_rotation(handle: *mut c_void, x: f32, y: f32, z: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_shear(handle: *mut c_void, out_values: *mut f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_set_shear(handle: *mut c_void, x: f32, y: f32, z: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_scale(handle: *mut c_void, out_values: *mut f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_set_scale(handle: *mut c_void, x: f32, y: f32, z: f32);
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_op_new_with_callback(
        callback_context: *mut c_void,
        out_transform_op: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_op_name_string(handle: *mut c_void) -> *mut c_char;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_op_is_inverse(handle: *mut c_void) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_op_copy_float4x4_at_time(
        handle: *mut c_void,
        time: f64,
        out_values: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_op_copy_double4x4_at_time(
        handle: *mut c_void,
        time: f64,
        out_values: *mut f64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_rotate_x_op_animated_value(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_rotate_y_op_animated_value(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_rotate_z_op_animated_value(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_rotate_op_animated_value(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_translate_op_animated_value(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_scale_op_animated_value(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_matrix_op_animated_value(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_orient_op_animated_value(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_new(
        out_stack: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_add_translate_op(
        handle: *mut c_void,
        name: *const c_char,
        inverse: i32,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_add_rotate_x_op(
        handle: *mut c_void,
        name: *const c_char,
        inverse: i32,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_add_rotate_y_op(
        handle: *mut c_void,
        name: *const c_char,
        inverse: i32,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_add_rotate_z_op(
        handle: *mut c_void,
        name: *const c_char,
        inverse: i32,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_add_rotate_op(
        handle: *mut c_void,
        name: *const c_char,
        rotation_order: u64,
        inverse: i32,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_add_scale_op(
        handle: *mut c_void,
        name: *const c_char,
        inverse: i32,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_add_matrix_op(
        handle: *mut c_void,
        name: *const c_char,
        inverse: i32,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_add_orient_op(
        handle: *mut c_void,
        name: *const c_char,
        inverse: i32,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_animated_value_named(
        handle: *mut c_void,
        name: *const c_char,
    ) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_copy_float4x4_at_time(
        handle: *mut c_void,
        time: f64,
        out_values: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_transform_stack_transform_ops(handle: *mut c_void) -> *mut c_void;

    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_new(
        reflective_texture: *mut c_void,
        irradiance_texture: *mut c_void,
        out_probe: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_generate_spherical_harmonics_from_irradiance(
        handle: *mut c_void,
        spherical_harmonics_level: u64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_reflective_texture(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_irradiance_texture(handle: *mut c_void) -> *mut c_void;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_spherical_harmonics_level(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_spherical_harmonics_coefficient_count(handle: *mut c_void) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_copy_spherical_harmonics_coefficients(
        handle: *mut c_void,
        out_values: *mut f32,
        capacity: u64,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_irradiance_data_source_new(
        min_x: f32,
        min_y: f32,
        min_z: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
        spherical_harmonics_level: u64,
        callback_context: *mut c_void,
        out_data_source: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_irradiance_data_source_bounding_box(
        handle: *mut c_void,
        out_min_x: *mut f32,
        out_min_y: *mut f32,
        out_min_z: *mut f32,
        out_max_x: *mut f32,
        out_max_y: *mut f32,
        out_max_z: *mut f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_irradiance_data_source_set_bounding_box(
        handle: *mut c_void,
        min_x: f32,
        min_y: f32,
        min_z: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_irradiance_data_source_spherical_harmonics_level(
        handle: *mut c_void,
    ) -> u64;
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_light_probe_irradiance_data_source_set_spherical_harmonics_level(
        handle: *mut c_void,
        spherical_harmonics_level: u64,
    );
    /// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
    pub fn mdl_asset_place_light_probes(
        density: f32,
        heuristic: i32,
        data_source: *mut c_void,
    ) -> *mut c_void;
}

/// Groups helper APIs for the corresponding Model I/O status symbols.
pub mod status {
    /// Exposes the corresponding Model I/O constant for ok: i32.
    pub const OK: i32 = 0;
    /// Exposes the corresponding Model I/O constant for invalid argument: i32.
    pub const INVALID_ARGUMENT: i32 = -1;
    /// Exposes the corresponding Model I/O constant for null result: i32.
    pub const NULL_RESULT: i32 = -2;
    /// Exposes the corresponding Model I/O constant for framework: i32.
    pub const FRAMEWORK: i32 = -3;
}
