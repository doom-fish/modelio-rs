import Foundation
import ModelIO
import simd

private func mdl_voxel_index_extent(_ extent: MDLVoxelIndexExtent) -> [String: Any] {
    [
        "minimum_extent": [extent.minimumExtent.x, extent.minimumExtent.y, extent.minimumExtent.z, extent.minimumExtent.w],
        "maximum_extent": [extent.maximumExtent.x, extent.maximumExtent.y, extent.maximumExtent.z, extent.maximumExtent.w],
    ]
}

private func mdl_voxel_array_info(_ voxelArray: MDLVoxelArray) -> [String: Any] {
    [
        "count": voxelArray.count,
        "bounding_box": mdl_bounding_box(voxelArray.boundingBox),
        "voxel_index_extent": mdl_voxel_index_extent(voxelArray.voxelIndexExtent),
        "is_valid_signed_shell_field": voxelArray.isValidSignedShellField,
        "shell_field_interior_thickness": voxelArray.shellFieldInteriorThickness,
        "shell_field_exterior_thickness": voxelArray.shellFieldExteriorThickness,
    ]
}

private func mdl_voxel_indices(_ data: Data?) -> [Int32] {
    guard let data else { return [] }
    return data.withUnsafeBytes { buffer in
        Array(buffer.bindMemory(to: Int32.self))
    }
}

@_cdecl("mdl_voxel_array_new_with_asset")
public func mdl_voxel_array_new_with_asset(
    _ assetHandle: UnsafeMutableRawPointer?,
    _ divisions: Int32,
    _ patchRadius: Float,
    _ outVoxelArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let asset = mdl_borrow_object(assetHandle) as? MDLAsset,
              let outVoxelArray
        else {
            throw ModelIOBridgeError.invalidArgument("missing asset or output voxel array pointer")
        }
        outVoxelArray.pointee = mdl_retain(MDLVoxelArray(asset: asset, divisions: divisions, patchRadius: patchRadius))
    }
}

@_cdecl("mdl_voxel_array_new_with_indices")
public func mdl_voxel_array_new_with_indices(
    _ values: UnsafePointer<Int32>?,
    _ count: UInt64,
    _ minX: Float,
    _ minY: Float,
    _ minZ: Float,
    _ maxX: Float,
    _ maxY: Float,
    _ maxZ: Float,
    _ voxelExtent: Float,
    _ outVoxelArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outVoxelArray else {
            throw ModelIOBridgeError.invalidArgument("missing output voxel array pointer")
        }
        let data = mdl_data_from_int32s(values, count: count * 4)
        let boundingBox = MDLAxisAlignedBoundingBox(
            maxBounds: SIMD3<Float>(maxX, maxY, maxZ),
            minBounds: SIMD3<Float>(minX, minY, minZ)
        )
        outVoxelArray.pointee = mdl_retain(MDLVoxelArray(data: data, boundingBox: boundingBox, voxelExtent: voxelExtent))
    }
}

@_cdecl("mdl_voxel_array_info_json")
public func mdl_voxel_array_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return nil }
    return mdl_string(mdl_json_string(from: mdl_voxel_array_info(voxelArray)) ?? "{}")
}

@_cdecl("mdl_voxel_array_count")
public func mdl_voxel_array_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return 0 }
    return UInt64(voxelArray.count)
}

@_cdecl("mdl_voxel_array_set_voxels_for_mesh")
public func mdl_voxel_array_set_voxels_for_mesh(
    _ handle: UnsafeMutableRawPointer?,
    _ meshHandle: UnsafeMutableRawPointer?,
    _ divisions: Int32,
    _ patchRadius: Float
) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray,
          let mesh = mdl_borrow_object(meshHandle) as? MDLMesh
    else {
        return
    }
    voxelArray.setVoxelsFor(mesh, divisions: divisions, patchRadius: patchRadius)
}

@_cdecl("mdl_voxel_array_set_voxel")
public func mdl_voxel_array_set_voxel(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Int32,
    _ y: Int32,
    _ z: Int32,
    _ shell: Int32
) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return }
    voxelArray.setVoxelAtIndex(vector_int4(x, y, z, shell))
}

@_cdecl("mdl_voxel_array_voxel_exists")
public func mdl_voxel_array_voxel_exists(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Int32,
    _ y: Int32,
    _ z: Int32,
    _ shell: Int32,
    _ allowAnyX: Int32,
    _ allowAnyY: Int32,
    _ allowAnyZ: Int32,
    _ allowAnyShell: Int32
) -> Int32 {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return 0 }
    return voxelArray.voxelExists(
        atIndex: vector_int4(x, y, z, shell),
        allowAnyX: allowAnyX != 0,
        allowAnyY: allowAnyY != 0,
        allowAnyZ: allowAnyZ != 0,
        allowAnyShell: allowAnyShell != 0
    ) ? 1 : 0
}

@_cdecl("mdl_voxel_array_copy_indices")
public func mdl_voxel_array_copy_indices(
    _ handle: UnsafeMutableRawPointer?,
    _ outValues: UnsafeMutablePointer<Int32>?,
    _ capacityIndices: UInt64
) -> UInt64 {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray,
          let outValues
    else {
        return 0
    }
    let values = mdl_voxel_indices(voxelArray.voxelIndices())
    let totalIndices = values.count / 4
    let copyCount = min(Int(capacityIndices), totalIndices)
    guard copyCount > 0 else { return 0 }
    let flattened = Array(values.prefix(copyCount * 4))
    flattened.withUnsafeBufferPointer { buffer in
        outValues.initialize(from: buffer.baseAddress!, count: flattened.count)
    }
    return UInt64(copyCount)
}

@_cdecl("mdl_voxel_array_copy_voxels_within_extent")
public func mdl_voxel_array_copy_voxels_within_extent(
    _ handle: UnsafeMutableRawPointer?,
    _ minX: Int32,
    _ minY: Int32,
    _ minZ: Int32,
    _ minShell: Int32,
    _ maxX: Int32,
    _ maxY: Int32,
    _ maxZ: Int32,
    _ maxShell: Int32,
    _ outValues: UnsafeMutablePointer<Int32>?,
    _ capacityIndices: UInt64
) -> UInt64 {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray,
          let outValues
    else {
        return 0
    }
    let extent = MDLVoxelIndexExtent(
        minimumExtent: vector_int4(minX, minY, minZ, minShell),
        maximumExtent: vector_int4(maxX, maxY, maxZ, maxShell)
    )
    let values = mdl_voxel_indices(voxelArray.voxels(within: extent))
    let totalIndices = values.count / 4
    let copyCount = min(Int(capacityIndices), totalIndices)
    guard copyCount > 0 else { return 0 }
    let flattened = Array(values.prefix(copyCount * 4))
    flattened.withUnsafeBufferPointer { buffer in
        outValues.initialize(from: buffer.baseAddress!, count: flattened.count)
    }
    return UInt64(copyCount)
}

@_cdecl("mdl_voxel_array_union")
public func mdl_voxel_array_union(_ handle: UnsafeMutableRawPointer?, _ otherHandle: UnsafeMutableRawPointer?) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray,
          let other = mdl_borrow_object(otherHandle) as? MDLVoxelArray
    else {
        return
    }
    voxelArray.union(with: other)
}

@_cdecl("mdl_voxel_array_intersect")
public func mdl_voxel_array_intersect(_ handle: UnsafeMutableRawPointer?, _ otherHandle: UnsafeMutableRawPointer?) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray,
          let other = mdl_borrow_object(otherHandle) as? MDLVoxelArray
    else {
        return
    }
    voxelArray.intersect(with: other)
}

@_cdecl("mdl_voxel_array_difference")
public func mdl_voxel_array_difference(_ handle: UnsafeMutableRawPointer?, _ otherHandle: UnsafeMutableRawPointer?) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray,
          let other = mdl_borrow_object(otherHandle) as? MDLVoxelArray
    else {
        return
    }
    voxelArray.difference(with: other)
}

@_cdecl("mdl_voxel_array_index_of_spatial_location")
public func mdl_voxel_array_index_of_spatial_location(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ outValues: UnsafeMutablePointer<Int32>?
) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray,
          let outValues
    else {
        return
    }
    let index = voxelArray.index(ofSpatialLocation: SIMD3<Float>(x, y, z))
    outValues[0] = index.x
    outValues[1] = index.y
    outValues[2] = index.z
    outValues[3] = index.w
}

@_cdecl("mdl_voxel_array_spatial_location_of_index")
public func mdl_voxel_array_spatial_location_of_index(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Int32,
    _ y: Int32,
    _ z: Int32,
    _ shell: Int32,
    _ outX: UnsafeMutablePointer<Float>?,
    _ outY: UnsafeMutablePointer<Float>?,
    _ outZ: UnsafeMutablePointer<Float>?
) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return }
    let location = voxelArray.spatialLocation(ofIndex: vector_int4(x, y, z, shell))
    outX?.pointee = location.x
    outY?.pointee = location.y
    outZ?.pointee = location.z
}

@_cdecl("mdl_voxel_array_voxel_bounding_box_at_index")
public func mdl_voxel_array_voxel_bounding_box_at_index(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Int32,
    _ y: Int32,
    _ z: Int32,
    _ shell: Int32,
    _ outMinX: UnsafeMutablePointer<Float>?,
    _ outMinY: UnsafeMutablePointer<Float>?,
    _ outMinZ: UnsafeMutablePointer<Float>?,
    _ outMaxX: UnsafeMutablePointer<Float>?,
    _ outMaxY: UnsafeMutablePointer<Float>?,
    _ outMaxZ: UnsafeMutablePointer<Float>?
) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return }
    let boundingBox = voxelArray.voxelBoundingBox(atIndex: vector_int4(x, y, z, shell))
    outMinX?.pointee = boundingBox.minBounds.x
    outMinY?.pointee = boundingBox.minBounds.y
    outMinZ?.pointee = boundingBox.minBounds.z
    outMaxX?.pointee = boundingBox.maxBounds.x
    outMaxY?.pointee = boundingBox.maxBounds.y
    outMaxZ?.pointee = boundingBox.maxBounds.z
}

@_cdecl("mdl_voxel_array_convert_to_signed_shell_field")
public func mdl_voxel_array_convert_to_signed_shell_field(_ handle: UnsafeMutableRawPointer?) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return }
    voxelArray.convertToSignedShellField()
}

@_cdecl("mdl_voxel_array_set_shell_field_interior_thickness")
public func mdl_voxel_array_set_shell_field_interior_thickness(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return }
    voxelArray.shellFieldInteriorThickness = value
}

@_cdecl("mdl_voxel_array_set_shell_field_exterior_thickness")
public func mdl_voxel_array_set_shell_field_exterior_thickness(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return }
    voxelArray.shellFieldExteriorThickness = value
}

@_cdecl("mdl_voxel_array_coarse_mesh")
public func mdl_voxel_array_coarse_mesh(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    mdl_voxel_array_coarse_mesh_with_allocator(handle, nil)
}

@_cdecl("mdl_voxel_array_coarse_mesh_with_allocator")
public func mdl_voxel_array_coarse_mesh_with_allocator(
    _ handle: UnsafeMutableRawPointer?,
    _ allocatorHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return nil }
    let allocator = mdl_borrow_object(allocatorHandle) as? any MDLMeshBufferAllocator
    guard let mesh = voxelArray.coarseMesh(using: allocator) else { return nil }
    return mdl_retain(mesh)
}

@_cdecl("mdl_voxel_array_mesh")
public func mdl_voxel_array_mesh(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    mdl_voxel_array_mesh_with_allocator(handle, nil)
}

@_cdecl("mdl_voxel_array_mesh_with_allocator")
public func mdl_voxel_array_mesh_with_allocator(
    _ handle: UnsafeMutableRawPointer?,
    _ allocatorHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let voxelArray = mdl_borrow_object(handle) as? MDLVoxelArray else { return nil }
    let allocator = mdl_borrow_object(allocatorHandle) as? any MDLMeshBufferAllocator
    guard let mesh = voxelArray.mesh(using: allocator) else { return nil }
    return mdl_retain(mesh)
}
