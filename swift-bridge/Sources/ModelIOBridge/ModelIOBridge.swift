import CoreGraphics
import Darwin
import Foundation
import ModelIO
import simd

private func mdl_geometry_type(_ rawValue: Int32) throws -> MDLGeometryType {
    guard let geometryType = MDLGeometryType(rawValue: Int(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLGeometryType: \(rawValue)")
    }
    return geometryType
}

private func mdl_semantic(_ rawValue: UInt32) throws -> MDLMaterialSemantic {
    guard let semantic = MDLMaterialSemantic(rawValue: UInt(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLMaterialSemantic: \(rawValue)")
    }
    return semantic
}

private func mdl_bounding_box(_ boundingBox: MDLAxisAlignedBoundingBox) -> [String: Any] {
    [
        "min": [boundingBox.minBounds.x, boundingBox.minBounds.y, boundingBox.minBounds.z],
        "max": [boundingBox.maxBounds.x, boundingBox.maxBounds.y, boundingBox.maxBounds.z],
    ]
}

private func mdl_texture_info(_ texture: MDLTexture) -> [String: Any] {
    var info: [String: Any] = [
        "name": texture.name,
        "dimensions": [texture.dimensions.x, texture.dimensions.y],
        "row_stride": texture.rowStride,
        "channel_count": texture.channelCount,
        "channel_encoding": texture.channelEncoding.rawValue,
        "is_cube": texture.isCube,
        "has_alpha_values": texture.hasAlphaValues,
    ]
    if let urlTexture = texture as? MDLURLTexture {
        info["url"] = urlTexture.url.absoluteString
    }
    return info
}

private func mdl_material_property_info(_ property: MDLMaterialProperty) -> [String: Any] {
    var info: [String: Any] = [
        "name": property.name,
        "semantic": property.semantic.rawValue,
        "property_type": property.type.rawValue,
        "string_value": property.stringValue as Any,
        "url_value": property.urlValue?.absoluteString as Any,
        "float_value": property.floatValue,
        "float2_value": [property.float2Value.x, property.float2Value.y],
        "float3_value": [property.float3Value.x, property.float3Value.y, property.float3Value.z],
        "float4_value": [property.float4Value.x, property.float4Value.y, property.float4Value.z, property.float4Value.w],
        "matrix4x4": [
            property.matrix4x4.columns.0.x, property.matrix4x4.columns.0.y, property.matrix4x4.columns.0.z, property.matrix4x4.columns.0.w,
            property.matrix4x4.columns.1.x, property.matrix4x4.columns.1.y, property.matrix4x4.columns.1.z, property.matrix4x4.columns.1.w,
            property.matrix4x4.columns.2.x, property.matrix4x4.columns.2.y, property.matrix4x4.columns.2.z, property.matrix4x4.columns.2.w,
            property.matrix4x4.columns.3.x, property.matrix4x4.columns.3.y, property.matrix4x4.columns.3.z, property.matrix4x4.columns.3.w,
        ],
        "color": mdl_color_components(property.color) as Any,
        "luminance": property.luminance,
    ]
    if let texture = property.textureSamplerValue?.texture {
        info["texture"] = mdl_texture_info(texture)
    }
    return info
}

private func mdl_mesh_buffer_info(_ buffer: any MDLMeshBuffer) -> [String: Any] {
    [
        "length": buffer.length,
        "buffer_type": buffer.type.rawValue,
    ]
}

private func mdl_vertex_attribute_info(_ attributeData: MDLVertexAttributeData) -> [String: Any] {
    [
        "stride": attributeData.stride,
        "format": attributeData.format.rawValue,
        "buffer_size": attributeData.bufferSize,
    ]
}

@_cdecl("mdl_asset_new_with_url")
public func mdl_asset_new_with_url(
    _ path: UnsafePointer<CChar>?,
    _ outAsset: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let path, let outAsset else {
            throw ModelIOBridgeError.invalidArgument("missing asset path or output pointer")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        outAsset.pointee = mdl_retain(MDLAsset(url: url))
    }
}

@_cdecl("mdl_asset_can_import_file_extension")
public func mdl_asset_can_import_file_extension(_ pathExtension: UnsafePointer<CChar>?) -> Int32 {
    guard let pathExtension else { return 0 }
    return MDLAsset.canImportFileExtension(String(cString: pathExtension)) ? 1 : 0
}

@_cdecl("mdl_asset_count")
public func mdl_asset_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return 0 }
    return UInt64(asset.count)
}

@_cdecl("mdl_asset_bounding_box")
public func mdl_asset_bounding_box(
    _ handle: UnsafeMutableRawPointer?,
    _ outMinX: UnsafeMutablePointer<Float>?,
    _ outMinY: UnsafeMutablePointer<Float>?,
    _ outMinZ: UnsafeMutablePointer<Float>?,
    _ outMaxX: UnsafeMutablePointer<Float>?,
    _ outMaxY: UnsafeMutablePointer<Float>?,
    _ outMaxZ: UnsafeMutablePointer<Float>?
) {
    let zero = MDLAxisAlignedBoundingBox(maxBounds: SIMD3<Float>(repeating: 0), minBounds: SIMD3<Float>(repeating: 0))
    let boundingBox = (mdl_borrow_object(handle) as? MDLAsset)?.boundingBox ?? zero
    outMinX?.pointee = boundingBox.minBounds.x
    outMinY?.pointee = boundingBox.minBounds.y
    outMinZ?.pointee = boundingBox.minBounds.z
    outMaxX?.pointee = boundingBox.maxBounds.x
    outMaxY?.pointee = boundingBox.maxBounds.y
    outMaxZ?.pointee = boundingBox.maxBounds.z
}

@_cdecl("mdl_asset_url_string")
public func mdl_asset_url_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset,
          let url = asset.url
    else {
        return nil
    }
    return mdl_string(url.path)
}

@_cdecl("mdl_asset_mesh_at_index")
public func mdl_asset_mesh_at_index(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset,
          index < UInt64(asset.count),
          let mesh = asset.object(at: Int(index)) as? MDLMesh
    else {
        return nil
    }
    return mdl_retain(mesh)
}

@_cdecl("mdl_asset_load_textures")
public func mdl_asset_load_textures(_ handle: UnsafeMutableRawPointer?) {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return }
    asset.loadTextures()
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

@_cdecl("mdl_submesh_index_count")
public func mdl_submesh_index_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return 0 }
    return UInt64(submesh.indexCount)
}

@_cdecl("mdl_submesh_index_type")
public func mdl_submesh_index_type(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return 0 }
    return UInt32(submesh.indexType.rawValue)
}

@_cdecl("mdl_submesh_geometry_type")
public func mdl_submesh_geometry_type(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return 0 }
    return Int32(submesh.geometryType.rawValue)
}

@_cdecl("mdl_submesh_name_string")
public func mdl_submesh_name_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return nil }
    return mdl_string(submesh.name)
}

@_cdecl("mdl_submesh_index_buffer")
public func mdl_submesh_index_buffer(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return nil }
    return mdl_retain(submesh.indexBuffer as AnyObject)
}

@_cdecl("mdl_submesh_material")
public func mdl_submesh_material(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh,
          let material = submesh.material
    else {
        return nil
    }
    return mdl_retain(material)
}

@_cdecl("mdl_material_count")
public func mdl_material_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial else { return 0 }
    return UInt64(material.count)
}

@_cdecl("mdl_material_name_string")
public func mdl_material_name_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial else { return nil }
    return mdl_string(material.name)
}

@_cdecl("mdl_material_property_at")
public func mdl_material_property_at(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial,
          index < UInt64(material.count),
          let property = material[Int(index)]
    else {
        return nil
    }
    return mdl_retain(property)
}

@_cdecl("mdl_material_property_named")
public func mdl_material_property_named(
    _ handle: UnsafeMutableRawPointer?,
    _ propertyName: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial,
          let propertyName,
          let property = material.propertyNamed(String(cString: propertyName))
    else {
        return nil
    }
    return mdl_retain(property)
}

@_cdecl("mdl_material_property_with_semantic")
public func mdl_material_property_with_semantic(
    _ handle: UnsafeMutableRawPointer?,
    _ semanticRaw: UInt32
) -> UnsafeMutableRawPointer? {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial,
          let semantic = try? mdl_semantic(semanticRaw)
    else {
        return nil
    }
    for index in 0..<material.count {
        if let property = material[index], property.semantic == semantic {
            return mdl_retain(property)
        }
    }
    return nil
}

@_cdecl("mdl_material_property_info_json")
public func mdl_material_property_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return nil }
    return mdl_string(mdl_json_string(from: mdl_material_property_info(property)) ?? "{}")
}

@_cdecl("mdl_material_property_texture")
public func mdl_material_property_texture(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty,
          let texture = property.textureSamplerValue?.texture
    else {
        return nil
    }
    return mdl_retain(texture)
}

@_cdecl("mdl_url_texture_new")
public func mdl_url_texture_new(
    _ path: UnsafePointer<CChar>?,
    _ name: UnsafePointer<CChar>?,
    _ outTexture: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let path, let outTexture else {
            throw ModelIOBridgeError.invalidArgument("missing texture path or output pointer")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        let textureName = name.map { String(cString: $0) }
        outTexture.pointee = mdl_retain(MDLURLTexture(url: url, name: textureName))
    }
}

@_cdecl("mdl_texture_info_json")
public func mdl_texture_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let texture = mdl_borrow_object(handle) as? MDLTexture else { return nil }
    return mdl_string(mdl_json_string(from: mdl_texture_info(texture)) ?? "{}")
}

@_cdecl("mdl_texture_texel_data_length")
public func mdl_texture_texel_data_length(
    _ handle: UnsafeMutableRawPointer?,
    _ topLeftOrigin: Int32
) -> UInt64 {
    guard let texture = mdl_borrow_object(handle) as? MDLTexture else { return 0 }
    let data = topLeftOrigin != 0 ? texture.texelDataWithTopLeftOrigin() : texture.texelDataWithBottomLeftOrigin()
    return UInt64(data?.count ?? 0)
}

@_cdecl("mdl_texture_copy_texel_data")
public func mdl_texture_copy_texel_data(
    _ handle: UnsafeMutableRawPointer?,
    _ topLeftOrigin: Int32,
    _ outBytes: UnsafeMutablePointer<UInt8>?,
    _ capacity: UInt64
) -> UInt64 {
    guard let texture = mdl_borrow_object(handle) as? MDLTexture,
          let outBytes,
          let data = topLeftOrigin != 0 ? texture.texelDataWithTopLeftOrigin() : texture.texelDataWithBottomLeftOrigin()
    else {
        return 0
    }
    let byteCount = min(Int(capacity), data.count)
    data.copyBytes(to: outBytes, count: byteCount)
    return UInt64(byteCount)
}

@_cdecl("mdl_vertex_attribute_data_info_json")
public func mdl_vertex_attribute_data_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attributeData = mdl_borrow_object(handle) as? MDLVertexAttributeData else { return nil }
    return mdl_string(mdl_json_string(from: mdl_vertex_attribute_info(attributeData)) ?? "{}")
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
