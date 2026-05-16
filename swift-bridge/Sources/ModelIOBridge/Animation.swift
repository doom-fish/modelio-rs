import Foundation
import ModelIO
import simd

private func mdl_double_matrix_to_float_array(_ matrix: matrix_double4x4) -> [Float] {
    [
        Float(matrix.columns.0.x), Float(matrix.columns.0.y), Float(matrix.columns.0.z), Float(matrix.columns.0.w),
        Float(matrix.columns.1.x), Float(matrix.columns.1.y), Float(matrix.columns.1.z), Float(matrix.columns.1.w),
        Float(matrix.columns.2.x), Float(matrix.columns.2.y), Float(matrix.columns.2.z), Float(matrix.columns.2.w),
        Float(matrix.columns.3.x), Float(matrix.columns.3.y), Float(matrix.columns.3.z), Float(matrix.columns.3.w),
    ]
}

private func mdl_float_array_to_double_matrix(_ values: UnsafePointer<Float>?) -> matrix_double4x4 {
    let matrix = mdl_matrix_from_array(values)
    return matrix_double4x4(columns: (
        SIMD4<Double>(Double(matrix.columns.0.x), Double(matrix.columns.0.y), Double(matrix.columns.0.z), Double(matrix.columns.0.w)),
        SIMD4<Double>(Double(matrix.columns.1.x), Double(matrix.columns.1.y), Double(matrix.columns.1.z), Double(matrix.columns.1.w)),
        SIMD4<Double>(Double(matrix.columns.2.x), Double(matrix.columns.2.y), Double(matrix.columns.2.z), Double(matrix.columns.2.w)),
        SIMD4<Double>(Double(matrix.columns.3.x), Double(matrix.columns.3.y), Double(matrix.columns.3.z), Double(matrix.columns.3.w))
    ))
}

private func mdl_packed_joint_animation_info(_ animation: MDLPackedJointAnimation) -> [String: Any] {
    [
        "name": animation.name,
        "path": animation.path,
        "joint_paths": animation.jointPaths,
        "joint_count": animation.jointPaths.count,
    ]
}

private func mdl_animation_bind_component_info(_ component: MDLAnimationBindComponent) -> [String: Any] {
    [
        "has_skeleton": component.skeleton != nil,
        "has_joint_animation": component.jointAnimation != nil,
        "joint_paths": component.jointPaths as Any,
        "geometry_bind_transform": mdl_double_matrix_to_float_array(component.geometryBindTransform),
    ]
}

@_cdecl("mdl_packed_joint_animation_new")
public func mdl_packed_joint_animation_new(
    _ name: UnsafePointer<CChar>?,
    _ jointPaths: UnsafePointer<UnsafePointer<CChar>?>?,
    _ jointPathCount: UInt64,
    _ outAnimation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let name, let outAnimation else {
            throw ModelIOBridgeError.invalidArgument("missing animation name or output pointer")
        }
        outAnimation.pointee = mdl_retain(
            MDLPackedJointAnimation(
                name: String(cString: name),
                jointPaths: mdl_string_array(jointPaths, count: jointPathCount)
            )
        )
    }
}

@_cdecl("mdl_packed_joint_animation_info_json")
public func mdl_packed_joint_animation_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let animation = mdl_borrow_object(handle) as? MDLPackedJointAnimation else { return nil }
    return mdl_string(mdl_json_string(from: mdl_packed_joint_animation_info(animation)) ?? "{}")
}

@_cdecl("mdl_packed_joint_animation_translations")
public func mdl_packed_joint_animation_translations(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let animation = mdl_borrow_object(handle) as? MDLPackedJointAnimation else { return nil }
    return mdl_retain(animation.translations)
}

@_cdecl("mdl_packed_joint_animation_rotations")
public func mdl_packed_joint_animation_rotations(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let animation = mdl_borrow_object(handle) as? MDLPackedJointAnimation else { return nil }
    return mdl_retain(animation.rotations)
}

@_cdecl("mdl_packed_joint_animation_scales")
public func mdl_packed_joint_animation_scales(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let animation = mdl_borrow_object(handle) as? MDLPackedJointAnimation else { return nil }
    return mdl_retain(animation.scales)
}

@_cdecl("mdl_animation_bind_component_new")
public func mdl_animation_bind_component_new(
    _ outComponent: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outComponent else {
            throw ModelIOBridgeError.invalidArgument("missing output animation bind component pointer")
        }
        outComponent.pointee = mdl_retain(MDLAnimationBindComponent())
    }
}

@_cdecl("mdl_animation_bind_component_info_json")
public func mdl_animation_bind_component_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let component = mdl_borrow_object(handle) as? MDLAnimationBindComponent else { return nil }
    return mdl_string(mdl_json_string(from: mdl_animation_bind_component_info(component)) ?? "{}")
}

@_cdecl("mdl_animation_bind_component_set_skeleton")
public func mdl_animation_bind_component_set_skeleton(_ handle: UnsafeMutableRawPointer?, _ skeletonHandle: UnsafeMutableRawPointer?) {
    guard let component = mdl_borrow_object(handle) as? MDLAnimationBindComponent else { return }
    component.skeleton = mdl_borrow_object(skeletonHandle) as? MDLSkeleton
}

@_cdecl("mdl_animation_bind_component_skeleton")
public func mdl_animation_bind_component_skeleton(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let component = mdl_borrow_object(handle) as? MDLAnimationBindComponent,
          let skeleton = component.skeleton
    else {
        return nil
    }
    return mdl_retain(skeleton)
}

@_cdecl("mdl_animation_bind_component_set_packed_joint_animation")
public func mdl_animation_bind_component_set_packed_joint_animation(_ handle: UnsafeMutableRawPointer?, _ animationHandle: UnsafeMutableRawPointer?) {
    guard let component = mdl_borrow_object(handle) as? MDLAnimationBindComponent else { return }
    component.jointAnimation = mdl_borrow_object(animationHandle) as? MDLPackedJointAnimation
}

@_cdecl("mdl_animation_bind_component_packed_joint_animation")
public func mdl_animation_bind_component_packed_joint_animation(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let component = mdl_borrow_object(handle) as? MDLAnimationBindComponent,
          let animation = component.jointAnimation as? MDLPackedJointAnimation
    else {
        return nil
    }
    return mdl_retain(animation)
}

@_cdecl("mdl_animation_bind_component_set_joint_paths")
public func mdl_animation_bind_component_set_joint_paths(
    _ handle: UnsafeMutableRawPointer?,
    _ jointPaths: UnsafePointer<UnsafePointer<CChar>?>?,
    _ jointPathCount: UInt64
) {
    guard let component = mdl_borrow_object(handle) as? MDLAnimationBindComponent else { return }
    component.jointPaths = mdl_string_array(jointPaths, count: jointPathCount)
}

@_cdecl("mdl_animation_bind_component_set_geometry_bind_transform")
public func mdl_animation_bind_component_set_geometry_bind_transform(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Float>?) {
    guard let component = mdl_borrow_object(handle) as? MDLAnimationBindComponent else { return }
    component.geometryBindTransform = mdl_float_array_to_double_matrix(values)
}
