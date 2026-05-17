import Foundation
import ModelIO
import simd

private let MDLX_OBJECT_KIND_UNKNOWN: Int32 = 0
private let MDLX_OBJECT_KIND_OBJECT: Int32 = 1
private let MDLX_OBJECT_KIND_MESH: Int32 = 2
private let MDLX_OBJECT_KIND_LIGHT: Int32 = 3
private let MDLX_OBJECT_KIND_PHYSICALLY_PLAUSIBLE_LIGHT: Int32 = 4
private let MDLX_OBJECT_KIND_CAMERA: Int32 = 5
private let MDLX_OBJECT_KIND_VOXEL_ARRAY: Int32 = 6
private let MDLX_OBJECT_KIND_SKELETON: Int32 = 7
private let MDLX_OBJECT_KIND_PACKED_JOINT_ANIMATION: Int32 = 8

private func mdl_object_kind_raw(_ object: MDLObject) -> Int32 {
    if object is MDLPhysicallyPlausibleLight {
        return MDLX_OBJECT_KIND_PHYSICALLY_PLAUSIBLE_LIGHT
    }
    if object is MDLCamera {
        return MDLX_OBJECT_KIND_CAMERA
    }
    if object is MDLVoxelArray {
        return MDLX_OBJECT_KIND_VOXEL_ARRAY
    }
    if object is MDLSkeleton {
        return MDLX_OBJECT_KIND_SKELETON
    }
    if object is MDLPackedJointAnimation {
        return MDLX_OBJECT_KIND_PACKED_JOINT_ANIMATION
    }
    if object is MDLLight {
        return MDLX_OBJECT_KIND_LIGHT
    }
    if object is MDLMesh {
        return MDLX_OBJECT_KIND_MESH
    }
    return MDLX_OBJECT_KIND_OBJECT
}

private func mdl_object_info(_ object: MDLObject) -> [String: Any] {
    [
        "kind": mdl_object_kind_raw(object),
        "name": object.name,
        "path": object.path,
        "hidden": object.hidden,
        "component_count": object.components.count,
        "child_count": object.children.count,
        "has_parent": object.parent != nil,
        "has_instance": object.instance != nil,
        "bounding_box": mdl_bounding_box(object.boundingBox(atTime: 0)),
    ]
}

private func mdl_object_at_path_impl(_ object: MDLObject, _ path: String) -> MDLObject? {
    let selector = NSSelectorFromString("objectAtPath:")
    guard object.responds(to: selector),
          let unmanaged = object.perform(selector, with: NSString(string: path)),
          let resolved = unmanaged.takeUnretainedValue() as? MDLObject
    else {
        return nil
    }
    return resolved
}

@_cdecl("mdl_object_new")
public func mdl_object_new(
    _ outObject: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outObject else {
            throw ModelIOBridgeError.invalidArgument("missing output object pointer")
        }
        outObject.pointee = mdl_retain(MDLObject())
    }
}

@_cdecl("mdl_object_info_json")
public func mdl_object_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let object = mdl_borrow_object(handle) as? MDLObject else { return nil }
    return mdl_string(mdl_json_string(from: mdl_object_info(object)) ?? "{}")
}

@_cdecl("mdl_object_kind")
public func mdl_object_kind(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let object = mdl_borrow_object(handle) as? MDLObject else {
        return MDLX_OBJECT_KIND_UNKNOWN
    }
    return mdl_object_kind_raw(object)
}

@_cdecl("mdl_object_name_string")
public func mdl_object_name_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let object = mdl_borrow_object(handle) as? MDLObject else { return nil }
    return mdl_string(object.name)
}

@_cdecl("mdl_object_set_name")
public func mdl_object_set_name(_ handle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let object = mdl_borrow_object(handle) as? MDLObject,
          let name
    else {
        return
    }
    object.name = String(cString: name)
}

@_cdecl("mdl_object_path_string")
public func mdl_object_path_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let object = mdl_borrow_object(handle) as? MDLObject else { return nil }
    return mdl_string(object.path)
}

@_cdecl("mdl_object_hidden")
public func mdl_object_hidden(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let object = mdl_borrow_object(handle) as? MDLObject else { return 0 }
    return object.hidden ? 1 : 0
}

@_cdecl("mdl_object_set_hidden")
public func mdl_object_set_hidden(_ handle: UnsafeMutableRawPointer?, _ hidden: Int32) {
    guard let object = mdl_borrow_object(handle) as? MDLObject else { return }
    object.hidden = hidden != 0
}

@_cdecl("mdl_object_add_child")
public func mdl_object_add_child(_ handle: UnsafeMutableRawPointer?, _ childHandle: UnsafeMutableRawPointer?) {
    guard let object = mdl_borrow_object(handle) as? MDLObject,
          let child = mdl_borrow_object(childHandle) as? MDLObject
    else {
        return
    }
    object.addChild(child)
}

@_cdecl("mdl_object_children_container")
public func mdl_object_children_container(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let object = mdl_borrow_object(handle) as? MDLObject,
          let container = object.children as? MDLObjectContainer
    else {
        return nil
    }
    return mdl_retain(container)
}

@_cdecl("mdl_object_set_children_container")
public func mdl_object_set_children_container(_ handle: UnsafeMutableRawPointer?, _ containerHandle: UnsafeMutableRawPointer?) {
    guard let object = mdl_borrow_object(handle) as? MDLObject else { return }
    object.children = (mdl_borrow_object(containerHandle) as? MDLObjectContainer) ?? MDLObjectContainer()
}

@_cdecl("mdl_object_child_count")
public func mdl_object_child_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let object = mdl_borrow_object(handle) as? MDLObject else { return 0 }
    return UInt64(object.children.count)
}

@_cdecl("mdl_object_child_at")
public func mdl_object_child_at(_ handle: UnsafeMutableRawPointer?, _ index: UInt64) -> UnsafeMutableRawPointer? {
    guard let object = mdl_borrow_object(handle) as? MDLObject,
          index < UInt64(object.children.count)
    else {
        return nil
    }
    return mdl_retain(object.children[Int(index)])
}

@_cdecl("mdl_object_at_path")
public func mdl_object_at_path(_ handle: UnsafeMutableRawPointer?, _ path: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let object = mdl_borrow_object(handle) as? MDLObject,
          let path
    else {
        return nil
    }

    let query = String(cString: path)
    var candidates: [String] = [query]

    if query.hasPrefix(object.path + "/") {
        candidates.append(String(query.dropFirst(object.path.count + 1)))
    }
    if query.hasPrefix("/") {
        candidates.append(String(query.dropFirst()))
    }
    if let lastComponent = query.split(separator: "/").last {
        candidates.append(String(lastComponent))
    }

    for candidate in candidates where !candidate.isEmpty {
        if let resolved = mdl_object_at_path_impl(object, candidate) {
            return mdl_retain(resolved)
        }
    }

    return nil
}

@_cdecl("mdl_object_bounding_box_at_time")
public func mdl_object_bounding_box_at_time(
    _ handle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ outMinX: UnsafeMutablePointer<Float>?,
    _ outMinY: UnsafeMutablePointer<Float>?,
    _ outMinZ: UnsafeMutablePointer<Float>?,
    _ outMaxX: UnsafeMutablePointer<Float>?,
    _ outMaxY: UnsafeMutablePointer<Float>?,
    _ outMaxZ: UnsafeMutablePointer<Float>?
) {
    let zero = MDLAxisAlignedBoundingBox(maxBounds: SIMD3<Float>(repeating: 0), minBounds: SIMD3<Float>(repeating: 0))
    let boundingBox = (mdl_borrow_object(handle) as? MDLObject)?.boundingBox(atTime: time) ?? zero
    outMinX?.pointee = boundingBox.minBounds.x
    outMinY?.pointee = boundingBox.minBounds.y
    outMinZ?.pointee = boundingBox.minBounds.z
    outMaxX?.pointee = boundingBox.maxBounds.x
    outMaxY?.pointee = boundingBox.maxBounds.y
    outMaxZ?.pointee = boundingBox.maxBounds.z
}

@_cdecl("mdl_object_container_new")
public func mdl_object_container_new(
    _ outContainer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outContainer else {
            throw ModelIOBridgeError.invalidArgument("missing output object container pointer")
        }
        outContainer.pointee = mdl_retain(MDLObjectContainer())
    }
}

@_cdecl("mdl_object_container_count")
public func mdl_object_container_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let container = mdl_borrow_object(handle) as? MDLObjectContainer else { return 0 }
    return UInt64(container.count)
}

@_cdecl("mdl_object_container_object_at")
public func mdl_object_container_object_at(_ handle: UnsafeMutableRawPointer?, _ index: UInt64) -> UnsafeMutableRawPointer? {
    guard let container = mdl_borrow_object(handle) as? MDLObjectContainer,
          index < UInt64(container.count)
    else {
        return nil
    }
    return mdl_retain(container.objects[Int(index)])
}

@_cdecl("mdl_object_container_add_object")
public func mdl_object_container_add_object(_ handle: UnsafeMutableRawPointer?, _ objectHandle: UnsafeMutableRawPointer?) {
    guard let container = mdl_borrow_object(handle) as? MDLObjectContainer,
          let object = mdl_borrow_object(objectHandle) as? MDLObject
    else {
        return
    }
    container.add(object)
}

@_cdecl("mdl_object_container_remove_object")
public func mdl_object_container_remove_object(_ handle: UnsafeMutableRawPointer?, _ objectHandle: UnsafeMutableRawPointer?) {
    guard let container = mdl_borrow_object(handle) as? MDLObjectContainer,
          let object = mdl_borrow_object(objectHandle) as? MDLObject
    else {
        return
    }
    container.remove(object)
}
