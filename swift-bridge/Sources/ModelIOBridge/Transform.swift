import Foundation
import ModelIO
import simd

private func mdl_copy_doubles(
    _ values: [Double],
    to outValues: UnsafeMutablePointer<Double>?,
    capacity: UInt64
) -> UInt64 {
    guard let outValues else { return 0 }
    let valueCount = min(Int(capacity), values.count)
    guard valueCount > 0 else { return 0 }
    values.withUnsafeBufferPointer { buffer in
        outValues.initialize(from: buffer.baseAddress!, count: valueCount)
    }
    return UInt64(valueCount)
}

private func mdl_transform_component(_ handle: UnsafeMutableRawPointer?) -> (any MDLTransformComponent)? {
    mdl_borrow_object(handle) as? any MDLTransformComponent
}

private func mdl_transform_op(_ handle: UnsafeMutableRawPointer?) -> (any MDLTransformOp)? {
    mdl_borrow_object(handle) as? any MDLTransformOp
}

private func mdl_transform_rotation_order(_ rawValue: UInt64) throws -> MDLTransformOpRotationOrder {
    guard let order = MDLTransformOpRotationOrder(rawValue: UInt(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLTransformOpRotationOrder: \(rawValue)")
    }
    return order
}

@_cdecl("mdl_transform_component_matrix")
public func mdl_transform_component_matrix(_ handle: UnsafeMutableRawPointer?, _ outValues: UnsafeMutablePointer<Float>?) {
    guard let component = mdl_transform_component(handle) else { return }
    _ = mdl_copy_floats(mdl_matrix_to_array(component.matrix), to: outValues, capacity: 16)
}

@_cdecl("mdl_transform_component_set_matrix")
public func mdl_transform_component_set_matrix(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Float>?) {
    guard let component = mdl_transform_component(handle) else { return }
    component.matrix = mdl_matrix_from_array(values)
}

@_cdecl("mdl_transform_component_resets_transform")
public func mdl_transform_component_resets_transform(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let component = mdl_transform_component(handle) else { return 0 }
    return component.resetsTransform ? 1 : 0
}

@_cdecl("mdl_transform_component_set_resets_transform")
public func mdl_transform_component_set_resets_transform(_ handle: UnsafeMutableRawPointer?, _ resetsTransform: Int32) {
    guard let component = mdl_transform_component(handle) else { return }
    component.resetsTransform = resetsTransform != 0
}

@_cdecl("mdl_transform_component_minimum_time")
public func mdl_transform_component_minimum_time(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let component = mdl_transform_component(handle) else { return 0 }
    return component.minimumTime
}

@_cdecl("mdl_transform_component_maximum_time")
public func mdl_transform_component_maximum_time(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let component = mdl_transform_component(handle) else { return 0 }
    return component.maximumTime
}

@_cdecl("mdl_transform_component_key_time_count")
public func mdl_transform_component_key_time_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let component = mdl_transform_component(handle) else { return 0 }
    return UInt64(component.keyTimes.count)
}

@_cdecl("mdl_transform_component_copy_key_times")
public func mdl_transform_component_copy_key_times(
    _ handle: UnsafeMutableRawPointer?,
    _ outValues: UnsafeMutablePointer<Double>?,
    _ capacity: UInt64
) -> UInt64 {
    guard let component = mdl_transform_component(handle) else { return 0 }
    let values = component.keyTimes.map(\.doubleValue)
    return mdl_copy_doubles(values, to: outValues, capacity: capacity)
}

@_cdecl("mdl_transform_component_local_transform_at_time")
public func mdl_transform_component_local_transform_at_time(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outValues: UnsafeMutablePointer<Float>?
) {
    guard let component = mdl_transform_component(handle) else { return }
    let matrix = component.localTransform?(atTime: time) ?? component.matrix
    _ = mdl_copy_floats(mdl_matrix_to_array(matrix), to: outValues, capacity: 16)
}

@_cdecl("mdl_transform_component_global_transform_with_object")
public func mdl_transform_component_global_transform_with_object(
    _ objectHandle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outValues: UnsafeMutablePointer<Float>?
) {
    guard let object = mdl_borrow_object(objectHandle) as? MDLObject else { return }
    _ = mdl_copy_floats(mdl_matrix_to_array(MDLTransform.globalTransform(with: object, atTime: time)), to: outValues, capacity: 16)
}

@_cdecl("mdl_transform_component_is_transform")
public func mdl_transform_component_is_transform(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let object = mdl_borrow_object(handle) else { return 0 }
    return object is MDLTransform ? 1 : 0
}

@_cdecl("mdl_transform_component_is_transform_stack")
public func mdl_transform_component_is_transform_stack(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let object = mdl_borrow_object(handle) else { return 0 }
    return object is MDLTransformStack ? 1 : 0
}

@_cdecl("mdl_object_transform_component")
public func mdl_object_transform_component(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let object = mdl_borrow_object(handle) as? MDLObject,
          let transform = object.transform
    else {
        return nil
    }
    return mdl_retain(transform as AnyObject)
}

@_cdecl("mdl_object_set_transform_component")
public func mdl_object_set_transform_component(
    _ handle: UnsafeMutableRawPointer?,
    _ componentHandle: UnsafeMutableRawPointer?
) {
    guard let object = mdl_borrow_object(handle) as? MDLObject else { return }
    object.transform = mdl_transform_component(componentHandle)
}

@_cdecl("mdl_transform_new")
public func mdl_transform_new(
    _ outTransform: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outTransform else {
            throw ModelIOBridgeError.invalidArgument("missing output transform pointer")
        }
        outTransform.pointee = mdl_retain(MDLTransform())
    }
}

@_cdecl("mdl_transform_new_with_component")
public func mdl_transform_new_with_component(
    _ componentHandle: UnsafeMutableRawPointer?,
    _ outTransform: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let component = mdl_transform_component(componentHandle),
              let outTransform
        else {
            throw ModelIOBridgeError.invalidArgument("missing transform component or output pointer")
        }
        outTransform.pointee = mdl_retain(MDLTransform(transformComponent: component))
    }
}

@_cdecl("mdl_transform_new_with_component_resets_transform")
public func mdl_transform_new_with_component_resets_transform(
    _ componentHandle: UnsafeMutableRawPointer?,
    _ resetsTransform: Int32,
    _ outTransform: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let component = mdl_transform_component(componentHandle),
              let outTransform
        else {
            throw ModelIOBridgeError.invalidArgument("missing transform component or output pointer")
        }
        outTransform.pointee = mdl_retain(
            MDLTransform(transformComponent: component, resetsTransform: resetsTransform != 0)
        )
    }
}

@_cdecl("mdl_transform_new_with_matrix")
public func mdl_transform_new_with_matrix(
    _ values: UnsafePointer<Float>?,
    _ outTransform: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outTransform else {
            throw ModelIOBridgeError.invalidArgument("missing output transform pointer")
        }
        outTransform.pointee = mdl_retain(MDLTransform(matrix: mdl_matrix_from_array(values)))
    }
}

@_cdecl("mdl_transform_new_with_matrix_resets_transform")
public func mdl_transform_new_with_matrix_resets_transform(
    _ values: UnsafePointer<Float>?,
    _ resetsTransform: Int32,
    _ outTransform: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outTransform else {
            throw ModelIOBridgeError.invalidArgument("missing output transform pointer")
        }
        outTransform.pointee = mdl_retain(
            MDLTransform(matrix: mdl_matrix_from_array(values), resetsTransform: resetsTransform != 0)
        )
    }
}

@_cdecl("mdl_transform_set_identity")
public func mdl_transform_set_identity(_ handle: UnsafeMutableRawPointer?) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    transform.setIdentity()
}

@_cdecl("mdl_transform_translation_at_time")
public func mdl_transform_translation_at_time(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outValues: UnsafeMutablePointer<Float>?
) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    _ = mdl_copy_floats(Array([transform.translation(atTime: time).x, transform.translation(atTime: time).y, transform.translation(atTime: time).z]), to: outValues, capacity: 3)
}

@_cdecl("mdl_transform_rotation_at_time")
public func mdl_transform_rotation_at_time(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outValues: UnsafeMutablePointer<Float>?
) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    let value = transform.rotation(atTime: time)
    _ = mdl_copy_floats([value.x, value.y, value.z], to: outValues, capacity: 3)
}

@_cdecl("mdl_transform_shear_at_time")
public func mdl_transform_shear_at_time(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outValues: UnsafeMutablePointer<Float>?
) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    let value = transform.shear(atTime: time)
    _ = mdl_copy_floats([value.x, value.y, value.z], to: outValues, capacity: 3)
}

@_cdecl("mdl_transform_scale_at_time")
public func mdl_transform_scale_at_time(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outValues: UnsafeMutablePointer<Float>?
) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    let value = transform.scale(atTime: time)
    _ = mdl_copy_floats([value.x, value.y, value.z], to: outValues, capacity: 3)
}

@_cdecl("mdl_transform_set_matrix_for_time")
public func mdl_transform_set_matrix_for_time(
    _ handle: UnsafeMutableRawPointer?,
    _ values: UnsafePointer<Float>?,
    _ time: Double
) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    transform.setMatrix(mdl_matrix_from_array(values), forTime: time)
}

@_cdecl("mdl_transform_set_translation_for_time")
public func mdl_transform_set_translation_for_time(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ time: Double
) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    transform.setTranslation(SIMD3<Float>(x, y, z), forTime: time)
}

@_cdecl("mdl_transform_set_rotation_for_time")
public func mdl_transform_set_rotation_for_time(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ time: Double
) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    transform.setRotation(SIMD3<Float>(x, y, z), forTime: time)
}

@_cdecl("mdl_transform_set_shear_for_time")
public func mdl_transform_set_shear_for_time(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ time: Double
) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    transform.setShear(SIMD3<Float>(x, y, z), forTime: time)
}

@_cdecl("mdl_transform_set_scale_for_time")
public func mdl_transform_set_scale_for_time(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ time: Double
) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    transform.setScale(SIMD3<Float>(x, y, z), forTime: time)
}

@_cdecl("mdl_transform_rotation_matrix_at_time")
public func mdl_transform_rotation_matrix_at_time(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outValues: UnsafeMutablePointer<Float>?
) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    _ = mdl_copy_floats(mdl_matrix_to_array(transform.rotationMatrix(atTime: time)), to: outValues, capacity: 16)
}

@_cdecl("mdl_transform_translation")
public func mdl_transform_translation(_ handle: UnsafeMutableRawPointer?, _ outValues: UnsafeMutablePointer<Float>?) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    let value = transform.translation
    _ = mdl_copy_floats([value.x, value.y, value.z], to: outValues, capacity: 3)
}

@_cdecl("mdl_transform_set_translation")
public func mdl_transform_set_translation(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    transform.translation = SIMD3<Float>(x, y, z)
}

@_cdecl("mdl_transform_rotation")
public func mdl_transform_rotation(_ handle: UnsafeMutableRawPointer?, _ outValues: UnsafeMutablePointer<Float>?) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    let value = transform.rotation
    _ = mdl_copy_floats([value.x, value.y, value.z], to: outValues, capacity: 3)
}

@_cdecl("mdl_transform_set_rotation")
public func mdl_transform_set_rotation(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    transform.rotation = SIMD3<Float>(x, y, z)
}

@_cdecl("mdl_transform_shear")
public func mdl_transform_shear(_ handle: UnsafeMutableRawPointer?, _ outValues: UnsafeMutablePointer<Float>?) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    let value = transform.shear
    _ = mdl_copy_floats([value.x, value.y, value.z], to: outValues, capacity: 3)
}

@_cdecl("mdl_transform_set_shear")
public func mdl_transform_set_shear(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    transform.shear = SIMD3<Float>(x, y, z)
}

@_cdecl("mdl_transform_scale")
public func mdl_transform_scale(_ handle: UnsafeMutableRawPointer?, _ outValues: UnsafeMutablePointer<Float>?) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    let value = transform.scale
    _ = mdl_copy_floats([value.x, value.y, value.z], to: outValues, capacity: 3)
}

@_cdecl("mdl_transform_set_scale")
public func mdl_transform_set_scale(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float) {
    guard let transform = mdl_borrow_object(handle) as? MDLTransform else { return }
    transform.scale = SIMD3<Float>(x, y, z)
}

@_cdecl("mdl_transform_op_name_string")
public func mdl_transform_op_name_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let transformOp = mdl_transform_op(handle) else { return nil }
    return mdl_string(transformOp.name)
}

@_cdecl("mdl_transform_op_is_inverse")
public func mdl_transform_op_is_inverse(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let transformOp = mdl_borrow_object(handle) else { return 0 }
    return ((transformOp as AnyObject).value(forKey: "inverse") as? NSNumber)?.boolValue == true ? 1 : 0
}

@_cdecl("mdl_transform_op_copy_float4x4_at_time")
public func mdl_transform_op_copy_float4x4_at_time(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outValues: UnsafeMutablePointer<Float>?
) {
    guard let transformOp = mdl_transform_op(handle) else { return }
    _ = mdl_copy_floats(mdl_matrix_to_array(transformOp.float4x4(atTime: time)), to: outValues, capacity: 16)
}

@_cdecl("mdl_transform_rotate_x_op_animated_value")
public func mdl_transform_rotate_x_op_animated_value(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let op = mdl_borrow_object(handle) as? MDLTransformRotateXOp else { return nil }
    return mdl_retain(op.animatedValue)
}

@_cdecl("mdl_transform_rotate_y_op_animated_value")
public func mdl_transform_rotate_y_op_animated_value(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let op = mdl_borrow_object(handle) as? MDLTransformRotateYOp else { return nil }
    return mdl_retain(op.animatedValue)
}

@_cdecl("mdl_transform_rotate_z_op_animated_value")
public func mdl_transform_rotate_z_op_animated_value(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let op = mdl_borrow_object(handle) as? MDLTransformRotateZOp else { return nil }
    return mdl_retain(op.animatedValue)
}

@_cdecl("mdl_transform_rotate_op_animated_value")
public func mdl_transform_rotate_op_animated_value(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let op = mdl_borrow_object(handle) as? MDLTransformRotateOp else { return nil }
    return mdl_retain(op.animatedValue)
}

@_cdecl("mdl_transform_translate_op_animated_value")
public func mdl_transform_translate_op_animated_value(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let op = mdl_borrow_object(handle) as? MDLTransformTranslateOp else { return nil }
    return mdl_retain(op.animatedValue)
}

@_cdecl("mdl_transform_scale_op_animated_value")
public func mdl_transform_scale_op_animated_value(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let op = mdl_borrow_object(handle) as? MDLTransformScaleOp else { return nil }
    return mdl_retain(op.animatedValue)
}

@_cdecl("mdl_transform_matrix_op_animated_value")
public func mdl_transform_matrix_op_animated_value(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let op = mdl_borrow_object(handle) as? MDLTransformMatrixOp else { return nil }
    return mdl_retain(op.animatedValue)
}

@_cdecl("mdl_transform_orient_op_animated_value")
public func mdl_transform_orient_op_animated_value(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let op = mdl_borrow_object(handle) as? MDLTransformOrientOp else { return nil }
    return mdl_retain(op.animatedValue)
}

@_cdecl("mdl_transform_stack_new")
public func mdl_transform_stack_new(
    _ outStack: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outStack else {
            throw ModelIOBridgeError.invalidArgument("missing output transform stack pointer")
        }
        outStack.pointee = mdl_retain(MDLTransformStack())
    }
}

@_cdecl("mdl_transform_stack_add_translate_op")
public func mdl_transform_stack_add_translate_op(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ inverse: Int32
) -> UnsafeMutableRawPointer? {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack,
          let name
    else {
        return nil
    }
    return mdl_retain(stack.addTranslateOp(String(cString: name), inverse: inverse != 0))
}

@_cdecl("mdl_transform_stack_add_rotate_x_op")
public func mdl_transform_stack_add_rotate_x_op(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ inverse: Int32
) -> UnsafeMutableRawPointer? {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack,
          let name
    else {
        return nil
    }
    return mdl_retain(stack.addRotateXOp(String(cString: name), inverse: inverse != 0))
}

@_cdecl("mdl_transform_stack_add_rotate_y_op")
public func mdl_transform_stack_add_rotate_y_op(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ inverse: Int32
) -> UnsafeMutableRawPointer? {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack,
          let name
    else {
        return nil
    }
    return mdl_retain(stack.addRotateYOp(String(cString: name), inverse: inverse != 0))
}

@_cdecl("mdl_transform_stack_add_rotate_z_op")
public func mdl_transform_stack_add_rotate_z_op(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ inverse: Int32
) -> UnsafeMutableRawPointer? {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack,
          let name
    else {
        return nil
    }
    return mdl_retain(stack.addRotateZOp(String(cString: name), inverse: inverse != 0))
}

@_cdecl("mdl_transform_stack_add_rotate_op")
public func mdl_transform_stack_add_rotate_op(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ rotationOrder: UInt64,
    _ inverse: Int32
) -> UnsafeMutableRawPointer? {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack,
          let name,
          let order = try? mdl_transform_rotation_order(rotationOrder)
    else {
        return nil
    }
    return mdl_retain(
        stack.addRotateOp(String(cString: name), order: order, inverse: inverse != 0)
    )
}

@_cdecl("mdl_transform_stack_add_scale_op")
public func mdl_transform_stack_add_scale_op(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ inverse: Int32
) -> UnsafeMutableRawPointer? {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack,
          let name
    else {
        return nil
    }
    return mdl_retain(stack.addScaleOp(String(cString: name), inverse: inverse != 0))
}

@_cdecl("mdl_transform_stack_add_matrix_op")
public func mdl_transform_stack_add_matrix_op(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ inverse: Int32
) -> UnsafeMutableRawPointer? {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack,
          let name
    else {
        return nil
    }
    return mdl_retain(stack.addMatrixOp(String(cString: name), inverse: inverse != 0))
}

@_cdecl("mdl_transform_stack_add_orient_op")
public func mdl_transform_stack_add_orient_op(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ inverse: Int32
) -> UnsafeMutableRawPointer? {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack,
          let name
    else {
        return nil
    }
    return mdl_retain(stack.addOrientOp(String(cString: name), inverse: inverse != 0))
}

@_cdecl("mdl_transform_stack_animated_value_named")
public func mdl_transform_stack_animated_value_named(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack,
          let name
    else {
        return nil
    }
    return mdl_retain(stack.animatedValue(withName: String(cString: name)))
}

@_cdecl("mdl_transform_stack_copy_float4x4_at_time")
public func mdl_transform_stack_copy_float4x4_at_time(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outValues: UnsafeMutablePointer<Float>?
) {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack else { return }
    _ = mdl_copy_floats(mdl_matrix_to_array(stack.float4x4(atTime: time)), to: outValues, capacity: 16)
}

@_cdecl("mdl_transform_stack_count")
public func mdl_transform_stack_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack else { return 0 }
    return UInt64(stack.count())
}

@_cdecl("mdl_transform_stack_transform_ops")
public func mdl_transform_stack_transform_ops(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let stack = mdl_borrow_object(handle) as? MDLTransformStack else { return nil }
    return mdl_retain(stack.transformOps as NSArray)
}
