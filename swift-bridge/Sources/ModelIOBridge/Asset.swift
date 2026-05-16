import Foundation
import ModelIO
import simd

private func mdl_asset_info(_ asset: MDLAsset) -> [String: Any] {
    var info: [String: Any] = [
        "count": asset.count,
        "frame_interval": asset.frameInterval,
        "start_time": asset.startTime,
        "end_time": asset.endTime,
        "bounding_box": mdl_bounding_box(asset.boundingBox),
        "up_axis": [asset.upAxis.x, asset.upAxis.y, asset.upAxis.z],
    ]
    if let url = asset.url {
        info["url"] = url.path
    }
    return info
}

@_cdecl("mdl_asset_new_empty")
public func mdl_asset_new_empty(
    _ outAsset: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outAsset else {
            throw ModelIOBridgeError.invalidArgument("missing output asset pointer")
        }
        outAsset.pointee = mdl_retain(MDLAsset(bufferAllocator: nil))
    }
}

@_cdecl("mdl_asset_new_with_url")
public func mdl_asset_new_with_url(
    _ path: UnsafePointer<CChar>?,
    _ outAsset: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let path, let outAsset else {
            throw ModelIOBridgeError.invalidArgument("missing asset path or output pointer")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        outAsset.pointee = mdl_retain(MDLAsset(url: url))
    }
}

@_cdecl("mdl_asset_can_import_file_extension")
public func mdl_asset_can_import_file_extension(_ pathExtension: UnsafePointer<CChar>?) -> Int32 {
    guard let pathExtension else { return 0 }
    return MDLAsset.canImportFileExtension(String(cString: pathExtension)) ? 1 : 0
}

@_cdecl("mdl_asset_can_export_file_extension")
public func mdl_asset_can_export_file_extension(_ pathExtension: UnsafePointer<CChar>?) -> Int32 {
    guard let pathExtension else { return 0 }
    return MDLAsset.canExportFileExtension(String(cString: pathExtension)) ? 1 : 0
}

@_cdecl("mdl_asset_info_json")
public func mdl_asset_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return nil }
    return mdl_string(mdl_json_string(from: mdl_asset_info(asset)) ?? "{}")
}

@_cdecl("mdl_asset_count")
public func mdl_asset_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return 0 }
    return UInt64(asset.count)
}

@_cdecl("mdl_asset_bounding_box")
public func mdl_asset_bounding_box(
    _ handle: UnsafeMutableRawPointer?,
    _ outMinX: UnsafeMutablePointer<Float>?,
    _ outMinY: UnsafeMutablePointer<Float>?,
    _ outMinZ: UnsafeMutablePointer<Float>?,
    _ outMaxX: UnsafeMutablePointer<Float>?,
    _ outMaxY: UnsafeMutablePointer<Float>?,
    _ outMaxZ: UnsafeMutablePointer<Float>?
) {
    let zero = MDLAxisAlignedBoundingBox(maxBounds: SIMD3<Float>(repeating: 0), minBounds: SIMD3<Float>(repeating: 0))
    let boundingBox = (mdl_borrow_object(handle) as? MDLAsset)?.boundingBox ?? zero
    outMinX?.pointee = boundingBox.minBounds.x
    outMinY?.pointee = boundingBox.minBounds.y
    outMinZ?.pointee = boundingBox.minBounds.z
    outMaxX?.pointee = boundingBox.maxBounds.x
    outMaxY?.pointee = boundingBox.maxBounds.y
    outMaxZ?.pointee = boundingBox.maxBounds.z
}

@_cdecl("mdl_asset_bounding_box_at_time")
public func mdl_asset_bounding_box_at_time(
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
    let boundingBox = (mdl_borrow_object(handle) as? MDLAsset)?.boundingBox(atTime: time) ?? zero
    outMinX?.pointee = boundingBox.minBounds.x
    outMinY?.pointee = boundingBox.minBounds.y
    outMinZ?.pointee = boundingBox.minBounds.z
    outMaxX?.pointee = boundingBox.maxBounds.x
    outMaxY?.pointee = boundingBox.maxBounds.y
    outMaxZ?.pointee = boundingBox.maxBounds.z
}

@_cdecl("mdl_asset_url_string")
public func mdl_asset_url_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset,
          let url = asset.url
    else {
        return nil
    }
    return mdl_string(url.path)
}

@_cdecl("mdl_asset_object_at_index")
public func mdl_asset_object_at_index(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset,
          index < UInt64(asset.count)
    else {
        return nil
    }
    return mdl_retain(asset.object(at: Int(index)))
}

@_cdecl("mdl_asset_mesh_at_index")
public func mdl_asset_mesh_at_index(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt64
) -> UnsafeMutableRawPointer? {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset,
          index < UInt64(asset.count),
          let mesh = asset.object(at: Int(index)) as? MDLMesh
    else {
        return nil
    }
    return mdl_retain(mesh)
}

@_cdecl("mdl_asset_object_at_path")
public func mdl_asset_object_at_path(
    _ handle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset,
          let path
    else {
        return nil
    }
    return mdl_retain(asset.object(atPath: String(cString: path)))
}

@_cdecl("mdl_asset_add_object")
public func mdl_asset_add_object(_ handle: UnsafeMutableRawPointer?, _ objectHandle: UnsafeMutableRawPointer?) {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset,
          let object = mdl_borrow_object(objectHandle) as? MDLObject
    else {
        return
    }
    asset.add(object)
}

@_cdecl("mdl_asset_remove_object")
public func mdl_asset_remove_object(_ handle: UnsafeMutableRawPointer?, _ objectHandle: UnsafeMutableRawPointer?) {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset,
          let object = mdl_borrow_object(objectHandle) as? MDLObject
    else {
        return
    }
    asset.remove(object)
}

@_cdecl("mdl_asset_load_textures")
public func mdl_asset_load_textures(_ handle: UnsafeMutableRawPointer?) {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return }
    asset.loadTextures()
}

@_cdecl("mdl_asset_export_to_url")
public func mdl_asset_export_to_url(
    _ handle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let asset = mdl_borrow_object(handle) as? MDLAsset,
              let path
        else {
            throw ModelIOBridgeError.invalidArgument("missing asset or export path")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        try asset.export(to: url)
    }
}

@_cdecl("mdl_asset_frame_interval")
public func mdl_asset_frame_interval(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return 0 }
    return asset.frameInterval
}

@_cdecl("mdl_asset_set_frame_interval")
public func mdl_asset_set_frame_interval(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return }
    asset.frameInterval = value
}

@_cdecl("mdl_asset_start_time")
public func mdl_asset_start_time(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return 0 }
    return asset.startTime
}

@_cdecl("mdl_asset_set_start_time")
public func mdl_asset_set_start_time(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return }
    asset.startTime = value
}

@_cdecl("mdl_asset_end_time")
public func mdl_asset_end_time(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return 0 }
    return asset.endTime
}

@_cdecl("mdl_asset_set_end_time")
public func mdl_asset_set_end_time(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return }
    asset.endTime = value
}

@_cdecl("mdl_asset_up_axis")
public func mdl_asset_up_axis(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Float>?,
    _ outY: UnsafeMutablePointer<Float>?,
    _ outZ: UnsafeMutablePointer<Float>?
) {
    let upAxis = (mdl_borrow_object(handle) as? MDLAsset)?.upAxis ?? SIMD3<Float>(0, 1, 0)
    outX?.pointee = upAxis.x
    outY?.pointee = upAxis.y
    outZ?.pointee = upAxis.z
}

@_cdecl("mdl_asset_set_up_axis")
public func mdl_asset_set_up_axis(_ handle: UnsafeMutableRawPointer?, _ x: Float, _ y: Float, _ z: Float) {
    guard let asset = mdl_borrow_object(handle) as? MDLAsset else { return }
    asset.upAxis = SIMD3<Float>(x, y, z)
}
