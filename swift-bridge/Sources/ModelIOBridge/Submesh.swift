import Foundation
import ModelIO

@_cdecl("mdl_submesh_index_count")
public func mdl_submesh_index_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return 0 }
    return UInt64(submesh.indexCount)
}

@_cdecl("mdl_submesh_index_type")
public func mdl_submesh_index_type(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return 0 }
    return UInt32(submesh.indexType.rawValue)
}

@_cdecl("mdl_submesh_geometry_type")
public func mdl_submesh_geometry_type(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return 0 }
    return Int32(submesh.geometryType.rawValue)
}

@_cdecl("mdl_submesh_name_string")
public func mdl_submesh_name_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return nil }
    return mdl_string(submesh.name)
}

@_cdecl("mdl_submesh_set_name")
public func mdl_submesh_set_name(_ handle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh,
          let name
    else {
        return
    }
    submesh.name = String(cString: name)
}

@_cdecl("mdl_submesh_index_buffer")
public func mdl_submesh_index_buffer(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return nil }
    return mdl_retain(submesh.indexBuffer as AnyObject)
}

@_cdecl("mdl_submesh_index_buffer_as_type")
public func mdl_submesh_index_buffer_as_type(_ handle: UnsafeMutableRawPointer?, _ rawValue: UInt32) -> UnsafeMutableRawPointer? {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh,
          let indexType = try? mdl_index_bit_depth(rawValue)
    else {
        return nil
    }
    return mdl_retain(submesh.indexBuffer(asIndexType: indexType) as AnyObject)
}

@_cdecl("mdl_submesh_material")
public func mdl_submesh_material(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh,
          let material = submesh.material
    else {
        return nil
    }
    return mdl_retain(material)
}

@_cdecl("mdl_submesh_set_material")
public func mdl_submesh_set_material(_ handle: UnsafeMutableRawPointer?, _ materialHandle: UnsafeMutableRawPointer?) {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return }
    submesh.material = mdl_borrow_object(materialHandle) as? MDLMaterial
}
