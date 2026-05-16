import Foundation
import ModelIO
import simd

private func mdl_animated_value_info(_ value: MDLAnimatedValue) -> [String: Any] {
    var info: [String: Any] = [
        "is_animated": value.isAnimated(),
        "precision": value.precision.rawValue,
        "time_sample_count": value.timeSampleCount,
        "minimum_time": value.minimumTime,
        "maximum_time": value.maximumTime,
        "interpolation": value.interpolation.rawValue,
        "key_times": value.keyTimes,
    ]
    switch value {
    case let scalarArray as MDLAnimatedScalarArray:
        info["element_count"] = scalarArray.elementCount
    case let vector3Array as MDLAnimatedVector3Array:
        info["element_count"] = vector3Array.elementCount
    case let quaternionArray as MDLAnimatedQuaternionArray:
        info["element_count"] = quaternionArray.elementCount
    default:
        break
    }
    return info
}

@_cdecl("mdl_animated_value_info_json")
public func mdl_animated_value_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let value = mdl_borrow_object(handle) as? MDLAnimatedValue else { return nil }
    return mdl_string(mdl_json_string(from: mdl_animated_value_info(value)) ?? "{}")
}

@_cdecl("mdl_animated_value_clear")
public func mdl_animated_value_clear(_ handle: UnsafeMutableRawPointer?) {
    guard let value = mdl_borrow_object(handle) as? MDLAnimatedValue else { return }
    value.clear()
}

@_cdecl("mdl_animated_value_set_interpolation")
public func mdl_animated_value_set_interpolation(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) {
    guard let value = mdl_borrow_object(handle) as? MDLAnimatedValue,
          let interpolation = try? mdl_interpolation(rawValue)
    else {
        return
    }
    value.interpolation = interpolation
}

@_cdecl("mdl_animated_scalar_new")
public func mdl_animated_scalar_new(
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outValue else {
            throw ModelIOBridgeError.invalidArgument("missing output animated scalar pointer")
        }
        outValue.pointee = mdl_retain(MDLAnimatedScalar())
    }
}

@_cdecl("mdl_animated_scalar_set_float")
public func mdl_animated_scalar_set_float(_ handle: UnsafeMutableRawPointer?, _ value: Float, _ time: Double) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedScalar else { return }
    animated.setFloat(value, atTime: time)
}

@_cdecl("mdl_animated_scalar_float_value")
public func mdl_animated_scalar_float_value(_ handle: UnsafeMutableRawPointer?, _ time: Double) -> Float {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedScalar else { return 0 }
    return animated.floatValue(atTime: time)
}

@_cdecl("mdl_animated_vector2_new")
public func mdl_animated_vector2_new(
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outValue else {
            throw ModelIOBridgeError.invalidArgument("missing output animated vector2 pointer")
        }
        outValue.pointee = mdl_retain(MDLAnimatedVector2())
    }
}

@_cdecl("mdl_animated_vector2_set_float2")
public func mdl_animated_vector2_set_float2(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ time: Double) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedVector2 else { return }
    animated.setFloat2(SIMD2<Float>(x, y), atTime: time)
}

@_cdecl("mdl_animated_vector2_copy_float2_value")
public func mdl_animated_vector2_copy_float2_value(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outX: UnsafeMutablePointer<Float>?,
    _ outY: UnsafeMutablePointer<Float>?
) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedVector2 else { return }
    let value = animated.float2Value(atTime: time)
    outX?.pointee = value.x
    outY?.pointee = value.y
}

@_cdecl("mdl_animated_vector3_new")
public func mdl_animated_vector3_new(
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outValue else {
            throw ModelIOBridgeError.invalidArgument("missing output animated vector3 pointer")
        }
        outValue.pointee = mdl_retain(MDLAnimatedVector3())
    }
}

@_cdecl("mdl_animated_vector3_set_float3")
public func mdl_animated_vector3_set_float3(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float, _ time: Double) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedVector3 else { return }
    animated.setFloat3(SIMD3<Float>(x, y, z), atTime: time)
}

@_cdecl("mdl_animated_vector3_copy_float3_value")
public func mdl_animated_vector3_copy_float3_value(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outX: UnsafeMutablePointer<Float>?,
    _ outY: UnsafeMutablePointer<Float>?,
    _ outZ: UnsafeMutablePointer<Float>?
) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedVector3 else { return }
    let value = animated.float3Value(atTime: time)
    outX?.pointee = value.x
    outY?.pointee = value.y
    outZ?.pointee = value.z
}

@_cdecl("mdl_animated_vector4_new")
public func mdl_animated_vector4_new(
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outValue else {
            throw ModelIOBridgeError.invalidArgument("missing output animated vector4 pointer")
        }
        outValue.pointee = mdl_retain(MDLAnimatedVector4())
    }
}

@_cdecl("mdl_animated_vector4_set_float4")
public func mdl_animated_vector4_set_float4(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float, _ w: Float, _ time: Double) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedVector4 else { return }
    animated.setFloat4(SIMD4<Float>(x, y, z, w), atTime: time)
}

@_cdecl("mdl_animated_vector4_copy_float4_value")
public func mdl_animated_vector4_copy_float4_value(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outX: UnsafeMutablePointer<Float>?,
    _ outY: UnsafeMutablePointer<Float>?,
    _ outZ: UnsafeMutablePointer<Float>?,
    _ outW: UnsafeMutablePointer<Float>?
) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedVector4 else { return }
    let value = animated.float4Value(atTime: time)
    outX?.pointee = value.x
    outY?.pointee = value.y
    outZ?.pointee = value.z
    outW?.pointee = value.w
}

@_cdecl("mdl_animated_quaternion_new")
public func mdl_animated_quaternion_new(
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outValue else {
            throw ModelIOBridgeError.invalidArgument("missing output animated quaternion pointer")
        }
        outValue.pointee = mdl_retain(MDLAnimatedQuaternion())
    }
}

@_cdecl("mdl_animated_quaternion_set_float")
public func mdl_animated_quaternion_set_float(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float, _ w: Float, _ time: Double) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedQuaternion else { return }
    animated.setFloatQuaternion(simd_quatf(ix: x, iy: y, iz: z, r: w), atTime: time)
}

@_cdecl("mdl_animated_quaternion_copy_float_value")
public func mdl_animated_quaternion_copy_float_value(_ handle: UnsafeMutableRawPointer?, _ time: Double, _ outValues: UnsafeMutablePointer<Float>?) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedQuaternion,
          let outValues
    else {
        return
    }
    _ = mdl_copy_floats(mdl_quaternion_to_array(animated.floatQuaternionValue(atTime: time)), to: outValues, capacity: 4)
}

@_cdecl("mdl_animated_matrix4x4_new")
public func mdl_animated_matrix4x4_new(
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outValue else {
            throw ModelIOBridgeError.invalidArgument("missing output animated matrix pointer")
        }
        outValue.pointee = mdl_retain(MDLAnimatedMatrix4x4())
    }
}

@_cdecl("mdl_animated_matrix4x4_set_float")
public func mdl_animated_matrix4x4_set_float(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Float>?, _ time: Double) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedMatrix4x4 else { return }
    animated.setFloat4x4(mdl_matrix_from_array(values), atTime: time)
}

@_cdecl("mdl_animated_matrix4x4_copy_float_value")
public func mdl_animated_matrix4x4_copy_float_value(_ handle: UnsafeMutableRawPointer?, _ time: Double, _ outValues: UnsafeMutablePointer<Float>?) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedMatrix4x4,
          let outValues
    else {
        return
    }
    _ = mdl_copy_floats(mdl_matrix_to_array(animated.float4x4Value(atTime: time)), to: outValues, capacity: 16)
}

@_cdecl("mdl_animated_scalar_array_new")
public func mdl_animated_scalar_array_new(
    _ elementCount: UInt64,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outValue else {
            throw ModelIOBridgeError.invalidArgument("missing output animated scalar array pointer")
        }
        outValue.pointee = mdl_retain(MDLAnimatedScalarArray(elementCount: Int(elementCount)))
    }
}

@_cdecl("mdl_animated_scalar_array_set_float")
public func mdl_animated_scalar_array_set_float(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Float>?, _ count: UInt64, _ time: Double) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedScalarArray else { return }
    guard let values, count > 0 else {
        animated.set(floatArray: [], atTime: time)
        return
    }
    animated.set(floatArray: Array(UnsafeBufferPointer(start: values, count: Int(count))), atTime: time)
}

@_cdecl("mdl_animated_scalar_array_copy_float_at_time")
public func mdl_animated_scalar_array_copy_float_at_time(_ handle: UnsafeMutableRawPointer?, _ time: Double, _ outValues: UnsafeMutablePointer<Float>?, _ capacity: UInt64) -> UInt64 {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedScalarArray else { return 0 }
    let values = animated.floatArray(atTime: time)
    let written = mdl_copy_floats(values, to: outValues, capacity: capacity)
    return min(UInt64(values.count), written)
}

@_cdecl("mdl_animated_vector3_array_new")
public func mdl_animated_vector3_array_new(
    _ elementCount: UInt64,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outValue else {
            throw ModelIOBridgeError.invalidArgument("missing output animated vector3 array pointer")
        }
        outValue.pointee = mdl_retain(MDLAnimatedVector3Array(elementCount: Int(elementCount)))
    }
}

@_cdecl("mdl_animated_vector3_array_set_float")
public func mdl_animated_vector3_array_set_float(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Float>?, _ count: UInt64, _ time: Double) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedVector3Array else { return }
    guard let values, count > 0 else {
        animated.set(float3Array: [], atTime: time)
        return
    }
    let raw = UnsafeBufferPointer(start: values, count: Int(count) * 3)
    var vectors: [SIMD3<Float>] = []
    vectors.reserveCapacity(Int(count))
    for index in 0..<Int(count) {
        vectors.append(SIMD3<Float>(raw[index * 3], raw[index * 3 + 1], raw[index * 3 + 2]))
    }
    animated.set(float3Array: vectors, atTime: time)
}

@_cdecl("mdl_animated_vector3_array_copy_float_at_time")
public func mdl_animated_vector3_array_copy_float_at_time(_ handle: UnsafeMutableRawPointer?, _ time: Double, _ outValues: UnsafeMutablePointer<Float>?, _ capacityElements: UInt64) -> UInt64 {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedVector3Array,
          let outValues
    else {
        return 0
    }
    let values = animated.float3Array(atTime: time)
    let copyCount = min(Int(capacityElements), values.count)
    guard copyCount > 0 else { return 0 }
    var flattened: [Float] = []
    flattened.reserveCapacity(copyCount * 3)
    for value in values.prefix(copyCount) {
        flattened.append(contentsOf: [value.x, value.y, value.z])
    }
    flattened.withUnsafeBufferPointer { buffer in
        outValues.initialize(from: buffer.baseAddress!, count: flattened.count)
    }
    return UInt64(copyCount)
}

@_cdecl("mdl_animated_quaternion_array_new")
public func mdl_animated_quaternion_array_new(
    _ elementCount: UInt64,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outValue else {
            throw ModelIOBridgeError.invalidArgument("missing output animated quaternion array pointer")
        }
        outValue.pointee = mdl_retain(MDLAnimatedQuaternionArray(elementCount: Int(elementCount)))
    }
}

@_cdecl("mdl_animated_quaternion_array_set_float")
public func mdl_animated_quaternion_array_set_float(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Float>?, _ count: UInt64, _ time: Double) {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedQuaternionArray else { return }
    guard let values, count > 0 else {
        animated.set(floatQuaternionArray: [], atTime: time)
        return
    }
    let raw = UnsafeBufferPointer(start: values, count: Int(count) * 4)
    var quaternions: [simd_quatf] = []
    quaternions.reserveCapacity(Int(count))
    for index in 0..<Int(count) {
        quaternions.append(simd_quatf(ix: raw[index * 4], iy: raw[index * 4 + 1], iz: raw[index * 4 + 2], r: raw[index * 4 + 3]))
    }
    animated.set(floatQuaternionArray: quaternions, atTime: time)
}

@_cdecl("mdl_animated_quaternion_array_copy_float_at_time")
public func mdl_animated_quaternion_array_copy_float_at_time(_ handle: UnsafeMutableRawPointer?, _ time: Double, _ outValues: UnsafeMutablePointer<Float>?, _ capacityElements: UInt64) -> UInt64 {
    guard let animated = mdl_borrow_object(handle) as? MDLAnimatedQuaternionArray,
          let outValues
    else {
        return 0
    }
    let values = animated.floatQuaternionArray(atTime: time)
    let copyCount = min(Int(capacityElements), values.count)
    guard copyCount > 0 else { return 0 }
    var flattened: [Float] = []
    flattened.reserveCapacity(copyCount * 4)
    for value in values.prefix(copyCount) {
        flattened.append(contentsOf: mdl_quaternion_to_array(value))
    }
    flattened.withUnsafeBufferPointer { buffer in
        outValues.initialize(from: buffer.baseAddress!, count: flattened.count)
    }
    return UInt64(copyCount)
}
