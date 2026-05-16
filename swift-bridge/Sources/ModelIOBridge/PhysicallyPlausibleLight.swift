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
