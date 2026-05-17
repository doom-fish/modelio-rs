import Foundation
import ModelIO

@_cdecl("mdl_utility_convert_to_usdz")
public func mdl_utility_convert_to_usdz(
    _ inputURL: UnsafePointer<CChar>?,
    _ outputURL: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let inputURL, let outputURL else {
            throw ModelIOBridgeError.invalidArgument("missing input or output URL")
        }
        if #available(macOS 15.0, *) {
            MDLUtility.convert(
                toUSDZ: URL(fileURLWithPath: String(cString: inputURL)),
                writeTo: URL(fileURLWithPath: String(cString: outputURL))
            )
        } else {
            throw ModelIOBridgeError.invalidArgument("MDLUtility.convertToUSDZ requires macOS 15.0 or newer")
        }
    }
}
