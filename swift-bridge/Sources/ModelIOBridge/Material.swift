import Foundation
import ModelIO
import simd

private func mdl_texture_info(_ texture: MDLTexture) -> [String: Any] {
    var info: [String: Any] = [
        "name": texture.name,
        "dimensions": [texture.dimensions.x, texture.dimensions.y],
        "row_stride": texture.rowStride,
        "channel_count": texture.channelCount,
        "mip_level_count": texture.mipLevelCount,
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
        "matrix4x4": mdl_matrix_to_array(property.matrix4x4),
        "color": mdl_color_components(property.color) as Any,
        "luminance": property.luminance,
    ]
    if let texture = property.textureSamplerValue?.texture {
        info["texture"] = mdl_texture_info(texture)
    }
    return info
}

private func mdl_material_info(_ material: MDLMaterial) -> [String: Any] {
    [
        "name": material.name,
        "count": material.count,
        "material_face": material.materialFace.rawValue,
    ]
}

@_cdecl("mdl_material_new")
public func mdl_material_new(
    _ name: UnsafePointer<CChar>?,
    _ physicallyPlausible: Int32,
    _ outMaterial: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let name, let outMaterial else {
            throw ModelIOBridgeError.invalidArgument("missing material name or output pointer")
        }
        let materialName = String(cString: name)
        let scatteringFunction: MDLScatteringFunction = physicallyPlausible != 0
            ? MDLPhysicallyPlausibleScatteringFunction()
            : MDLScatteringFunction()
        outMaterial.pointee = mdl_retain(MDLMaterial(name: materialName, scatteringFunction: scatteringFunction))
    }
}

@_cdecl("mdl_material_info_json")
public func mdl_material_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial else { return nil }
    return mdl_string(mdl_json_string(from: mdl_material_info(material)) ?? "{}")
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

@_cdecl("mdl_material_material_face")
public func mdl_material_material_face(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial else { return 0 }
    return UInt32(material.materialFace.rawValue)
}

@_cdecl("mdl_material_set_material_face")
public func mdl_material_set_material_face(_ handle: UnsafeMutableRawPointer?, _ faceRaw: UInt32) {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial,
          let face = try? mdl_material_face(faceRaw)
    else {
        return
    }
    material.materialFace = face
}

@_cdecl("mdl_material_remove_all_properties")
public func mdl_material_remove_all_properties(_ handle: UnsafeMutableRawPointer?) {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial else { return }
    material.removeAllProperties()
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
          let semantic = try? mdl_semantic(semanticRaw),
          let property = material.property(with: semantic)
    else {
        return nil
    }
    return mdl_retain(property)
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

@_cdecl("mdl_material_property_set_float")
public func mdl_material_property_set_float(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return }
    property.floatValue = value
}

@_cdecl("mdl_material_property_set_float2")
public func mdl_material_property_set_float2(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float) {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return }
    property.float2Value = SIMD2<Float>(x, y)
}

@_cdecl("mdl_material_property_set_float3")
public func mdl_material_property_set_float3(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float) {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return }
    property.float3Value = SIMD3<Float>(x, y, z)
}

@_cdecl("mdl_material_property_set_float4")
public func mdl_material_property_set_float4(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float, _ w: Float) {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return }
    property.float4Value = SIMD4<Float>(x, y, z, w)
}

@_cdecl("mdl_material_property_set_matrix4x4")
public func mdl_material_property_set_matrix4x4(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Float>?) {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return }
    property.matrix4x4 = mdl_matrix_from_array(values)
}

@_cdecl("mdl_material_property_set_string")
public func mdl_material_property_set_string(_ handle: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?) {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return }
    property.stringValue = value.map { String(cString: $0) }
}

@_cdecl("mdl_material_property_set_url")
public func mdl_material_property_set_url(_ handle: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?) {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return }
    property.urlValue = value.map { URL(fileURLWithPath: String(cString: $0)) }
}

@_cdecl("mdl_material_property_set_color")
public func mdl_material_property_set_color(
    _ handle: UnsafeMutableRawPointer?,
    _ red: Float,
    _ green: Float,
    _ blue: Float,
    _ alpha: Float
) {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return }
    property.color = mdl_color(red, green, blue, alpha)
}

@_cdecl("mdl_material_property_set_luminance")
public func mdl_material_property_set_luminance(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return }
    property.luminance = value
}
