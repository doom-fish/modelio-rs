import Darwin
import Foundation
import ModelIO
import simd

private func mdl_mesh_buffer_info(_ buffer: any MDLMeshBuffer) -> [String: Any] {
    [
        "length": buffer.length,
        "buffer_type": buffer.type.rawValue,
    ]
}

private func mdl_vertex_attribute_data_info(_ attributeData: MDLVertexAttributeData) -> [String: Any] {
    [
        "stride": attributeData.stride,
        "format": attributeData.format.rawValue,
        "buffer_size": attributeData.bufferSize,
    ]
}

@_cdecl("mdl_mesh_new_box")
public func mdl_mesh_new_box(
    _ extentX: Float,
    _ extentY: Float,
    _ extentZ: Float,
    _ segmentX: UInt32,
    _ segmentY: UInt32,
    _ segmentZ: UInt32,
    _ inwardNormals: Int32,
    _ geometryTypeRaw: Int32,
    _ outMesh: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outMesh else {
            throw ModelIOBridgeError.invalidArgument("missing output mesh pointer")
        }
        let mesh = MDLMesh(
            boxWithExtent: SIMD3<Float>(extentX, extentY, extentZ),
            segments: SIMD3<UInt32>(segmentX, segmentY, segmentZ),
            inwardNormals: inwardNormals != 0,
            geometryType: try mdl_geometry_type(geometryTypeRaw),
            allocator: nil
        )
        outMesh.pointee = mdl_retain(mesh)
    }
}

@_cdecl("mdl_mesh_new_ellipsoid")
public func mdl_mesh_new_ellipsoid(
    _ extentX: Float,
    _ extentY: Float,
    _ extentZ: Float,
    _ segmentX: UInt32,
    _ segmentY: UInt32,
    _ inwardNormals: Int32,
    _ hemisphere: Int32,
    _ geometryTypeRaw: Int32,
    _ outMesh: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outMesh else {
            throw ModelIOBridgeError.invalidArgument("missing output mesh pointer")
        }
        let extent = SIMD3<Float>(extentX, extentY, extentZ)
        let segments = SIMD2<UInt32>(segmentX, segmentY)
        let geometryType = try mdl_geometry_type(geometryTypeRaw)
        let mesh: MDLMesh
        if hemisphere != 0 {
            mesh = MDLMesh(
                hemisphereWithExtent: extent,
                segments: segments,
                inwardNormals: inwardNormals != 0,
                cap: true,
                geometryType: geometryType,
                allocator: nil
            )
        } else {
            mesh = MDLMesh(
                sphereWithExtent: extent,
                segments: segments,
                inwardNormals: inwardNormals != 0,
                geometryType: geometryType,
                allocator: nil
            )
        }
        outMesh.pointee = mdl_retain(mesh)
    }
}

@_cdecl("mdl_mesh_new_cylinder")
public func mdl_mesh_new_cylinder(
    _ extentX: Float,
    _ extentY: Float,
    _ extentZ: Float,
    _ segmentX: UInt32,
    _ segmentY: UInt32,
    _ inwardNormals: Int32,
    _ topCap: Int32,
    _ bottomCap: Int32,
    _ geometryTypeRaw: Int32,
    _ outMesh: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outMesh else {
            throw ModelIOBridgeError.invalidArgument("missing output mesh pointer")
        }
        let mesh = MDLMesh(
            cylinderWithExtent: SIMD3<Float>(extentX, extentY, extentZ),
            segments: SIMD2<UInt32>(segmentX, segmentY),
            inwardNormals: inwardNormals != 0,
            topCap: topCap != 0,
            bottomCap: bottomCap != 0,
            geometryType: try mdl_geometry_type(geometryTypeRaw),
            allocator: nil
        )
        outMesh.pointee = mdl_retain(mesh)
    }
}

@_cdecl("mdl_mesh_new_plane")
public func mdl_mesh_new_plane(
    _ extentX: Float,
    _ extentY: Float,
    _ extentZ: Float,
    _ segmentX: UInt32,
    _ segmentY: UInt32,
    _ geometryTypeRaw: Int32,
    _ outMesh: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outMesh else {
            throw ModelIOBridgeError.invalidArgument("missing output mesh pointer")
        }
        let mesh = MDLMesh(
            planeWithExtent: SIMD3<Float>(extentX, extentY, extentZ),
            segments: SIMD2<UInt32>(segmentX, segmentY),
            geometryType: try mdl_geometry_type(geometryTypeRaw),
            allocator: nil
        )
        outMesh.pointee = mdl_retain(mesh)
    }
}

@_cdecl("mdl_mesh_new_icosahedron")
public func mdl_mesh_new_icosahedron(
    _ extentX: Float,
    _ extentY: Float,
    _ extentZ: Float,
    _ inwardNormals: Int32,
    _ geometryTypeRaw: Int32,
    _ outMesh: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outMesh else {
            throw ModelIOBridgeError.invalidArgument("missing output mesh pointer")
        }
        let mesh = MDLMesh(
            icosahedronWithExtent: SIMD3<Float>(extentX, extentY, extentZ),
            inwardNormals: inwardNormals != 0,
            geometryType: try mdl_geometry_type(geometryTypeRaw),
            allocator: nil
        )
        outMesh.pointee = mdl_retain(mesh)
    }
}

@_cdecl("mdl_mesh_vertex_count")
public func mdl_mesh_vertex_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let mesh = mdl_borrow_object(handle) as? MDLMesh else { return 0 }
    return UInt64(mesh.vertexCount)
}

@_cdecl("mdl_mesh_vertex_buffer_count")
public func mdl_mesh_vertex_buffer_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let mesh = mdl_borrow_object(handle) as? MDLMesh else { return 0 }
    return UInt64(mesh.vertexBuffers.count)
}

@_cdecl("mdl_mesh_vertex_buffer_at")
public func mdl_mesh_vertex_buffer_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let mesh = mdl_borrow_object(handle) as? MDLMesh,
          index < UInt64(mesh.vertexBuffers.count)
    else {
        return nil
    }
    return mdl_retain(mesh.vertexBuffers[Int(index)] as AnyObject)
}

@_cdecl("mdl_mesh_submesh_count")
public func mdl_mesh_submesh_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let mesh = mdl_borrow_object(handle) as? MDLMesh else { return 0 }
    return UInt64(mesh.submeshes?.count ?? 0)
}

@_cdecl("mdl_mesh_submesh_at")
public func mdl_mesh_submesh_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let mesh = mdl_borrow_object(handle) as? MDLMesh,
          let submeshes = mesh.submeshes,
          index < UInt64(submeshes.count),
          let submesh = submeshes[Int(index)] as? MDLSubmesh
    else {
        return nil
    }
    return mdl_retain(submesh)
}

@_cdecl("mdl_mesh_bounding_box")
public func mdl_mesh_bounding_box(
    _ handle: UnsafeMutableRawPointer?,
    _ outMinX: UnsafeMutablePointer<Float>?,
    _ outMinY: UnsafeMutablePointer<Float>?,
    _ outMinZ: UnsafeMutablePointer<Float>?,
    _ outMaxX: UnsafeMutablePointer<Float>?,
    _ outMaxY: UnsafeMutablePointer<Float>?,
    _ outMaxZ: UnsafeMutablePointer<Float>?
) {
    let zero = MDLAxisAlignedBoundingBox(maxBounds: SIMD3<Float>(repeating: 0), minBounds: SIMD3<Float>(repeating: 0))
    let boundingBox = (mdl_borrow_object(handle) as? MDLMesh)?.boundingBox ?? zero
    outMinX?.pointee = boundingBox.minBounds.x
    outMinY?.pointee = boundingBox.minBounds.y
    outMinZ?.pointee = boundingBox.minBounds.z
    outMaxX?.pointee = boundingBox.maxBounds.x
    outMaxY?.pointee = boundingBox.maxBounds.y
    outMaxZ?.pointee = boundingBox.maxBounds.z
}

@_cdecl("mdl_mesh_vertex_descriptor")
public func mdl_mesh_vertex_descriptor(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let mesh = mdl_borrow_object(handle) as? MDLMesh else { return nil }
    return mdl_retain(mesh.vertexDescriptor)
}

@_cdecl("mdl_mesh_vertex_attribute_data")
public func mdl_mesh_vertex_attribute_data(
    _ handle: UnsafeMutableRawPointer?,
    _ attributeName: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let mesh = mdl_borrow_object(handle) as? MDLMesh,
          let attributeName
    else {
        return nil
    }
    guard let attributeData = mesh.vertexAttributeData(forAttributeNamed: String(cString: attributeName)) else {
        return nil
    }
    return mdl_retain(attributeData)
}

@_cdecl("mdl_mesh_buffer_info_json")
public func mdl_mesh_buffer_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let buffer = mdl_borrow_object(handle) as? any MDLMeshBuffer else { return nil }
    return mdl_string(mdl_json_string(from: mdl_mesh_buffer_info(buffer)) ?? "{}")
}

@_cdecl("mdl_mesh_buffer_copy_bytes")
public func mdl_mesh_buffer_copy_bytes(
    _ handle: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UInt8>?,
    _ capacity: UInt64
) -> UInt64 {
    guard let buffer = mdl_borrow_object(handle) as? any MDLMeshBuffer,
          let outBytes
    else {
        return 0
    }
    let map = buffer.map()
    let byteCount = min(Int(capacity), buffer.length)
    memcpy(outBytes, map.bytes, byteCount)
    return UInt64(byteCount)
}

@_cdecl("mdl_vertex_attribute_data_info_json")
public func mdl_vertex_attribute_data_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributeData = mdl_borrow_object(handle) as? MDLVertexAttributeData else { return nil }
    return mdl_string(mdl_json_string(from: mdl_vertex_attribute_data_info(attributeData)) ?? "{}")
}

@_cdecl("mdl_vertex_attribute_data_copy_bytes")
public func mdl_vertex_attribute_data_copy_bytes(
    _ handle: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UInt8>?,
    _ capacity: UInt64
) -> UInt64 {
    guard let attributeData = mdl_borrow_object(handle) as? MDLVertexAttributeData,
          let outBytes
    else {
        return 0
    }
    let byteCount = min(Int(capacity), attributeData.bufferSize)
    memcpy(outBytes, attributeData.dataStart, byteCount)
    return UInt64(byteCount)
}
