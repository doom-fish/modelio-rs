import Foundation
import ModelIO

private func mdl_physically_plausible_light_info(_ light: MDLPhysicallyPlausibleLight) -> [String: Any] {
    [
        "light_type": light.lightType.rawValue,
        "color_space": light.colorSpace,
        "color": mdl_color_components(light.color) as Any,
        "lumens": light.lumens,
        "inner_cone_angle": light.innerConeAngle,
        "outer_cone_angle": light.outerConeAngle,
        "attenuation_start_distance": light.attenuationStartDistance,
        "attenuation_end_distance": light.attenuationEndDistance,
    ]
}

private func mdl_area_light_info(_ light: MDLAreaLight) -> [String: Any] {
    var info = mdl_physically_plausible_light_info(light)
    info["area_radius"] = light.areaRadius
    info["super_elliptic_power"] = [light.superEllipticPower.x, light.superEllipticPower.y]
    info["aspect"] = light.aspect
    return info
}

private func mdl_photometric_light_info(_ light: MDLPhotometricLight) -> [String: Any] {
    var info = mdl_physically_plausible_light_info(light)
    info["spherical_harmonics_level"] = light.sphericalHarmonicsLevel
    info["spherical_harmonics_coefficients_length"] = light.sphericalHarmonicsCoefficients?.count ?? 0
    info["has_light_cube_map"] = light.lightCubeMap != nil
    return info
}

@_cdecl("mdl_physically_plausible_light_new")
public func mdl_physically_plausible_light_new(
    _ outLight: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outLight else {
            throw ModelIOBridgeError.invalidArgument("missing output light pointer")
        }
        outLight.pointee = mdl_retain(MDLPhysicallyPlausibleLight())
    }
}

@_cdecl("mdl_physically_plausible_light_info_json")
public func mdl_physically_plausible_light_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let light = mdl_borrow_object(handle) as? MDLPhysicallyPlausibleLight else { return nil }
    return mdl_string(mdl_json_string(from: mdl_physically_plausible_light_info(light)) ?? "{}")
}

@_cdecl("mdl_physically_plausible_light_set_color_temperature")
public func mdl_physically_plausible_light_set_color_temperature(_ handle: UnsafeMutableRawPointer?, _ temperature: Float) {
    guard let light = mdl_borrow_object(handle) as? MDLPhysicallyPlausibleLight else { return }
    light.setColorByTemperature(temperature)
}

@_cdecl("mdl_physically_plausible_light_set_color")
public func mdl_physically_plausible_light_set_color(
    _ handle: UnsafeMutableRawPointer?,
    _ red: Float,
    _ green: Float,
    _ blue: Float,
    _ alpha: Float
) {
    guard let light = mdl_borrow_object(handle) as? MDLPhysicallyPlausibleLight else { return }
    light.color = mdl_color(red, green, blue, alpha)
}

@_cdecl("mdl_physically_plausible_light_set_lumens")
public func mdl_physically_plausible_light_set_lumens(_ handle: UnsafeMutableRawPointer?, _ lumens: Float) {
    guard let light = mdl_borrow_object(handle) as? MDLPhysicallyPlausibleLight else { return }
    light.lumens = lumens
}

@_cdecl("mdl_physically_plausible_light_set_inner_cone_angle")
public func mdl_physically_plausible_light_set_inner_cone_angle(_ handle: UnsafeMutableRawPointer?, _ angle: Float) {
    guard let light = mdl_borrow_object(handle) as? MDLPhysicallyPlausibleLight else { return }
    light.innerConeAngle = angle
}

@_cdecl("mdl_physically_plausible_light_set_outer_cone_angle")
public func mdl_physically_plausible_light_set_outer_cone_angle(_ handle: UnsafeMutableRawPointer?, _ angle: Float) {
    guard let light = mdl_borrow_object(handle) as? MDLPhysicallyPlausibleLight else { return }
    light.outerConeAngle = angle
}

@_cdecl("mdl_physically_plausible_light_set_attenuation_start_distance")
public func mdl_physically_plausible_light_set_attenuation_start_distance(_ handle: UnsafeMutableRawPointer?, _ distance: Float) {
    guard let light = mdl_borrow_object(handle) as? MDLPhysicallyPlausibleLight else { return }
    light.attenuationStartDistance = distance
}

@_cdecl("mdl_physically_plausible_light_set_attenuation_end_distance")
public func mdl_physically_plausible_light_set_attenuation_end_distance(_ handle: UnsafeMutableRawPointer?, _ distance: Float) {
    guard let light = mdl_borrow_object(handle) as? MDLPhysicallyPlausibleLight else { return }
    light.attenuationEndDistance = distance
}

@_cdecl("mdl_area_light_new")
public func mdl_area_light_new(
    _ outLight: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outLight else {
            throw ModelIOBridgeError.invalidArgument("missing output area light pointer")
        }
        outLight.pointee = mdl_retain(MDLAreaLight())
    }
}

@_cdecl("mdl_area_light_info_json")
public func mdl_area_light_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let light = mdl_borrow_object(handle) as? MDLAreaLight else { return nil }
    return mdl_string(mdl_json_string(from: mdl_area_light_info(light)) ?? "{}")
}

@_cdecl("mdl_area_light_set_area_radius")
public func mdl_area_light_set_area_radius(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let light = mdl_borrow_object(handle) as? MDLAreaLight else { return }
    light.areaRadius = value
}

@_cdecl("mdl_area_light_set_super_elliptic_power")
public func mdl_area_light_set_super_elliptic_power(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float) {
    guard let light = mdl_borrow_object(handle) as? MDLAreaLight else { return }
    light.superEllipticPower = SIMD2<Float>(x, y)
}

@_cdecl("mdl_area_light_set_aspect")
public func mdl_area_light_set_aspect(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let light = mdl_borrow_object(handle) as? MDLAreaLight else { return }
    light.aspect = value
}

@_cdecl("mdl_photometric_light_new")
public func mdl_photometric_light_new(
    _ outLight: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outLight else {
            throw ModelIOBridgeError.invalidArgument("missing output photometric light pointer")
        }
        outLight.pointee = mdl_retain(MDLPhotometricLight())
    }
}

@_cdecl("mdl_photometric_light_new_with_ies_profile")
public func mdl_photometric_light_new_with_ies_profile(
    _ path: UnsafePointer<CChar>?,
    _ outLight: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let path, let outLight else {
            throw ModelIOBridgeError.invalidArgument("missing IES path or output photometric light pointer")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        guard let light = MDLPhotometricLight(iesProfile: url) else {
            throw ModelIOBridgeError.nullResult("MDLPhotometricLight IES initializer returned nil")
        }
        outLight.pointee = mdl_retain(light)
    }
}

@_cdecl("mdl_photometric_light_info_json")
public func mdl_photometric_light_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let light = mdl_borrow_object(handle) as? MDLPhotometricLight else { return nil }
    return mdl_string(mdl_json_string(from: mdl_photometric_light_info(light)) ?? "{}")
}

@_cdecl("mdl_photometric_light_generate_spherical_harmonics_from_light")
public func mdl_photometric_light_generate_spherical_harmonics_from_light(_ handle: UnsafeMutableRawPointer?, _ level: UInt64) {
    guard let light = mdl_borrow_object(handle) as? MDLPhotometricLight else { return }
    light.generateSphericalHarmonics(fromLight: Int(level))
}

@_cdecl("mdl_photometric_light_generate_cubemap_from_light")
public func mdl_photometric_light_generate_cubemap_from_light(_ handle: UnsafeMutableRawPointer?, _ textureSize: UInt64) {
    guard let light = mdl_borrow_object(handle) as? MDLPhotometricLight else { return }
    light.generateCubemap(fromLight: Int(textureSize))
}

@_cdecl("mdl_photometric_light_generate_texture")
public func mdl_photometric_light_generate_texture(_ handle: UnsafeMutableRawPointer?, _ textureSize: UInt64) -> UnsafeMutableRawPointer? {
    guard let light = mdl_borrow_object(handle) as? MDLPhotometricLight else { return nil }
    return mdl_retain(light.generateTexture(Int(textureSize)))
}

@_cdecl("mdl_photometric_light_light_cube_map")
public func mdl_photometric_light_light_cube_map(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let light = mdl_borrow_object(handle) as? MDLPhotometricLight,
          let texture = light.lightCubeMap
    else {
        return nil
    }
    return mdl_retain(texture)
}
