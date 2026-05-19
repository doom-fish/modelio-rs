import Darwin
import Foundation
import ModelIO

@_silgen_name("mdlx_mesh_buffer_allocator_new_zone")
private func mdlx_mesh_buffer_allocator_new_zone(
    _ context: UnsafeMutableRawPointer?,
    _ capacity: UInt64
) -> UnsafeMutableRawPointer?

@_silgen_name("mdlx_mesh_buffer_allocator_new_zone_for_buffers_with_size")
private func mdlx_mesh_buffer_allocator_new_zone_for_buffers_with_size(
    _ context: UnsafeMutableRawPointer?,
    _ sizes: UnsafePointer<UInt64>?,
    _ types: UnsafePointer<UInt32>?,
    _ count: UInt64
) -> UnsafeMutableRawPointer?

@_silgen_name("mdlx_mesh_buffer_allocator_new_buffer")
private func mdlx_mesh_buffer_allocator_new_buffer(
    _ context: UnsafeMutableRawPointer?,
    _ length: UInt64,
    _ bufferTypeRaw: UInt32
) -> UnsafeMutableRawPointer?

@_silgen_name("mdlx_mesh_buffer_allocator_new_buffer_with_data")
private func mdlx_mesh_buffer_allocator_new_buffer_with_data(
    _ context: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ count: UInt64,
    _ bufferTypeRaw: UInt32
) -> UnsafeMutableRawPointer?

@_silgen_name("mdlx_mesh_buffer_allocator_new_buffer_from_zone_length")
private func mdlx_mesh_buffer_allocator_new_buffer_from_zone_length(
    _ context: UnsafeMutableRawPointer?,
    _ zone: UnsafeMutableRawPointer?,
    _ length: UInt64,
    _ bufferTypeRaw: UInt32
) -> UnsafeMutableRawPointer?

@_silgen_name("mdlx_mesh_buffer_allocator_new_buffer_from_zone_data")
private func mdlx_mesh_buffer_allocator_new_buffer_from_zone_data(
    _ context: UnsafeMutableRawPointer?,
    _ zone: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ count: UInt64,
    _ bufferTypeRaw: UInt32
) -> UnsafeMutableRawPointer?

@_silgen_name("mdlx_mesh_buffer_allocator_release")
private func mdlx_mesh_buffer_allocator_release(_ context: UnsafeMutableRawPointer?)

private final class RustMeshBufferAllocator: NSObject, MDLMeshBufferAllocator {
    private let callbackContext: UnsafeMutableRawPointer?
    private let fallbackAllocator = MDLMeshBufferDataAllocator()

    init(callbackContext: UnsafeMutableRawPointer?) {
        self.callbackContext = callbackContext
        super.init()
    }

    deinit {
        mdlx_mesh_buffer_allocator_release(callbackContext)
    }

    func newZone(_ capacity: Int) -> any MDLMeshBufferZone {
        guard let zone = mdl_take_retained_object(
            mdlx_mesh_buffer_allocator_new_zone(callbackContext, UInt64(capacity))
        ) as? any MDLMeshBufferZone else {
            return fallbackAllocator.newZone(capacity)
        }
        return zone
    }

    func newZoneForBuffers(withSize sizes: [NSNumber], andType types: [NSNumber]) -> any MDLMeshBufferZone {
        guard sizes.count == types.count else {
            return fallbackAllocator.newZoneForBuffers(withSize: sizes, andType: types)
        }
        let rawSizes = sizes.map(\.uint64Value)
        let rawTypes = types.map(\.uint32Value)
        let handle = rawSizes.withUnsafeBufferPointer { sizeBuffer in
            rawTypes.withUnsafeBufferPointer { typeBuffer in
                mdlx_mesh_buffer_allocator_new_zone_for_buffers_with_size(
                    callbackContext,
                    sizeBuffer.baseAddress,
                    typeBuffer.baseAddress,
                    UInt64(rawSizes.count)
                )
            }
        }
        guard let zone = mdl_take_retained_object(handle) as? any MDLMeshBufferZone else {
            return fallbackAllocator.newZoneForBuffers(withSize: sizes, andType: types)
        }
        return zone
    }

    func newBuffer(_ length: Int, type: MDLMeshBufferType) -> any MDLMeshBuffer {
        guard let buffer = mdl_take_retained_object(
            mdlx_mesh_buffer_allocator_new_buffer(
                callbackContext,
                UInt64(length),
                UInt32(type.rawValue)
            )
        ) as? any MDLMeshBuffer else {
            return fallbackAllocator.newBuffer(length, type: type)
        }
        return buffer
    }

    func newBuffer(with data: Data, type: MDLMeshBufferType) -> any MDLMeshBuffer {
        let handle = data.withUnsafeBytes { rawBuffer in
            mdlx_mesh_buffer_allocator_new_buffer_with_data(
                callbackContext,
                rawBuffer.bindMemory(to: UInt8.self).baseAddress,
                UInt64(data.count),
                UInt32(type.rawValue)
            )
        }
        guard let buffer = mdl_take_retained_object(handle) as? any MDLMeshBuffer else {
            return fallbackAllocator.newBuffer(with: data, type: type)
        }
        return buffer
    }

    func newBuffer(from zone: (any MDLMeshBufferZone)?, length: Int, type: MDLMeshBufferType) -> (any MDLMeshBuffer)? {
        let zoneHandle = zone.map { mdl_retain($0 as AnyObject) }
        let handle = mdlx_mesh_buffer_allocator_new_buffer_from_zone_length(
            callbackContext,
            zoneHandle,
            UInt64(length),
            UInt32(type.rawValue)
        )
        if let buffer = mdl_take_retained_object(handle) as? any MDLMeshBuffer {
            return buffer
        }
        return fallbackAllocator.newBuffer(from: zone, length: length, type: type)
    }

    func newBuffer(from zone: (any MDLMeshBufferZone)?, data: Data, type: MDLMeshBufferType) -> (any MDLMeshBuffer)? {
        let zoneHandle = zone.map { mdl_retain($0 as AnyObject) }
        let handle = data.withUnsafeBytes { rawBuffer in
            mdlx_mesh_buffer_allocator_new_buffer_from_zone_data(
                callbackContext,
                zoneHandle,
                rawBuffer.bindMemory(to: UInt8.self).baseAddress,
                UInt64(data.count),
                UInt32(type.rawValue)
            )
        }
        if let buffer = mdl_take_retained_object(handle) as? any MDLMeshBuffer {
            return buffer
        }
        return fallbackAllocator.newBuffer(from: zone, data: data, type: type)
    }
}

private func mdl_mesh_buffer_type(_ rawValue: UInt32) throws -> MDLMeshBufferType {
    guard let bufferType = MDLMeshBufferType(rawValue: UInt(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLMeshBufferType: \(rawValue)")
    }
    return bufferType
}

private func mdl_mesh_buffer(_ handle: UnsafeMutableRawPointer?) -> (any MDLMeshBuffer)? {
    mdl_borrow_object(handle) as? any MDLMeshBuffer
}

private func mdl_mesh_buffer_allocator_object(_ handle: UnsafeMutableRawPointer?) -> (any MDLMeshBufferAllocator)? {
    mdl_borrow_object(handle) as? any MDLMeshBufferAllocator
}

private func mdl_mesh_buffer_zone_object(_ handle: UnsafeMutableRawPointer?) -> (any MDLMeshBufferZone)? {
    mdl_borrow_object(handle) as? any MDLMeshBufferZone
}

@_cdecl("mdl_mesh_buffer_fill_data")
public func mdl_mesh_buffer_fill_data(
    _ handle: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ count: UInt64,
    _ offset: UInt64
) {
    guard let buffer = mdl_mesh_buffer(handle),
          let bytes,
          count > 0
    else {
        return
    }
    buffer.fill(Data(bytes: bytes, count: Int(count)), offset: Int(offset))
}

@_cdecl("mdl_mesh_buffer_map")
public func mdl_mesh_buffer_map(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let buffer = mdl_mesh_buffer(handle) else { return nil }
    return mdl_retain(buffer.map())
}

@_cdecl("mdl_mesh_buffer_allocator")
public func mdl_mesh_buffer_allocator(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let buffer = mdl_mesh_buffer(handle) else { return nil }
    return mdl_retain(buffer.allocator as AnyObject)
}

@_cdecl("mdl_mesh_buffer_zone")
public func mdl_mesh_buffer_zone(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let buffer = mdl_mesh_buffer(handle) else { return nil }
    guard let zone = (buffer as AnyObject).value(forKey: "zone") as? AnyObject else {
        return nil
    }
    return mdl_retain(zone)
}

@_cdecl("mdl_mesh_buffer_is_data")
public func mdl_mesh_buffer_is_data(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let buffer = mdl_borrow_object(handle) else { return 0 }
    return buffer is MDLMeshBufferData ? 1 : 0
}

@_cdecl("mdl_mesh_buffer_data_new")
public func mdl_mesh_buffer_data_new(
    _ length: UInt64,
    _ bufferTypeRaw: UInt32,
    _ outBuffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outBuffer else {
            throw ModelIOBridgeError.invalidArgument("missing output mesh buffer pointer")
        }
        outBuffer.pointee = mdl_retain(MDLMeshBufferData(type: try mdl_mesh_buffer_type(bufferTypeRaw), length: Int(length)))
    }
}

@_cdecl("mdl_mesh_buffer_data_new_with_bytes")
public func mdl_mesh_buffer_data_new_with_bytes(
    _ bytes: UnsafePointer<UInt8>?,
    _ count: UInt64,
    _ bufferTypeRaw: UInt32,
    _ outBuffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outBuffer else {
            throw ModelIOBridgeError.invalidArgument("missing output mesh buffer pointer")
        }
        let data = bytes.map { Data(bytes: $0, count: Int(count)) } ?? Data()
        outBuffer.pointee = mdl_retain(MDLMeshBufferData(type: try mdl_mesh_buffer_type(bufferTypeRaw), data: data))
    }
}

@_cdecl("mdl_mesh_buffer_data_copy_data")
public func mdl_mesh_buffer_data_copy_data(
    _ handle: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UInt8>?,
    _ capacity: UInt64
) -> UInt64 {
    guard let buffer = mdl_borrow_object(handle) as? MDLMeshBufferData else { return 0 }
    return mdl_copy_data(buffer.data, to: outBytes, capacity: capacity)
}

@_cdecl("mdl_mesh_buffer_map_copy_bytes")
public func mdl_mesh_buffer_map_copy_bytes(
    _ handle: UnsafeMutableRawPointer?,
    _ length: UInt64,
    _ outBytes: UnsafeMutablePointer<UInt8>?,
    _ capacity: UInt64
) -> UInt64 {
    guard let map = mdl_borrow_object(handle) as? MDLMeshBufferMap else {
        return 0
    }
    let source = map.bytes
    let byteCount = min(Int(length), Int(capacity))
    guard byteCount > 0 else { return 0 }
    memcpy(outBytes, source, byteCount)
    return UInt64(byteCount)
}

@_cdecl("mdl_mesh_buffer_map_write_bytes")
public func mdl_mesh_buffer_map_write_bytes(
    _ handle: UnsafeMutableRawPointer?,
    _ length: UInt64,
    _ bytes: UnsafePointer<UInt8>?,
    _ count: UInt64,
    _ offset: UInt64
) -> UInt64 {
    guard let map = mdl_borrow_object(handle) as? MDLMeshBufferMap,
          let bytes,
          offset < length
    else {
        return 0
    }
    let destination = map.bytes
    let writeCount = min(Int(count), Int(length - offset))
    guard writeCount > 0 else { return 0 }
    memcpy(destination.advanced(by: Int(offset)), bytes, writeCount)
    return UInt64(writeCount)
}

@_cdecl("mdl_mesh_buffer_allocator_new_with_callback")
public func mdl_mesh_buffer_allocator_new_with_callback(
    _ callbackContext: UnsafeMutableRawPointer?,
    _ outAllocator: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outAllocator else {
            throw ModelIOBridgeError.invalidArgument("missing output allocator pointer")
        }
        outAllocator.pointee = mdl_retain(RustMeshBufferAllocator(callbackContext: callbackContext))
    }
}

@_cdecl("mdl_mesh_buffer_allocator_new_zone")
public func mdl_mesh_buffer_allocator_new_zone(
    _ handle: UnsafeMutableRawPointer?,
    _ capacity: UInt64,
    _ outZone: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let allocator = mdl_mesh_buffer_allocator_object(handle),
              let outZone
        else {
            throw ModelIOBridgeError.invalidArgument("missing allocator or output zone pointer")
        }
        outZone.pointee = mdl_retain(allocator.newZone(Int(capacity)) as AnyObject)
    }
}

@_cdecl("mdl_mesh_buffer_allocator_new_zone_for_buffers_with_size")
public func mdl_mesh_buffer_allocator_new_zone_for_buffers_with_size(
    _ handle: UnsafeMutableRawPointer?,
    _ sizes: UnsafePointer<UInt64>?,
    _ types: UnsafePointer<UInt32>?,
    _ count: UInt64,
    _ outZone: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let allocator = mdl_mesh_buffer_allocator_object(handle),
              let outZone,
              let sizes,
              let types
        else {
            throw ModelIOBridgeError.invalidArgument("missing allocator, sizes, types, or output zone pointer")
        }
        let sizeValues = (0..<Int(count)).map { NSNumber(value: sizes[$0]) }
        let typeValues = try (0..<Int(count)).map { index in
            NSNumber(value: try mdl_mesh_buffer_type(types[index]).rawValue)
        }
        outZone.pointee = mdl_retain(
            allocator.newZoneForBuffers(withSize: sizeValues, andType: typeValues) as AnyObject
        )
    }
}

@_cdecl("mdl_mesh_buffer_allocator_new_buffer")
public func mdl_mesh_buffer_allocator_new_buffer(
    _ handle: UnsafeMutableRawPointer?,
    _ length: UInt64,
    _ bufferTypeRaw: UInt32,
    _ outBuffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let allocator = mdl_mesh_buffer_allocator_object(handle),
              let outBuffer
        else {
            throw ModelIOBridgeError.invalidArgument("missing allocator or output buffer pointer")
        }
        outBuffer.pointee = mdl_retain(allocator.newBuffer(Int(length), type: try mdl_mesh_buffer_type(bufferTypeRaw)) as AnyObject)
    }
}

@_cdecl("mdl_mesh_buffer_allocator_new_buffer_with_data")
public func mdl_mesh_buffer_allocator_new_buffer_with_data(
    _ handle: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ count: UInt64,
    _ bufferTypeRaw: UInt32,
    _ outBuffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let allocator = mdl_mesh_buffer_allocator_object(handle),
              let outBuffer
        else {
            throw ModelIOBridgeError.invalidArgument("missing allocator or output buffer pointer")
        }
        let data = bytes.map { Data(bytes: $0, count: Int(count)) } ?? Data()
        outBuffer.pointee = mdl_retain(allocator.newBuffer(with: data, type: try mdl_mesh_buffer_type(bufferTypeRaw)) as AnyObject)
    }
}

@_cdecl("mdl_mesh_buffer_allocator_new_buffer_from_zone_length")
public func mdl_mesh_buffer_allocator_new_buffer_from_zone_length(
    _ handle: UnsafeMutableRawPointer?,
    _ zoneHandle: UnsafeMutableRawPointer?,
    _ length: UInt64,
    _ bufferTypeRaw: UInt32,
    _ outBuffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let allocator = mdl_mesh_buffer_allocator_object(handle),
              let outBuffer
        else {
            throw ModelIOBridgeError.invalidArgument("missing allocator or output buffer pointer")
        }
        let zone = mdl_mesh_buffer_zone_object(zoneHandle)
        outBuffer.pointee = (try allocator.newBuffer(from: zone, length: Int(length), type: mdl_mesh_buffer_type(bufferTypeRaw)))
            .map { mdl_retain($0 as AnyObject) }
    }
}

@_cdecl("mdl_mesh_buffer_allocator_new_buffer_from_zone_data")
public func mdl_mesh_buffer_allocator_new_buffer_from_zone_data(
    _ handle: UnsafeMutableRawPointer?,
    _ zoneHandle: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ count: UInt64,
    _ bufferTypeRaw: UInt32,
    _ outBuffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let allocator = mdl_mesh_buffer_allocator_object(handle),
              let outBuffer
        else {
            throw ModelIOBridgeError.invalidArgument("missing allocator or output buffer pointer")
        }
        let zone = mdl_mesh_buffer_zone_object(zoneHandle)
        let data = bytes.map { Data(bytes: $0, count: Int(count)) } ?? Data()
        outBuffer.pointee = (try allocator.newBuffer(from: zone, data: data, type: mdl_mesh_buffer_type(bufferTypeRaw)))
            .map { mdl_retain($0 as AnyObject) }
    }
}

@_cdecl("mdl_mesh_buffer_data_allocator_new")
public func mdl_mesh_buffer_data_allocator_new(
    _ outAllocator: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outAllocator else {
            throw ModelIOBridgeError.invalidArgument("missing output allocator pointer")
        }
        outAllocator.pointee = mdl_retain(MDLMeshBufferDataAllocator())
    }
}

@_cdecl("mdl_mesh_buffer_zone_capacity")
public func mdl_mesh_buffer_zone_capacity(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let zone = mdl_mesh_buffer_zone_object(handle) else { return 0 }
    return UInt64(zone.capacity)
}

@_cdecl("mdl_mesh_buffer_zone_allocator")
public func mdl_mesh_buffer_zone_allocator(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let zone = mdl_mesh_buffer_zone_object(handle) else { return nil }
    return mdl_retain(zone.allocator as AnyObject)
}

@_cdecl("mdl_mesh_buffer_zone_is_default")
public func mdl_mesh_buffer_zone_is_default(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let zone = mdl_borrow_object(handle) else { return 0 }
    return zone is MDLMeshBufferZoneDefault ? 1 : 0
}

@_cdecl("mdl_mesh_buffer_zone_default_new")
public func mdl_mesh_buffer_zone_default_new(
    _ outZone: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outZone else {
            throw ModelIOBridgeError.invalidArgument("missing output zone pointer")
        }
        outZone.pointee = mdl_retain(MDLMeshBufferZoneDefault())
    }
}
