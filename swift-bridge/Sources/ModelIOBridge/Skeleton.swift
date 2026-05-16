import Foundation
import ModelIO
import simd

private func mdl_skeleton_info(_ skeleton: MDLSkeleton) -> [String: Any] {
    [
        "name": skeleton.name,
        "path": skeleton.path,
        "joint_paths": skeleton.jointPaths,
        "joint_count": skeleton.jointPaths.count,
        "joint_bind_transform_count": skeleton.jointBindTransforms.float4x4Array.count,
        "joint_rest_transform_count": skeleton.jointRestTransforms.float4x4Array.count,
    ]
}

private func mdl_copy_matrices(
    _ matrices: [matrix_float4x4],
    to outValues: UnsafeMutablePointer<Float>?,
    capacityMatrices: UInt64
) -> UInt64 {
    guard let outValues else { return 0 }
    let copyCount = min(Int(capacityMatrices), matrices.count)
    guard copyCount > 0 else { return 0 }
    var flattened: [Float] = []
    flattened.reserveCapacity(copyCount * 16)
    for matrix in matrices.prefix(copyCount) {
        flattened.append(contentsOf: mdl_matrix_to_array(matrix))
    }
    flattened.withUnsafeBufferPointer { buffer in
        outValues.initialize(from: buffer.baseAddress!, count: flattened.count)
    }
    return UInt64(copyCount)
}

@_cdecl("mdl_skeleton_new")
public func mdl_skeleton_new(
    _ name: UnsafePointer<CChar>?,
    _ jointPaths: UnsafePointer<UnsafePointer<CChar>?>?,
    _ jointPathCount: UInt64,
    _ outSkeleton: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let name, let outSkeleton else {
            throw ModelIOBridgeError.invalidArgument("missing skeleton name or output pointer")
        }
        outSkeleton.pointee = mdl_retain(
            MDLSkeleton(
                name: String(cString: name),
                jointPaths: mdl_string_array(jointPaths, count: jointPathCount)
            )
        )
    }
}

@_cdecl("mdl_skeleton_info_json")
public func mdl_skeleton_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let skeleton = mdl_borrow_object(handle) as? MDLSkeleton else { return nil }
    return mdl_string(mdl_json_string(from: mdl_skeleton_info(skeleton)) ?? "{}")
}

@_cdecl("mdl_skeleton_copy_joint_bind_transforms")
public func mdl_skeleton_copy_joint_bind_transforms(_ handle: UnsafeMutableRawPointer?, _ outValues: UnsafeMutablePointer<Float>?, _ capacityMatrices: UInt64) -> UInt64 {
    guard let skeleton = mdl_borrow_object(handle) as? MDLSkeleton else { return 0 }
    return mdl_copy_matrices(skeleton.jointBindTransforms.float4x4Array, to: outValues, capacityMatrices: capacityMatrices)
}

@_cdecl("mdl_skeleton_copy_joint_rest_transforms")
public func mdl_skeleton_copy_joint_rest_transforms(_ handle: UnsafeMutableRawPointer?, _ outValues: UnsafeMutablePointer<Float>?, _ capacityMatrices: UInt64) -> UInt64 {
    guard let skeleton = mdl_borrow_object(handle) as? MDLSkeleton else { return 0 }
    return mdl_copy_matrices(skeleton.jointRestTransforms.float4x4Array, to: outValues, capacityMatrices: capacityMatrices)
}
