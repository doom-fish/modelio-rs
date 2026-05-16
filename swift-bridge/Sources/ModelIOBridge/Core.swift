import CoreGraphics
import Foundation

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

@_cdecl("mdl_object_retain")
public func mdl_object_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let object = mdl_borrow_object(handle) else { return nil }
    return mdl_retain(object)
}

@_cdecl("mdl_object_release")
public func mdl_object_release(_ handle: UnsafeMutableRawPointer?) {
    mdl_release(handle)
}
