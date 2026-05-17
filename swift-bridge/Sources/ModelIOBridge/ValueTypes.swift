import Foundation
import ModelIO
import simd

private func mdl_matrix4x4_array_info(_ array: MDLMatrix4x4Array) -> [String: Any] {
    [
        "element_count": array.elementCount,
        "precision": array.precision.rawValue,
    ]
}

private func mdl_float_matrix_array(_ values: UnsafePointer<Float>?, count: UInt64) -> [matrix_float4x4] {
    guard let values, count > 0 else { return [] }
    return (0..<Int(count)).map { index in
        mdl_matrix_from_array(values.advanced(by: index * 16))
    }
}

@_cdecl("mdl_matrix4x4_array_new")
public func mdl_matrix4x4_array_new(
    _ elementCount: UInt64,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outArray else {
            throw ModelIOBridgeError.invalidArgument("missing output matrix array pointer")
        }
        outArray.pointee = mdl_retain(MDLMatrix4x4Array(elementCount: Int(elementCount)))
    }
}

@_cdecl("mdl_matrix4x4_array_info_json")
public func mdl_matrix4x4_array_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let array = mdl_borrow_object(handle) as? MDLMatrix4x4Array else { return nil }
    return mdl_string(mdl_json_string(from: mdl_matrix4x4_array_info(array)) ?? "{}")
}

@_cdecl("mdl_matrix4x4_array_clear")
public func mdl_matrix4x4_array_clear(_ handle: UnsafeMutableRawPointer?) {
    guard let array = mdl_borrow_object(handle) as? MDLMatrix4x4Array else { return }
    array.clear()
}

@_cdecl("mdl_matrix4x4_array_set_float_matrices")
public func mdl_matrix4x4_array_set_float_matrices(
    _ handle: UnsafeMutableRawPointer?,
    _ values: UnsafePointer<Float>?,
    _ count: UInt64
) {
    guard let array = mdl_borrow_object(handle) as? MDLMatrix4x4Array else { return }
    array.float4x4Array = mdl_float_matrix_array(values, count: count)
}

@_cdecl("mdl_matrix4x4_array_copy_float_matrices")
public func mdl_matrix4x4_array_copy_float_matrices(
    _ handle: UnsafeMutableRawPointer?,
    _ outValues: UnsafeMutablePointer<Float>?,
    _ capacityMatrices: UInt64
) -> UInt64 {
    guard let array = mdl_borrow_object(handle) as? MDLMatrix4x4Array,
          let outValues
    else {
        return 0
    }
    let matrices = array.float4x4Array
    let copyCount = min(Int(capacityMatrices), matrices.count)
    guard copyCount > 0 else { return 0 }
    let flattened = matrices.prefix(copyCount).flatMap(mdl_matrix_to_array)
    flattened.withUnsafeBufferPointer { buffer in
        outValues.initialize(from: buffer.baseAddress!, count: buffer.count)
    }
    return UInt64(copyCount)
}
