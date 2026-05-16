import Foundation
import ModelIO
import simd

private func mdl_vertex_attribute_info(_ attribute: MDLVertexAttribute) -> [String: Any] {
    [
        "name": attribute.name,
        "format": attribute.format.rawValue,
        "offset": attribute.offset,
        "buffer_index": attribute.bufferIndex,
        "time": attribute.time,
        "initialization_value": [attribute.initializationValue.x, attribute.initializationValue.y, attribute.initializationValue.z, attribute.initializationValue.w],
    ]
}

private func mdl_vertex_descriptor_info(_ descriptor: MDLVertexDescriptor) -> [String: Any] {
    let attributes = descriptor.attributes.compactMap { $0 as? MDLVertexAttribute }.map(mdl_vertex_attribute_info)
    let layoutStrides = descriptor.layouts.compactMap { ($0 as? MDLVertexBufferLayout)?.stride }
    return [
        "attribute_count": attributes.count,
        "attributes": attributes,
        "layout_strides": layoutStrides,
    ]
}

@_cdecl("mdl_vertex_attribute_new")
public func mdl_vertex_attribute_new(
    _ name: UnsafePointer<CChar>?,
    _ formatRaw: UInt32,
    _ offset: UInt64,
    _ bufferIndex: UInt64,
    _ outAttribute: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let name, let outAttribute else {
            throw ModelIOBridgeError.invalidArgument("missing vertex attribute name or output pointer")
        }
        outAttribute.pointee = mdl_retain(
            MDLVertexAttribute(
                name: String(cString: name),
                format: try mdl_vertex_format(formatRaw),
                offset: Int(offset),
                bufferIndex: Int(bufferIndex)
            )
        )
    }
}

@_cdecl("mdl_vertex_attribute_info_json")
public func mdl_vertex_attribute_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let attribute = mdl_borrow_object(handle) as? MDLVertexAttribute else { return nil }
    return mdl_string(mdl_json_string(from: mdl_vertex_attribute_info(attribute)) ?? "{}")
}

@_cdecl("mdl_vertex_attribute_set_name")
public func mdl_vertex_attribute_set_name(_ handle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let attribute = mdl_borrow_object(handle) as? MDLVertexAttribute,
          let name
    else {
        return
    }
    attribute.name = String(cString: name)
}

@_cdecl("mdl_vertex_attribute_set_format")
public func mdl_vertex_attribute_set_format(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) {
    guard let attribute = mdl_borrow_object(handle) as? MDLVertexAttribute,
          let format = try? mdl_vertex_format(rawValue)
    else {
        return
    }
    attribute.format = format
}

@_cdecl("mdl_vertex_attribute_set_offset")
public func mdl_vertex_attribute_set_offset(_ handle: UnsafeMutableRawPointer?, _ offset: UInt64) {
    guard let attribute = mdl_borrow_object(handle) as? MDLVertexAttribute else { return }
    attribute.offset = Int(offset)
}

@_cdecl("mdl_vertex_attribute_set_buffer_index")
public func mdl_vertex_attribute_set_buffer_index(_ handle: UnsafeMutableRawPointer?, _ bufferIndex: UInt64) {
    guard let attribute = mdl_borrow_object(handle) as? MDLVertexAttribute else { return }
    attribute.bufferIndex = Int(bufferIndex)
}

@_cdecl("mdl_vertex_attribute_set_time")
public func mdl_vertex_attribute_set_time(_ handle: UnsafeMutableRawPointer?, _ time: Double) {
    guard let attribute = mdl_borrow_object(handle) as? MDLVertexAttribute else { return }
    attribute.time = time
}

@_cdecl("mdl_vertex_attribute_set_initialization_value")
public func mdl_vertex_attribute_set_initialization_value(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float, _ w: Float) {
    guard let attribute = mdl_borrow_object(handle) as? MDLVertexAttribute else { return }
    attribute.initializationValue = SIMD4<Float>(x, y, z, w)
}

@_cdecl("mdl_vertex_descriptor_new_copy")
public func mdl_vertex_descriptor_new_copy(
    _ handle: UnsafeMutableRawPointer?,
    _ outDescriptor: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let descriptor = mdl_borrow_object(handle) as? MDLVertexDescriptor,
              let outDescriptor
        else {
            throw ModelIOBridgeError.invalidArgument("missing source vertex descriptor or output pointer")
        }
        outDescriptor.pointee = mdl_retain(MDLVertexDescriptor(vertexDescriptor: descriptor))
    }
}

@_cdecl("mdl_vertex_descriptor_info_json")
public func mdl_vertex_descriptor_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptor = mdl_borrow_object(handle) as? MDLVertexDescriptor else { return nil }
    return mdl_string(mdl_json_string(from: mdl_vertex_descriptor_info(descriptor)) ?? "{}")
}

@_cdecl("mdl_vertex_descriptor_attribute_count")
public func mdl_vertex_descriptor_attribute_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let descriptor = mdl_borrow_object(handle) as? MDLVertexDescriptor else { return 0 }
    return UInt64(descriptor.attributes.count)
}

@_cdecl("mdl_vertex_descriptor_attribute_at")
public func mdl_vertex_descriptor_attribute_at(_ handle: UnsafeMutableRawPointer?, _ index: UInt64) -> UnsafeMutableRawPointer? {
    guard let descriptor = mdl_borrow_object(handle) as? MDLVertexDescriptor,
          index < UInt64(descriptor.attributes.count),
          let attribute = descriptor.attributes[Int(index)] as? MDLVertexAttribute
    else {
        return nil
    }
    return mdl_retain(attribute)
}

@_cdecl("mdl_vertex_descriptor_attribute_named")
public func mdl_vertex_descriptor_attribute_named(_ handle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let descriptor = mdl_borrow_object(handle) as? MDLVertexDescriptor,
          let name,
          let attribute = descriptor.attributeNamed(String(cString: name))
    else {
        return nil
    }
    return mdl_retain(attribute)
}

@_cdecl("mdl_vertex_descriptor_reset")
public func mdl_vertex_descriptor_reset(_ handle: UnsafeMutableRawPointer?) {
    guard let descriptor = mdl_borrow_object(handle) as? MDLVertexDescriptor else { return }
    descriptor.reset()
}

@_cdecl("mdl_vertex_descriptor_set_packed_offsets")
public func mdl_vertex_descriptor_set_packed_offsets(_ handle: UnsafeMutableRawPointer?) {
    guard let descriptor = mdl_borrow_object(handle) as? MDLVertexDescriptor else { return }
    descriptor.setPackedOffsets()
}

@_cdecl("mdl_vertex_descriptor_set_packed_strides")
public func mdl_vertex_descriptor_set_packed_strides(_ handle: UnsafeMutableRawPointer?) {
    guard let descriptor = mdl_borrow_object(handle) as? MDLVertexDescriptor else { return }
    descriptor.setPackedStrides()
}
