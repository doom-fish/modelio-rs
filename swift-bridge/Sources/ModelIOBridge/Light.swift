import CoreGraphics
import Foundation
import ModelIO
import simd

private func mdl_light_info(_ light: MDLLight) -> [String: Any] {
    [
        "light_type": light.lightType.rawValue,
        "color_space": light.colorSpace,
    ]
}

@_cdecl("mdl_light_new")
public func mdl_light_new(
    _ outLight: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outLight else {
            throw ModelIOBridgeError.invalidArgument("missing output light pointer")
        }
        outLight.pointee = mdl_retain(MDLLight())
    }
}

@_cdecl("mdl_light_info_json")
public func mdl_light_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let light = mdl_borrow_object(handle) as? MDLLight else { return nil }
    return mdl_string(mdl_json_string(from: mdl_light_info(light)) ?? "{}")
}

@_cdecl("mdl_light_set_light_type")
public func mdl_light_set_light_type(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) {
    guard let light = mdl_borrow_object(handle) as? MDLLight,
          let lightType = try? mdl_light_type(rawValue)
    else {
        return
    }
    light.lightType = lightType
}

@_cdecl("mdl_light_set_color_space")
public func mdl_light_set_color_space(_ handle: UnsafeMutableRawPointer?, _ colorSpace: UnsafePointer<CChar>?) {
    guard let light = mdl_borrow_object(handle) as? MDLLight,
          let colorSpace
    else {
        return
    }
    light.colorSpace = String(cString: colorSpace)
}

@_cdecl("mdl_light_irradiance_at_point")
public func mdl_light_irradiance_at_point(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ outR: UnsafeMutablePointer<Float>?,
    _ outG: UnsafeMutablePointer<Float>?,
    _ outB: UnsafeMutablePointer<Float>?,
    _ outA: UnsafeMutablePointer<Float>?
) {
    guard let light = mdl_borrow_object(handle) as? MDLLight else {
        outR?.pointee = 0
        outG?.pointee = 0
        outB?.pointee = 0
        outA?.pointee = 0
        return
    }
    let irradiance = light.irradiance(atPoint: SIMD3<Float>(x, y, z)).takeUnretainedValue()
    let components = mdl_color_components(irradiance) ?? [0, 0, 0, 0]
    outR?.pointee = components[0]
    outG?.pointee = components[1]
    outB?.pointee = components[2]
    outA?.pointee = components[3]
}
