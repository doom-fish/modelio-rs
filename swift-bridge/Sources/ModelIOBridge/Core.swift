import CoreGraphics
import Darwin
import Foundation
import ModelIO
import simd

public let MDLX_OK: Int32 = 0
public let MDLX_INVALID_ARGUMENT: Int32 = -1
public let MDLX_NULL_RESULT: Int32 = -2
public let MDLX_FRAMEWORK: Int32 = -3
public let MDLX_UNKNOWN: Int32 = -99

@inline(__always)
public func mdl_string(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
public func mdl_retain(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func mdl_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@inline(__always)
public func mdl_borrow_object(_ handle: UnsafeMutableRawPointer?) -> AnyObject? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue()
}

public enum ModelIOBridgeError: Error, CustomStringConvertible {
    case invalidArgument(String)
    case nullResult(String)
    case framework(Error)
    case unknown(String)

    public var description: String {
        switch self {
        case .invalidArgument(let message), .nullResult(let message), .unknown(let message):
            return message
        case .framework(let error):
            return error.localizedDescription
        }
    }

    public var statusCode: Int32 {
        switch self {
        case .invalidArgument:
            return MDLX_INVALID_ARGUMENT
        case .nullResult:
            return MDLX_NULL_RESULT
        case .framework:
            return MDLX_FRAMEWORK
        case .unknown:
            return MDLX_UNKNOWN
        }
    }
}

@inline(__always)
public func mdl_status(from error: Error) -> Int32 {
    if let error = error as? ModelIOBridgeError {
        return error.statusCode
    }
    return MDLX_FRAMEWORK
}

@inline(__always)
public func mdl_message(from error: Error) -> String {
    if let error = error as? ModelIOBridgeError {
        return error.description
    }
    return (error as NSError).localizedDescription
}

@inline(__always)
public func mdl_fail(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ error: Error
) -> Int32 {
    if let outError {
        outError.pointee = mdl_string(mdl_message(from: error))
    }
    return mdl_status(from: error)
}

@inline(__always)
public func mdl_run(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ work: () throws -> Void
) -> Int32 {
    do {
        try work()
        outError?.pointee = nil
        return MDLX_OK
    } catch {
        return mdl_fail(outError, error)
    }
}

@inline(__always)
public func mdl_color_components(_ color: CGColor?) -> [Float]? {
    guard let color else { return nil }
    let colorSpace = CGColorSpace(name: CGColorSpace.sRGB)
    let converted = colorSpace.flatMap { color.converted(to: $0, intent: .defaultIntent, options: nil) } ?? color
    guard let components = converted.components else {
        return nil
    }

    switch components.count {
    case 4:
        return components.map(Float.init)
    case 2:
        return [Float(components[0]), Float(components[0]), Float(components[0]), Float(components[1])]
    case 1:
        return [Float(components[0]), Float(components[0]), Float(components[0]), 1.0]
    default:
        return nil
    }
}

@inline(__always)
public func mdl_color(_ red: Float, _ green: Float, _ blue: Float, _ alpha: Float) -> CGColor {
    CGColor(colorSpace: CGColorSpace(name: CGColorSpace.sRGB)!, components: [CGFloat(red), CGFloat(green), CGFloat(blue), CGFloat(alpha)])!
}

@inline(__always)
public func mdl_json_string(from value: Any) -> String? {
    guard JSONSerialization.isValidJSONObject(value) else {
        return nil
    }
    do {
        let data = try JSONSerialization.data(withJSONObject: value, options: [.sortedKeys])
        return String(data: data, encoding: .utf8)
    } catch {
        return nil
    }
}

@inline(__always)
public func mdl_string_array(
    _ values: UnsafePointer<UnsafePointer<CChar>?>?,
    count: UInt64
) -> [String] {
    guard let values, count > 0 else { return [] }
    return (0..<Int(count)).compactMap { index in
        values[index].map { String(cString: $0) }
    }
}

@inline(__always)
public func mdl_bounding_box(_ boundingBox: MDLAxisAlignedBoundingBox) -> [String: Any] {
    [
        "min": [boundingBox.minBounds.x, boundingBox.minBounds.y, boundingBox.minBounds.z],
        "max": [boundingBox.maxBounds.x, boundingBox.maxBounds.y, boundingBox.maxBounds.z],
    ]
}

@inline(__always)
public func mdl_matrix_to_array(_ matrix: matrix_float4x4) -> [Float] {
    [
        matrix.columns.0.x, matrix.columns.0.y, matrix.columns.0.z, matrix.columns.0.w,
        matrix.columns.1.x, matrix.columns.1.y, matrix.columns.1.z, matrix.columns.1.w,
        matrix.columns.2.x, matrix.columns.2.y, matrix.columns.2.z, matrix.columns.2.w,
        matrix.columns.3.x, matrix.columns.3.y, matrix.columns.3.z, matrix.columns.3.w,
    ]
}

@inline(__always)
public func mdl_matrix_from_array(_ values: UnsafePointer<Float>?) -> matrix_float4x4 {
    guard let values else { return matrix_identity_float4x4 }
    return matrix_float4x4(columns: (
        SIMD4<Float>(values[0], values[1], values[2], values[3]),
        SIMD4<Float>(values[4], values[5], values[6], values[7]),
        SIMD4<Float>(values[8], values[9], values[10], values[11]),
        SIMD4<Float>(values[12], values[13], values[14], values[15])
    ))
}

@inline(__always)
public func mdl_quaternion_to_array(_ quaternion: simd_quatf) -> [Float] {
    [quaternion.vector.x, quaternion.vector.y, quaternion.vector.z, quaternion.vector.w]
}

@inline(__always)
public func mdl_quaternion_from_array(_ values: UnsafePointer<Float>?) -> simd_quatf {
    guard let values else {
        return simd_quatf(ix: 0, iy: 0, iz: 0, r: 1)
    }
    return simd_quatf(ix: values[0], iy: values[1], iz: values[2], r: values[3])
}

@discardableResult
@inline(__always)
public func mdl_copy_data(
    _ data: Data,
    to outBytes: UnsafeMutableRawPointer?,
    capacity: UInt64
) -> UInt64 {
    guard let outBytes else { return 0 }
    let byteCount = min(Int(capacity), data.count)
    data.copyBytes(to: outBytes.assumingMemoryBound(to: UInt8.self), count: byteCount)
    return UInt64(byteCount)
}

@discardableResult
@inline(__always)
public func mdl_copy_floats(
    _ values: [Float],
    to outValues: UnsafeMutablePointer<Float>?,
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

@discardableResult
@inline(__always)
public func mdl_copy_int32s(
    _ values: [Int32],
    to outValues: UnsafeMutablePointer<Int32>?,
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

@inline(__always)
public func mdl_data_from_int32s(
    _ values: UnsafePointer<Int32>?,
    count: UInt64
) -> Data {
    guard let values, count > 0 else { return Data() }
    return Data(bytes: values, count: Int(count) * MemoryLayout<Int32>.stride)
}

@inline(__always)
public func mdl_vector_int4(_ values: UnsafePointer<Int32>?) -> vector_int4 {
    guard let values else { return vector_int4(repeating: 0) }
    return vector_int4(values[0], values[1], values[2], values[3])
}

@inline(__always)
public func mdl_vector_float3(_ x: Float, _ y: Float, _ z: Float) -> vector_float3 {
    vector_float3(x, y, z)
}

@inline(__always)
public func mdl_geometry_type(_ rawValue: Int32) throws -> MDLGeometryType {
    guard let geometryType = MDLGeometryType(rawValue: Int(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLGeometryType: \(rawValue)")
    }
    return geometryType
}

@inline(__always)
public func mdl_index_bit_depth(_ rawValue: UInt32) throws -> MDLIndexBitDepth {
    guard let indexType = MDLIndexBitDepth(rawValue: UInt(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLIndexBitDepth: \(rawValue)")
    }
    return indexType
}

@inline(__always)
public func mdl_semantic(_ rawValue: UInt32) throws -> MDLMaterialSemantic {
    guard let semantic = MDLMaterialSemantic(rawValue: UInt(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLMaterialSemantic: \(rawValue)")
    }
    return semantic
}

@inline(__always)
public func mdl_material_face(_ rawValue: UInt32) throws -> MDLMaterialFace {
    guard let face = MDLMaterialFace(rawValue: UInt(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLMaterialFace: \(rawValue)")
    }
    return face
}

@inline(__always)
public func mdl_light_type(_ rawValue: UInt32) throws -> MDLLightType {
    guard let lightType = MDLLightType(rawValue: UInt(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLLightType: \(rawValue)")
    }
    return lightType
}

@inline(__always)
public func mdl_camera_projection(_ rawValue: UInt32) throws -> MDLCameraProjection {
    guard let projection = MDLCameraProjection(rawValue: UInt(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLCameraProjection: \(rawValue)")
    }
    return projection
}

@inline(__always)
public func mdl_interpolation(_ rawValue: UInt32) throws -> MDLAnimatedValueInterpolation {
    guard let interpolation = MDLAnimatedValueInterpolation(rawValue: UInt(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLAnimatedValueInterpolation: \(rawValue)")
    }
    return interpolation
}

@inline(__always)
public func mdl_vertex_format(_ rawValue: UInt32) throws -> MDLVertexFormat {
    guard let format = MDLVertexFormat(rawValue: UInt(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLVertexFormat: \(rawValue)")
    }
    return format
}

@inline(__always)
public func mdl_texture_channel_encoding(_ rawValue: Int32) throws -> MDLTextureChannelEncoding {
    guard let encoding = MDLTextureChannelEncoding(rawValue: Int(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLTextureChannelEncoding: \(rawValue)")
    }
    return encoding
}

@_cdecl("mdl_object_retain")
public func mdl_object_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let object = mdl_borrow_object(handle) else { return nil }
    return mdl_retain(object)
}

@_cdecl("mdl_object_release")
public func mdl_object_release(_ handle: UnsafeMutableRawPointer?) {
    mdl_release(handle)
}
