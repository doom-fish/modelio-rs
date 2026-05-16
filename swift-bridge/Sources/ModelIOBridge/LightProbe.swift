import Foundation
import ModelIO
import simd

@_silgen_name("mdlx_light_probe_irradiance_data_source_coefficients")
private func mdlx_light_probe_irradiance_data_source_coefficients(
    _ context: UnsafeMutableRawPointer?,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ outValues: UnsafeMutablePointer<Float>?,
    _ capacity: UInt64
) -> UInt64

@_silgen_name("mdlx_light_probe_irradiance_data_source_release")
private func mdlx_light_probe_irradiance_data_source_release(_ context: UnsafeMutableRawPointer?)

private func mdl_probe_placement(_ rawValue: Int32) throws -> MDLProbePlacement {
    guard let placement = MDLProbePlacement(rawValue: Int(rawValue)) else {
        throw ModelIOBridgeError.invalidArgument("invalid MDLProbePlacement: \(rawValue)")
    }
    return placement
}

private final class RustLightProbeIrradianceDataSource: NSObject, MDLLightProbeIrradianceDataSource {
    var boundingBox: MDLAxisAlignedBoundingBox
    var sphericalHarmonicsLevel: UInt
    private let callbackContext: UnsafeMutableRawPointer?

    init(
        boundingBox: MDLAxisAlignedBoundingBox,
        sphericalHarmonicsLevel: UInt,
        callbackContext: UnsafeMutableRawPointer?
    ) {
        self.boundingBox = boundingBox
        self.sphericalHarmonicsLevel = sphericalHarmonicsLevel
        self.callbackContext = callbackContext
        super.init()
    }

    deinit {
        mdlx_light_probe_irradiance_data_source_release(callbackContext)
    }

    func sphericalHarmonicsCoefficients(atPosition position: SIMD3<Float>) -> Data {
        let valueCount = Int(
            mdlx_light_probe_irradiance_data_source_coefficients(
                callbackContext,
                position.x,
                position.y,
                position.z,
                nil,
                0
            )
        )
        guard valueCount > 0 else { return Data() }
        var values = [Float](repeating: 0, count: valueCount)
        let written = values.withUnsafeMutableBufferPointer { buffer in
            mdlx_light_probe_irradiance_data_source_coefficients(
                callbackContext,
                position.x,
                position.y,
                position.z,
                buffer.baseAddress,
                UInt64(buffer.count)
            )
        }
        let clampedCount = min(valueCount, Int(written))
        return values.withUnsafeBufferPointer { buffer in
            Data(bytes: buffer.baseAddress!, count: clampedCount * MemoryLayout<Float>.stride)
        }
    }
}

@_cdecl("mdl_light_probe_irradiance_data_source_new")
public func mdl_light_probe_irradiance_data_source_new(
    _ minX: Float,
    _ minY: Float,
    _ minZ: Float,
    _ maxX: Float,
    _ maxY: Float,
    _ maxZ: Float,
    _ sphericalHarmonicsLevel: UInt64,
    _ callbackContext: UnsafeMutableRawPointer?,
    _ outDataSource: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outDataSource else {
            throw ModelIOBridgeError.invalidArgument("missing output data source pointer")
        }
        let dataSource = RustLightProbeIrradianceDataSource(
            boundingBox: MDLAxisAlignedBoundingBox(
                maxBounds: SIMD3<Float>(maxX, maxY, maxZ),
                minBounds: SIMD3<Float>(minX, minY, minZ)
            ),
            sphericalHarmonicsLevel: UInt(sphericalHarmonicsLevel),
            callbackContext: callbackContext
        )
        outDataSource.pointee = mdl_retain(dataSource)
    }
}

@_cdecl("mdl_light_probe_irradiance_data_source_bounding_box")
public func mdl_light_probe_irradiance_data_source_bounding_box(
    _ handle: UnsafeMutableRawPointer?,
    _ outMinX: UnsafeMutablePointer<Float>?,
    _ outMinY: UnsafeMutablePointer<Float>?,
    _ outMinZ: UnsafeMutablePointer<Float>?,
    _ outMaxX: UnsafeMutablePointer<Float>?,
    _ outMaxY: UnsafeMutablePointer<Float>?,
    _ outMaxZ: UnsafeMutablePointer<Float>?
) {
    guard let dataSource = mdl_borrow_object(handle) as? RustLightProbeIrradianceDataSource else { return }
    outMinX?.pointee = dataSource.boundingBox.minBounds.x
    outMinY?.pointee = dataSource.boundingBox.minBounds.y
    outMinZ?.pointee = dataSource.boundingBox.minBounds.z
    outMaxX?.pointee = dataSource.boundingBox.maxBounds.x
    outMaxY?.pointee = dataSource.boundingBox.maxBounds.y
    outMaxZ?.pointee = dataSource.boundingBox.maxBounds.z
}

@_cdecl("mdl_light_probe_irradiance_data_source_set_bounding_box")
public func mdl_light_probe_irradiance_data_source_set_bounding_box(
    _ handle: UnsafeMutableRawPointer?,
    _ minX: Float,
    _ minY: Float,
    _ minZ: Float,
    _ maxX: Float,
    _ maxY: Float,
    _ maxZ: Float
) {
    guard let dataSource = mdl_borrow_object(handle) as? RustLightProbeIrradianceDataSource else { return }
    dataSource.boundingBox = MDLAxisAlignedBoundingBox(
        maxBounds: SIMD3<Float>(maxX, maxY, maxZ),
        minBounds: SIMD3<Float>(minX, minY, minZ)
    )
}

@_cdecl("mdl_light_probe_irradiance_data_source_spherical_harmonics_level")
public func mdl_light_probe_irradiance_data_source_spherical_harmonics_level(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let dataSource = mdl_borrow_object(handle) as? RustLightProbeIrradianceDataSource else {
        return 0
    }
    return UInt64(dataSource.sphericalHarmonicsLevel)
}

@_cdecl("mdl_light_probe_irradiance_data_source_set_spherical_harmonics_level")
public func mdl_light_probe_irradiance_data_source_set_spherical_harmonics_level(
    _ handle: UnsafeMutableRawPointer?,
    _ sphericalHarmonicsLevel: UInt64
) {
    guard let dataSource = mdl_borrow_object(handle) as? RustLightProbeIrradianceDataSource else {
        return
    }
    dataSource.sphericalHarmonicsLevel = UInt(sphericalHarmonicsLevel)
}

@_cdecl("mdl_asset_place_light_probes")
public func mdl_asset_place_light_probes(
    _ density: Float,
    _ heuristicRaw: Int32,
    _ dataSourceHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let dataSource = mdl_borrow_object(dataSourceHandle) as? RustLightProbeIrradianceDataSource,
          let heuristic = try? mdl_probe_placement(heuristicRaw)
    else {
        return nil
    }
    let probes = MDLAsset.placeLightProbes(withDensity: density, heuristic: heuristic, using: dataSource)
    return mdl_retain(probes as NSArray)
}

@_cdecl("mdl_light_probe_new")
public func mdl_light_probe_new(
    _ reflectiveTextureHandle: UnsafeMutableRawPointer?,
    _ irradianceTextureHandle: UnsafeMutableRawPointer?,
    _ outProbe: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outProbe else {
            throw ModelIOBridgeError.invalidArgument("missing output light probe pointer")
        }
        let reflectiveTexture = mdl_borrow_object(reflectiveTextureHandle) as? MDLTexture
        let irradianceTexture = mdl_borrow_object(irradianceTextureHandle) as? MDLTexture
        outProbe.pointee = mdl_retain(
            MDLLightProbe(reflectiveTexture: reflectiveTexture, irradianceTexture: irradianceTexture)
        )
    }
}

@_cdecl("mdl_light_probe_generate_spherical_harmonics_from_irradiance")
public func mdl_light_probe_generate_spherical_harmonics_from_irradiance(
    _ handle: UnsafeMutableRawPointer?,
    _ sphericalHarmonicsLevel: UInt64
) {
    guard let probe = mdl_borrow_object(handle) as? MDLLightProbe else { return }
    probe.generateSphericalHarmonics(fromIrradiance: Int(sphericalHarmonicsLevel))
}

@_cdecl("mdl_light_probe_reflective_texture")
public func mdl_light_probe_reflective_texture(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let probe = mdl_borrow_object(handle) as? MDLLightProbe,
          let texture = probe.reflectiveTexture
    else {
        return nil
    }
    return mdl_retain(texture)
}

@_cdecl("mdl_light_probe_irradiance_texture")
public func mdl_light_probe_irradiance_texture(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let probe = mdl_borrow_object(handle) as? MDLLightProbe,
          let texture = probe.irradianceTexture
    else {
        return nil
    }
    return mdl_retain(texture)
}

@_cdecl("mdl_light_probe_spherical_harmonics_level")
public func mdl_light_probe_spherical_harmonics_level(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let probe = mdl_borrow_object(handle) as? MDLLightProbe else { return 0 }
    return UInt64(probe.sphericalHarmonicsLevel)
}

@_cdecl("mdl_light_probe_spherical_harmonics_coefficient_count")
public func mdl_light_probe_spherical_harmonics_coefficient_count(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let probe = mdl_borrow_object(handle) as? MDLLightProbe,
          let coefficients = probe.sphericalHarmonicsCoefficients
    else {
        return 0
    }
    return UInt64(coefficients.count / MemoryLayout<Float>.stride)
}

@_cdecl("mdl_light_probe_copy_spherical_harmonics_coefficients")
public func mdl_light_probe_copy_spherical_harmonics_coefficients(
    _ handle: UnsafeMutableRawPointer?,
    _ outValues: UnsafeMutablePointer<Float>?,
    _ capacity: UInt64
) -> UInt64 {
    guard let probe = mdl_borrow_object(handle) as? MDLLightProbe,
          let coefficients = probe.sphericalHarmonicsCoefficients,
          let outValues
    else {
        return 0
    }
    let valueCount = min(Int(capacity), coefficients.count / MemoryLayout<Float>.stride)
    guard valueCount > 0 else { return 0 }
    coefficients.withUnsafeBytes { rawBuffer in
        guard let baseAddress = rawBuffer.baseAddress?.assumingMemoryBound(to: Float.self) else {
            return
        }
        outValues.initialize(from: baseAddress, count: valueCount)
    }
    return UInt64(valueCount)
}
