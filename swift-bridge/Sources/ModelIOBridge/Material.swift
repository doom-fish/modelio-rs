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

private func mdl_named(_ handle: UnsafeMutableRawPointer?) -> (NSObject & MDLNamed)? {
    mdl_borrow_object(handle) as? (NSObject & MDLNamed)
}

private func mdl_scattering_function(_ handle: UnsafeMutableRawPointer?) -> MDLScatteringFunction? {
    mdl_borrow_object(handle) as? MDLScatteringFunction
}

private func mdl_physically_plausible_scattering_function(
    _ handle: UnsafeMutableRawPointer?
) -> MDLPhysicallyPlausibleScatteringFunction? {
    mdl_borrow_object(handle) as? MDLPhysicallyPlausibleScatteringFunction
}

private func mdl_scattering_property(
    _ scatteringFunction: MDLScatteringFunction,
    code: UInt32
) -> MDLMaterialProperty? {
    switch code {
    case 1: return scatteringFunction.baseColor
    case 2: return scatteringFunction.emission
    case 3: return scatteringFunction.specular
    case 4: return scatteringFunction.materialIndexOfRefraction
    case 5: return scatteringFunction.interfaceIndexOfRefraction
    case 6: return scatteringFunction.normal
    case 7: return scatteringFunction.ambientOcclusion
    case 8: return scatteringFunction.ambientOcclusionScale
    default: return nil
    }
}

private func mdl_physically_plausible_scattering_property(
    _ scatteringFunction: MDLPhysicallyPlausibleScatteringFunction,
    code: UInt32
) -> MDLMaterialProperty? {
    switch code {
    case 1: return scatteringFunction.subsurface
    case 2: return scatteringFunction.metallic
    case 3: return scatteringFunction.specularAmount
    case 4: return scatteringFunction.specularTint
    case 5: return scatteringFunction.roughness
    case 6: return scatteringFunction.anisotropic
    case 7: return scatteringFunction.anisotropicRotation
    case 8: return scatteringFunction.sheen
    case 9: return scatteringFunction.sheenTint
    case 10: return scatteringFunction.clearcoat
    case 11: return scatteringFunction.clearcoatGloss
    default: return nil
    }
}

private func mdl_texture_wrap_mode(_ rawValue: UInt32) -> MDLMaterialTextureWrapMode? {
    switch rawValue {
    case 0: return .clamp
    case 1: return .repeat
    case 2: return .mirror
    default: return nil
    }
}

private func mdl_texture_filter_mode(_ rawValue: UInt32) -> MDLMaterialTextureFilterMode? {
    switch rawValue {
    case 0: return .nearest
    case 1: return .linear
    default: return nil
    }
}

private func mdl_mip_map_filter_mode(_ rawValue: UInt32) -> MDLMaterialMipMapFilterMode? {
    switch rawValue {
    case 0: return .nearest
    case 1: return .linear
    default: return nil
    }
}

private func mdl_texture_filter_info(_ filter: MDLTextureFilter) -> [String: Any] {
    [
        "s_wrap_mode": filter.sWrapMode.rawValue,
        "t_wrap_mode": filter.tWrapMode.rawValue,
        "r_wrap_mode": filter.rWrapMode.rawValue,
        "min_filter": filter.minFilter.rawValue,
        "mag_filter": filter.magFilter.rawValue,
        "mip_filter": filter.mipFilter.rawValue,
    ]
}

private func mdl_texture_sampler_info(_ sampler: MDLTextureSampler) -> [String: Any] {
    [
        "has_texture": sampler.texture != nil,
        "has_hardware_filter": sampler.hardwareFilter != nil,
        "has_transform": sampler.transform != nil,
    ]
}

private func mdl_object_array<T: AnyObject>(_ values: UnsafePointer<UnsafeMutableRawPointer?>?, count: UInt64, as _: T.Type) -> [T] {
    guard let values, count > 0 else { return [] }
    return (0..<Int(count)).compactMap { index in
        mdl_borrow_object(values[index]) as? T
    }
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

@_cdecl("mdl_material_new_with_scattering_function")
public func mdl_material_new_with_scattering_function(
    _ name: UnsafePointer<CChar>?,
    _ scatteringFunctionHandle: UnsafeMutableRawPointer?,
    _ outMaterial: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let name,
              let scatteringFunction = mdl_scattering_function(scatteringFunctionHandle),
              let outMaterial
        else {
            throw ModelIOBridgeError.invalidArgument("missing material name, scattering function, or output pointer")
        }
        outMaterial.pointee = mdl_retain(
            MDLMaterial(
                name: String(cString: name),
                scatteringFunction: scatteringFunction
            )
        )
    }
}

@_cdecl("mdl_material_scattering_function")
public func mdl_material_scattering_function(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial else { return nil }
    return mdl_retain(material.scatteringFunction)
}

@_cdecl("mdl_scattering_function_new")
public func mdl_scattering_function_new(
    _ outScatteringFunction: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outScatteringFunction else {
            throw ModelIOBridgeError.invalidArgument("missing output scattering function pointer")
        }
        outScatteringFunction.pointee = mdl_retain(MDLScatteringFunction())
    }
}

@_cdecl("mdl_scattering_function_property")
public func mdl_scattering_function_property(
    _ handle: UnsafeMutableRawPointer?,
    _ code: UInt32
) -> UnsafeMutableRawPointer? {
    guard let scatteringFunction = mdl_scattering_function(handle),
          let property = mdl_scattering_property(scatteringFunction, code: code)
    else {
        return nil
    }
    return mdl_retain(property)
}

@_cdecl("mdl_scattering_function_is_physically_plausible")
public func mdl_scattering_function_is_physically_plausible(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let scatteringFunction = mdl_borrow_object(handle) else { return 0 }
    return scatteringFunction is MDLPhysicallyPlausibleScatteringFunction ? 1 : 0
}

@_cdecl("mdl_physically_plausible_scattering_function_new")
public func mdl_physically_plausible_scattering_function_new(
    _ outScatteringFunction: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outScatteringFunction else {
            throw ModelIOBridgeError.invalidArgument("missing output scattering function pointer")
        }
        outScatteringFunction.pointee = mdl_retain(MDLPhysicallyPlausibleScatteringFunction())
    }
}

@_cdecl("mdl_physically_plausible_scattering_function_version")
public func mdl_physically_plausible_scattering_function_version(_ handle: UnsafeMutableRawPointer?) -> Int64 {
    guard let scatteringFunction = mdl_physically_plausible_scattering_function(handle) else {
        return 0
    }
    return Int64(scatteringFunction.version)
}

@_cdecl("mdl_physically_plausible_scattering_function_property")
public func mdl_physically_plausible_scattering_function_property(
    _ handle: UnsafeMutableRawPointer?,
    _ code: UInt32
) -> UnsafeMutableRawPointer? {
    guard let scatteringFunction = mdl_physically_plausible_scattering_function(handle),
          let property = mdl_physically_plausible_scattering_property(scatteringFunction, code: code)
    else {
        return nil
    }
    return mdl_retain(property)
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

@_cdecl("mdl_material_set_name")
public func mdl_material_set_name(_ handle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial,
          let name
    else {
        return
    }
    material.name = String(cString: name)
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

@_cdecl("mdl_material_load_textures_using_resolver")
public func mdl_material_load_textures_using_resolver(
    _ handle: UnsafeMutableRawPointer?,
    _ resolverHandle: UnsafeMutableRawPointer?
) {
    guard let material = mdl_borrow_object(handle) as? MDLMaterial,
          let resolver = mdl_borrow_object(resolverHandle) as? any MDLAssetResolver
    else {
        return
    }
    material.loadTextures(using: resolver)
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

@_cdecl("mdl_material_property_new")
public func mdl_material_property_new(
    _ name: UnsafePointer<CChar>?,
    _ semanticRaw: UInt32,
    _ outProperty: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let name,
              let semantic = try? mdl_semantic(semanticRaw),
              let outProperty
        else {
            throw ModelIOBridgeError.invalidArgument("missing material property name, semantic, or output pointer")
        }
        outProperty.pointee = mdl_retain(MDLMaterialProperty(name: String(cString: name), semantic: semantic))
    }
}

@_cdecl("mdl_named_name_string")
public func mdl_named_name_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let named = mdl_named(handle) else { return nil }
    return mdl_string(named.name)
}

@_cdecl("mdl_named_set_name")
public func mdl_named_set_name(_ handle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let named = mdl_named(handle),
          let name
    else {
        return
    }
    named.name = String(cString: name)
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

@_cdecl("mdl_material_property_texture_sampler")
public func mdl_material_property_texture_sampler(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty,
          let sampler = property.textureSamplerValue
    else {
        return nil
    }
    return mdl_retain(sampler)
}

@_cdecl("mdl_material_property_set_texture_sampler")
public func mdl_material_property_set_texture_sampler(_ handle: UnsafeMutableRawPointer?, _ samplerHandle: UnsafeMutableRawPointer?) {
    guard let property = mdl_borrow_object(handle) as? MDLMaterialProperty else { return }
    property.textureSamplerValue = mdl_borrow_object(samplerHandle) as? MDLTextureSampler
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

@_cdecl("mdl_texture_filter_new")
public func mdl_texture_filter_new(
    _ outFilter: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outFilter else {
            throw ModelIOBridgeError.invalidArgument("missing output texture filter pointer")
        }
        outFilter.pointee = mdl_retain(MDLTextureFilter())
    }
}

@_cdecl("mdl_texture_filter_info_json")
public func mdl_texture_filter_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let filter = mdl_borrow_object(handle) as? MDLTextureFilter else { return nil }
    return mdl_string(mdl_json_string(from: mdl_texture_filter_info(filter)) ?? "{}")
}

@_cdecl("mdl_texture_filter_set_s_wrap_mode")
public func mdl_texture_filter_set_s_wrap_mode(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) {
    guard let filter = mdl_borrow_object(handle) as? MDLTextureFilter,
          let mode = mdl_texture_wrap_mode(rawValue)
    else {
        return
    }
    filter.sWrapMode = mode
}

@_cdecl("mdl_texture_filter_set_t_wrap_mode")
public func mdl_texture_filter_set_t_wrap_mode(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) {
    guard let filter = mdl_borrow_object(handle) as? MDLTextureFilter,
          let mode = mdl_texture_wrap_mode(rawValue)
    else {
        return
    }
    filter.tWrapMode = mode
}

@_cdecl("mdl_texture_filter_set_r_wrap_mode")
public func mdl_texture_filter_set_r_wrap_mode(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) {
    guard let filter = mdl_borrow_object(handle) as? MDLTextureFilter,
          let mode = mdl_texture_wrap_mode(rawValue)
    else {
        return
    }
    filter.rWrapMode = mode
}

@_cdecl("mdl_texture_filter_set_min_filter")
public func mdl_texture_filter_set_min_filter(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) {
    guard let filter = mdl_borrow_object(handle) as? MDLTextureFilter,
          let mode = mdl_texture_filter_mode(rawValue)
    else {
        return
    }
    filter.minFilter = mode
}

@_cdecl("mdl_texture_filter_set_mag_filter")
public func mdl_texture_filter_set_mag_filter(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) {
    guard let filter = mdl_borrow_object(handle) as? MDLTextureFilter,
          let mode = mdl_texture_filter_mode(rawValue)
    else {
        return
    }
    filter.magFilter = mode
}

@_cdecl("mdl_texture_filter_set_mip_filter")
public func mdl_texture_filter_set_mip_filter(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) {
    guard let filter = mdl_borrow_object(handle) as? MDLTextureFilter,
          let mode = mdl_mip_map_filter_mode(rawValue)
    else {
        return
    }
    filter.mipFilter = mode
}

@_cdecl("mdl_texture_sampler_new")
public func mdl_texture_sampler_new(
    _ outSampler: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outSampler else {
            throw ModelIOBridgeError.invalidArgument("missing output texture sampler pointer")
        }
        outSampler.pointee = mdl_retain(MDLTextureSampler())
    }
}

@_cdecl("mdl_texture_sampler_info_json")
public func mdl_texture_sampler_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let sampler = mdl_borrow_object(handle) as? MDLTextureSampler else { return nil }
    return mdl_string(mdl_json_string(from: mdl_texture_sampler_info(sampler)) ?? "{}")
}

@_cdecl("mdl_texture_sampler_texture")
public func mdl_texture_sampler_texture(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let sampler = mdl_borrow_object(handle) as? MDLTextureSampler,
          let texture = sampler.texture
    else {
        return nil
    }
    return mdl_retain(texture)
}

@_cdecl("mdl_texture_sampler_set_texture")
public func mdl_texture_sampler_set_texture(_ handle: UnsafeMutableRawPointer?, _ textureHandle: UnsafeMutableRawPointer?) {
    guard let sampler = mdl_borrow_object(handle) as? MDLTextureSampler else { return }
    sampler.texture = mdl_borrow_object(textureHandle) as? MDLTexture
}

@_cdecl("mdl_texture_sampler_hardware_filter")
public func mdl_texture_sampler_hardware_filter(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let sampler = mdl_borrow_object(handle) as? MDLTextureSampler,
          let filter = sampler.hardwareFilter
    else {
        return nil
    }
    return mdl_retain(filter)
}

@_cdecl("mdl_texture_sampler_set_hardware_filter")
public func mdl_texture_sampler_set_hardware_filter(_ handle: UnsafeMutableRawPointer?, _ filterHandle: UnsafeMutableRawPointer?) {
    guard let sampler = mdl_borrow_object(handle) as? MDLTextureSampler else { return }
    sampler.hardwareFilter = mdl_borrow_object(filterHandle) as? MDLTextureFilter
}

@_cdecl("mdl_texture_sampler_transform")
public func mdl_texture_sampler_transform(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let sampler = mdl_borrow_object(handle) as? MDLTextureSampler,
          let transform = sampler.transform
    else {
        return nil
    }
    return mdl_retain(transform)
}

@_cdecl("mdl_texture_sampler_set_transform")
public func mdl_texture_sampler_set_transform(_ handle: UnsafeMutableRawPointer?, _ transformHandle: UnsafeMutableRawPointer?) {
    guard let sampler = mdl_borrow_object(handle) as? MDLTextureSampler else { return }
    sampler.transform = mdl_borrow_object(transformHandle) as? MDLTransform
}

@_cdecl("mdl_material_property_connection_new")
public func mdl_material_property_connection_new(
    _ outputHandle: UnsafeMutableRawPointer?,
    _ inputHandle: UnsafeMutableRawPointer?,
    _ outConnection: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let output = mdl_borrow_object(outputHandle) as? MDLMaterialProperty,
              let input = mdl_borrow_object(inputHandle) as? MDLMaterialProperty,
              let outConnection
        else {
            throw ModelIOBridgeError.invalidArgument("missing material properties or output connection pointer")
        }
        outConnection.pointee = mdl_retain(MDLMaterialPropertyConnection(output: output, input: input))
    }
}

@_cdecl("mdl_material_property_connection_output")
public func mdl_material_property_connection_output(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let connection = mdl_borrow_object(handle) as? MDLMaterialPropertyConnection,
          let output = connection.output
    else {
        return nil
    }
    return mdl_retain(output)
}

@_cdecl("mdl_material_property_connection_input")
public func mdl_material_property_connection_input(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let connection = mdl_borrow_object(handle) as? MDLMaterialPropertyConnection,
          let input = connection.input
    else {
        return nil
    }
    return mdl_retain(input)
}

@_cdecl("mdl_material_property_node_new")
public func mdl_material_property_node_new(
    _ inputHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ inputCount: UInt64,
    _ outputHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ outputCount: UInt64,
    _ outNode: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outNode else {
            throw ModelIOBridgeError.invalidArgument("missing output node pointer")
        }
        let inputs = mdl_object_array(inputHandles, count: inputCount, as: MDLMaterialProperty.self)
        let outputs = mdl_object_array(outputHandles, count: outputCount, as: MDLMaterialProperty.self)
        outNode.pointee = mdl_retain(MDLMaterialPropertyNode(inputs: inputs, outputs: outputs) { _ in })
    }
}

@_cdecl("mdl_material_property_node_inputs")
public func mdl_material_property_node_inputs(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node = mdl_borrow_object(handle) as? MDLMaterialPropertyNode else { return nil }
    return mdl_retain(node.inputs as NSArray)
}

@_cdecl("mdl_material_property_node_outputs")
public func mdl_material_property_node_outputs(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node = mdl_borrow_object(handle) as? MDLMaterialPropertyNode else { return nil }
    return mdl_retain(node.outputs as NSArray)
}

@_cdecl("mdl_material_property_graph_new")
public func mdl_material_property_graph_new(
    _ nodeHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ nodeCount: UInt64,
    _ connectionHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ connectionCount: UInt64,
    _ outGraph: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outGraph else {
            throw ModelIOBridgeError.invalidArgument("missing output graph pointer")
        }
        let nodes = mdl_object_array(nodeHandles, count: nodeCount, as: MDLMaterialPropertyNode.self)
        let connections = mdl_object_array(connectionHandles, count: connectionCount, as: MDLMaterialPropertyConnection.self)
        outGraph.pointee = mdl_retain(MDLMaterialPropertyGraph(nodes: nodes, connections: connections))
    }
}

@_cdecl("mdl_material_property_graph_evaluate")
public func mdl_material_property_graph_evaluate(_ handle: UnsafeMutableRawPointer?) {
    guard let graph = mdl_borrow_object(handle) as? MDLMaterialPropertyGraph else { return }
    graph.evaluate()
}

@_cdecl("mdl_material_property_graph_nodes")
public func mdl_material_property_graph_nodes(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let graph = mdl_borrow_object(handle) as? MDLMaterialPropertyGraph else { return nil }
    return mdl_retain(graph.nodes as NSArray)
}

@_cdecl("mdl_material_property_graph_connections")
public func mdl_material_property_graph_connections(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let graph = mdl_borrow_object(handle) as? MDLMaterialPropertyGraph else { return nil }
    return mdl_retain(graph.connections as NSArray)
}
