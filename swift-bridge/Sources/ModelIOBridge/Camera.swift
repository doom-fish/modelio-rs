import Foundation
import ModelIO
import simd

private func mdl_camera_info(_ camera: MDLCamera) -> [String: Any] {
    [
        "projection": camera.projection.rawValue,
        "projection_matrix": mdl_matrix_to_array(camera.projectionMatrix),
        "near_visibility_distance": camera.nearVisibilityDistance,
        "far_visibility_distance": camera.farVisibilityDistance,
        "world_to_meters_conversion_scale": camera.worldToMetersConversionScale,
        "barrel_distortion": camera.barrelDistortion,
        "fisheye_distortion": camera.fisheyeDistortion,
        "optical_vignetting": camera.opticalVignetting,
        "chromatic_aberration": camera.chromaticAberration,
        "focal_length": camera.focalLength,
        "focus_distance": camera.focusDistance,
        "field_of_view": camera.fieldOfView,
        "f_stop": camera.fStop,
        "aperture_blade_count": camera.apertureBladeCount,
        "maximum_circle_of_confusion": camera.maximumCircleOfConfusion,
        "shutter_open_interval": camera.shutterOpenInterval,
        "sensor_vertical_aperture": camera.sensorVerticalAperture,
        "sensor_aspect": camera.sensorAspect,
        "sensor_enlargement": [camera.sensorEnlargement.x, camera.sensorEnlargement.y],
        "sensor_shift": [camera.sensorShift.x, camera.sensorShift.y],
        "flash": [camera.flash.x, camera.flash.y, camera.flash.z],
        "exposure_compression": [camera.exposureCompression.x, camera.exposureCompression.y],
        "exposure": [camera.exposure.x, camera.exposure.y, camera.exposure.z],
    ]
}

@_cdecl("mdl_camera_new")
public func mdl_camera_new(
    _ outCamera: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outCamera else {
            throw ModelIOBridgeError.invalidArgument("missing output camera pointer")
        }
        outCamera.pointee = mdl_retain(MDLCamera())
    }
}

@_cdecl("mdl_camera_info_json")
public func mdl_camera_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return nil }
    return mdl_string(mdl_json_string(from: mdl_camera_info(camera)) ?? "{}")
}

@_cdecl("mdl_camera_set_projection")
public func mdl_camera_set_projection(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera,
          let projection = try? mdl_camera_projection(rawValue)
    else {
        return
    }
    camera.projection = projection
}

@_cdecl("mdl_camera_set_near_visibility_distance")
public func mdl_camera_set_near_visibility_distance(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return }
    camera.nearVisibilityDistance = value
}

@_cdecl("mdl_camera_set_far_visibility_distance")
public func mdl_camera_set_far_visibility_distance(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return }
    camera.farVisibilityDistance = value
}

@_cdecl("mdl_camera_set_world_to_meters_conversion_scale")
public func mdl_camera_set_world_to_meters_conversion_scale(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return }
    camera.worldToMetersConversionScale = value
}

@_cdecl("mdl_camera_set_focal_length")
public func mdl_camera_set_focal_length(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return }
    camera.focalLength = value
}

@_cdecl("mdl_camera_set_focus_distance")
public func mdl_camera_set_focus_distance(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return }
    camera.focusDistance = value
}

@_cdecl("mdl_camera_set_field_of_view")
public func mdl_camera_set_field_of_view(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return }
    camera.fieldOfView = value
}

@_cdecl("mdl_camera_look_at")
public func mdl_camera_look_at(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return }
    camera.look(at: SIMD3<Float>(x, y, z))
}

@_cdecl("mdl_camera_look_at_from")
public func mdl_camera_look_at_from(
    _ handle: UnsafeMutableRawPointer?,
    _ focusX: Float,
    _ focusY: Float,
    _ focusZ: Float,
    _ cameraX: Float,
    _ cameraY: Float,
    _ cameraZ: Float
) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return }
    camera.look(at: SIMD3<Float>(focusX, focusY, focusZ), from: SIMD3<Float>(cameraX, cameraY, cameraZ))
}

@_cdecl("mdl_camera_frame_bounding_box")
public func mdl_camera_frame_bounding_box(
    _ handle: UnsafeMutableRawPointer?,
    _ minX: Float,
    _ minY: Float,
    _ minZ: Float,
    _ maxX: Float,
    _ maxY: Float,
    _ maxZ: Float,
    _ setNearAndFar: Int32
) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return }
    let boundingBox = MDLAxisAlignedBoundingBox(maxBounds: SIMD3<Float>(maxX, maxY, maxZ), minBounds: SIMD3<Float>(minX, minY, minZ))
    camera.frameBoundingBox(boundingBox, setNearAndFar: setNearAndFar != 0)
}

@_cdecl("mdl_camera_ray_to")
public func mdl_camera_ray_to(
    _ handle: UnsafeMutableRawPointer?,
    _ pixelX: Int32,
    _ pixelY: Int32,
    _ viewportWidth: Int32,
    _ viewportHeight: Int32,
    _ outX: UnsafeMutablePointer<Float>?,
    _ outY: UnsafeMutablePointer<Float>?,
    _ outZ: UnsafeMutablePointer<Float>?
) {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else {
        outX?.pointee = 0
        outY?.pointee = 0
        outZ?.pointee = 0
        return
    }
    let ray = camera.ray(to: vector_int2(pixelX, pixelY), forViewPort: vector_int2(viewportWidth, viewportHeight))
    outX?.pointee = ray.x
    outY?.pointee = ray.y
    outZ?.pointee = ray.z
}

private func mdl_stereoscopic_camera_info(_ camera: MDLStereoscopicCamera) -> [String: Any] {
    [
        "inter_pupillary_distance": camera.interPupillaryDistance,
        "left_vergence": camera.leftVergence,
        "right_vergence": camera.rightVergence,
        "overlap": camera.overlap,
        "left_view_matrix": mdl_matrix_to_array(camera.leftViewMatrix),
        "right_view_matrix": mdl_matrix_to_array(camera.rightViewMatrix),
        "left_projection_matrix": mdl_matrix_to_array(camera.leftProjectionMatrix),
        "right_projection_matrix": mdl_matrix_to_array(camera.rightProjectionMatrix),
    ]
}

@_cdecl("mdl_camera_bokeh_kernel")
public func mdl_camera_bokeh_kernel(
    _ handle: UnsafeMutableRawPointer?,
    _ width: Int32,
    _ height: Int32
) -> UnsafeMutableRawPointer? {
    guard let camera = mdl_borrow_object(handle) as? MDLCamera else { return nil }
    return mdl_retain(camera.bokehKernel(withSize: vector_int2(width, height)))
}

@_cdecl("mdl_stereoscopic_camera_new")
public func mdl_stereoscopic_camera_new(
    _ outCamera: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outCamera else {
            throw ModelIOBridgeError.invalidArgument("missing output stereoscopic camera pointer")
        }
        outCamera.pointee = mdl_retain(MDLStereoscopicCamera())
    }
}

@_cdecl("mdl_stereoscopic_camera_info_json")
public func mdl_stereoscopic_camera_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let camera = mdl_borrow_object(handle) as? MDLStereoscopicCamera else { return nil }
    return mdl_string(mdl_json_string(from: mdl_stereoscopic_camera_info(camera)) ?? "{}")
}

@_cdecl("mdl_stereoscopic_camera_set_inter_pupillary_distance")
public func mdl_stereoscopic_camera_set_inter_pupillary_distance(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLStereoscopicCamera else { return }
    camera.interPupillaryDistance = value
}

@_cdecl("mdl_stereoscopic_camera_set_left_vergence")
public func mdl_stereoscopic_camera_set_left_vergence(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLStereoscopicCamera else { return }
    camera.leftVergence = value
}

@_cdecl("mdl_stereoscopic_camera_set_right_vergence")
public func mdl_stereoscopic_camera_set_right_vergence(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLStereoscopicCamera else { return }
    camera.rightVergence = value
}

@_cdecl("mdl_stereoscopic_camera_set_overlap")
public func mdl_stereoscopic_camera_set_overlap(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let camera = mdl_borrow_object(handle) as? MDLStereoscopicCamera else { return }
    camera.overlap = value
}
