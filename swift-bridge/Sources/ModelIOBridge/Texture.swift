import CoreGraphics
import Foundation
import ModelIO
import simd

private func mdl_texture_info(_ texture: MDLTexture) -> [String: Any] {
    var info: [String: Any] = [
        "name": texture.name,
        "dimensions": [texture.dimensions.x, texture.dimensions.y],
        "row_stride": texture.rowStride,
        "channel_count": texture.channelCount,
        "mip_level_count": texture.mipLevelCount,
        "channel_encoding": texture.channelEncoding.rawValue,
        "is_cube": texture.isCube,
        "has_alpha_values": texture.hasAlphaValues,
    ]
    if let urlTexture = texture as? MDLURLTexture {
        info["url"] = urlTexture.url.absoluteString
    }
    return info
}

@_cdecl("mdl_url_texture_new")
public func mdl_url_texture_new(
    _ path: UnsafePointer<CChar>?,
    _ name: UnsafePointer<CChar>?,
    _ outTexture: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let path, let outTexture else {
            throw ModelIOBridgeError.invalidArgument("missing texture path or output pointer")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        let textureName = name.map { String(cString: $0) }
        outTexture.pointee = mdl_retain(MDLURLTexture(url: url, name: textureName))
    }
}

@_cdecl("mdl_checkerboard_texture_new")
public func mdl_checkerboard_texture_new(
    _ divisions: Float,
    _ name: UnsafePointer<CChar>?,
    _ width: Int32,
    _ height: Int32,
    _ channelCount: UInt64,
    _ channelEncodingRaw: Int32,
    _ color1R: Float,
    _ color1G: Float,
    _ color1B: Float,
    _ color1A: Float,
    _ color2R: Float,
    _ color2G: Float,
    _ color2B: Float,
    _ color2A: Float,
    _ outTexture: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outTexture else {
            throw ModelIOBridgeError.invalidArgument("missing output texture pointer")
        }
        let checkerboard = MDLCheckerboardTexture(
            divisions: divisions,
            name: name.map { String(cString: $0) },
            dimensions: vector_int2(width, height),
            channelCount: Int32(channelCount),
            channelEncoding: try mdl_texture_channel_encoding(channelEncodingRaw),
            color1: mdl_color(color1R, color1G, color1B, color1A),
            color2: mdl_color(color2R, color2G, color2B, color2A)
        )
        outTexture.pointee = mdl_retain(checkerboard)
    }
}

@_cdecl("mdl_texture_info_json")
public func mdl_texture_info_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let texture = mdl_borrow_object(handle) as? MDLTexture else { return nil }
    return mdl_string(mdl_json_string(from: mdl_texture_info(texture)) ?? "{}")
}

@_cdecl("mdl_texture_write_to_url")
public func mdl_texture_write_to_url(
    _ handle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let texture = mdl_borrow_object(handle) as? MDLTexture,
              let path
        else {
            throw ModelIOBridgeError.invalidArgument("missing texture or output path")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        guard texture.write(to: url) else {
            throw ModelIOBridgeError.framework(NSError(domain: "modelio", code: -1, userInfo: [NSLocalizedDescriptionKey: "texture write failed"]))
        }
    }
}

@_cdecl("mdl_texture_texel_data_length")
public func mdl_texture_texel_data_length(
    _ handle: UnsafeMutableRawPointer?,
    _ topLeftOrigin: Int32
) -> UInt64 {
    guard let texture = mdl_borrow_object(handle) as? MDLTexture else { return 0 }
    let data = topLeftOrigin != 0 ? texture.texelDataWithTopLeftOrigin() : texture.texelDataWithBottomLeftOrigin()
    return UInt64(data?.count ?? 0)
}

@_cdecl("mdl_texture_copy_texel_data")
public func mdl_texture_copy_texel_data(
    _ handle: UnsafeMutableRawPointer?,
    _ topLeftOrigin: Int32,
    _ outBytes: UnsafeMutablePointer<UInt8>?,
    _ capacity: UInt64
) -> UInt64 {
    guard let texture = mdl_borrow_object(handle) as? MDLTexture,
          let outBytes,
          let data = topLeftOrigin != 0 ? texture.texelDataWithTopLeftOrigin() : texture.texelDataWithBottomLeftOrigin()
    else {
        return 0
    }
    let byteCount = min(Int(capacity), data.count)
    data.copyBytes(to: outBytes, count: byteCount)
    return UInt64(byteCount)
}
