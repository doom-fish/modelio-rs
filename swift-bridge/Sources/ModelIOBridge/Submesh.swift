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

@_cdecl("mdl_submesh_topology")
public func mdl_submesh_topology(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh,
          let topology = submesh.topology
    else {
        return nil
    }
    return mdl_retain(topology)
}

@_cdecl("mdl_submesh_set_topology")
public func mdl_submesh_set_topology(_ handle: UnsafeMutableRawPointer?, _ topologyHandle: UnsafeMutableRawPointer?) {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return }
    submesh.topology = mdl_borrow_object(topologyHandle) as? MDLSubmeshTopology
}

@_cdecl("mdl_submesh_set_material")
public func mdl_submesh_set_material(_ handle: UnsafeMutableRawPointer?, _ materialHandle: UnsafeMutableRawPointer?) {
    guard let submesh = mdl_borrow_object(handle) as? MDLSubmesh else { return }
    submesh.material = mdl_borrow_object(materialHandle) as? MDLMaterial
}

private func mdl_submesh_topology_buffer(_ handle: UnsafeMutableRawPointer?, keyPath: ReferenceWritableKeyPath<MDLSubmeshTopology, (any MDLMeshBuffer)?>) -> UnsafeMutableRawPointer? {
    guard let topology = mdl_borrow_object(handle) as? MDLSubmeshTopology,
          let buffer = topology[keyPath: keyPath]
    else {
        return nil
    }
    return mdl_retain(buffer as AnyObject)
}

private func mdl_set_submesh_topology_buffer(
    _ handle: UnsafeMutableRawPointer?,
    _ bufferHandle: UnsafeMutableRawPointer?,
    keyPath: ReferenceWritableKeyPath<MDLSubmeshTopology, (any MDLMeshBuffer)?>
) {
    guard let topology = mdl_borrow_object(handle) as? MDLSubmeshTopology else { return }
    topology[keyPath: keyPath] = mdl_borrow_object(bufferHandle) as? any MDLMeshBuffer
}

@_cdecl("mdl_submesh_topology_new")
public func mdl_submesh_topology_new(
    _ submeshHandle: UnsafeMutableRawPointer?,
    _ outTopology: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let submesh = mdl_borrow_object(submeshHandle) as? MDLSubmesh,
              let outTopology
        else {
            throw ModelIOBridgeError.invalidArgument("missing submesh or output topology pointer")
        }
        outTopology.pointee = mdl_retain(MDLSubmeshTopology(submesh: submesh))
    }
}

@_cdecl("mdl_submesh_topology_face_topology")
public func mdl_submesh_topology_face_topology(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    mdl_submesh_topology_buffer(handle, keyPath: \MDLSubmeshTopology.faceTopology)
}

@_cdecl("mdl_submesh_topology_set_face_topology")
public func mdl_submesh_topology_set_face_topology(_ handle: UnsafeMutableRawPointer?, _ bufferHandle: UnsafeMutableRawPointer?) {
    mdl_set_submesh_topology_buffer(handle, bufferHandle, keyPath: \MDLSubmeshTopology.faceTopology)
}

@_cdecl("mdl_submesh_topology_face_count")
public func mdl_submesh_topology_face_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let topology = mdl_borrow_object(handle) as? MDLSubmeshTopology else { return 0 }
    return UInt64(topology.faceCount)
}

@_cdecl("mdl_submesh_topology_set_face_count")
public func mdl_submesh_topology_set_face_count(_ handle: UnsafeMutableRawPointer?, _ count: UInt64) {
    guard let topology = mdl_borrow_object(handle) as? MDLSubmeshTopology else { return }
    topology.faceCount = Int(count)
}

@_cdecl("mdl_submesh_topology_vertex_crease_indices")
public func mdl_submesh_topology_vertex_crease_indices(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    mdl_submesh_topology_buffer(handle, keyPath: \MDLSubmeshTopology.vertexCreaseIndices)
}

@_cdecl("mdl_submesh_topology_set_vertex_crease_indices")
public func mdl_submesh_topology_set_vertex_crease_indices(_ handle: UnsafeMutableRawPointer?, _ bufferHandle: UnsafeMutableRawPointer?) {
    mdl_set_submesh_topology_buffer(handle, bufferHandle, keyPath: \MDLSubmeshTopology.vertexCreaseIndices)
}

@_cdecl("mdl_submesh_topology_vertex_creases")
public func mdl_submesh_topology_vertex_creases(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    mdl_submesh_topology_buffer(handle, keyPath: \MDLSubmeshTopology.vertexCreases)
}

@_cdecl("mdl_submesh_topology_set_vertex_creases")
public func mdl_submesh_topology_set_vertex_creases(_ handle: UnsafeMutableRawPointer?, _ bufferHandle: UnsafeMutableRawPointer?) {
    mdl_set_submesh_topology_buffer(handle, bufferHandle, keyPath: \MDLSubmeshTopology.vertexCreases)
}

@_cdecl("mdl_submesh_topology_vertex_crease_count")
public func mdl_submesh_topology_vertex_crease_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let topology = mdl_borrow_object(handle) as? MDLSubmeshTopology else { return 0 }
    return UInt64(topology.vertexCreaseCount)
}

@_cdecl("mdl_submesh_topology_set_vertex_crease_count")
public func mdl_submesh_topology_set_vertex_crease_count(_ handle: UnsafeMutableRawPointer?, _ count: UInt64) {
    guard let topology = mdl_borrow_object(handle) as? MDLSubmeshTopology else { return }
    topology.vertexCreaseCount = Int(count)
}

@_cdecl("mdl_submesh_topology_edge_crease_indices")
public func mdl_submesh_topology_edge_crease_indices(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    mdl_submesh_topology_buffer(handle, keyPath: \MDLSubmeshTopology.edgeCreaseIndices)
}

@_cdecl("mdl_submesh_topology_set_edge_crease_indices")
public func mdl_submesh_topology_set_edge_crease_indices(_ handle: UnsafeMutableRawPointer?, _ bufferHandle: UnsafeMutableRawPointer?) {
    mdl_set_submesh_topology_buffer(handle, bufferHandle, keyPath: \MDLSubmeshTopology.edgeCreaseIndices)
}

@_cdecl("mdl_submesh_topology_edge_creases")
public func mdl_submesh_topology_edge_creases(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    mdl_submesh_topology_buffer(handle, keyPath: \MDLSubmeshTopology.edgeCreases)
}

@_cdecl("mdl_submesh_topology_set_edge_creases")
public func mdl_submesh_topology_set_edge_creases(_ handle: UnsafeMutableRawPointer?, _ bufferHandle: UnsafeMutableRawPointer?) {
    mdl_set_submesh_topology_buffer(handle, bufferHandle, keyPath: \MDLSubmeshTopology.edgeCreases)
}

@_cdecl("mdl_submesh_topology_edge_crease_count")
public func mdl_submesh_topology_edge_crease_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let topology = mdl_borrow_object(handle) as? MDLSubmeshTopology else { return 0 }
    return UInt64(topology.edgeCreaseCount)
}

@_cdecl("mdl_submesh_topology_set_edge_crease_count")
public func mdl_submesh_topology_set_edge_crease_count(_ handle: UnsafeMutableRawPointer?, _ count: UInt64) {
    guard let topology = mdl_borrow_object(handle) as? MDLSubmeshTopology else { return }
    topology.edgeCreaseCount = Int(count)
}
