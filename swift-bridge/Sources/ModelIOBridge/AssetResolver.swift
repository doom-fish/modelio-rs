import Darwin
import Foundation
import ModelIO

@_silgen_name("mdlx_asset_resolver_can_resolve_named")
private func mdlx_asset_resolver_can_resolve_named(
    _ context: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> Int32

@_silgen_name("mdlx_asset_resolver_resolve_named")
private func mdlx_asset_resolver_resolve_named(
    _ context: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>?

@_silgen_name("mdlx_asset_resolver_release")
private func mdlx_asset_resolver_release(_ context: UnsafeMutableRawPointer?)

private final class RustAssetResolver: NSObject, MDLAssetResolver {
    private let callbackContext: UnsafeMutableRawPointer?

    init(callbackContext: UnsafeMutableRawPointer?) {
        self.callbackContext = callbackContext
        super.init()
    }

    deinit {
        mdlx_asset_resolver_release(callbackContext)
    }

    func canResolveAssetNamed(_ name: String) -> Bool {
        name.withCString { mdlx_asset_resolver_can_resolve_named(callbackContext, $0) != 0 }
    }

    func resolveAssetNamed(_ name: String) -> URL {
        let pointer = name.withCString { mdlx_asset_resolver_resolve_named(callbackContext, $0) }
        guard let pointer else {
            return URL(fileURLWithPath: name)
        }
        defer { free(pointer) }
        let value = String(cString: pointer)
        if let url = URL(string: value), url.scheme != nil {
            return url
        }
        return URL(fileURLWithPath: value)
    }
}

private func mdl_asset_resolver(_ handle: UnsafeMutableRawPointer?) -> (any MDLAssetResolver)? {
    mdl_borrow_object(handle) as? any MDLAssetResolver
}

@_cdecl("mdl_asset_resolver_new_with_callback")
public func mdl_asset_resolver_new_with_callback(
    _ callbackContext: UnsafeMutableRawPointer?,
    _ outResolver: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let outResolver else {
            throw ModelIOBridgeError.invalidArgument("missing output resolver pointer")
        }
        outResolver.pointee = mdl_retain(RustAssetResolver(callbackContext: callbackContext))
    }
}

@_cdecl("mdl_asset_resolver_can_resolve_named")
public func mdl_asset_resolver_can_resolve_named(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> Int32 {
    guard let resolver = mdl_asset_resolver(handle),
          let name
    else {
        return 0
    }
    return resolver.canResolveAssetNamed(String(cString: name)) ? 1 : 0
}

@_cdecl("mdl_asset_resolver_resolve_named")
public func mdl_asset_resolver_resolve_named(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let resolver = mdl_asset_resolver(handle),
          let name,
          resolver.canResolveAssetNamed(String(cString: name))
    else {
        return nil
    }
    return mdl_string(resolver.resolveAssetNamed(String(cString: name)).absoluteString)
}

@_cdecl("mdl_path_asset_resolver_new")
public func mdl_path_asset_resolver_new(
    _ path: UnsafePointer<CChar>?,
    _ outResolver: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let path, let outResolver else {
            throw ModelIOBridgeError.invalidArgument("missing path or output resolver pointer")
        }
        outResolver.pointee = mdl_retain(MDLPathAssetResolver(path: String(cString: path)))
    }
}

@_cdecl("mdl_path_asset_resolver_path")
public func mdl_path_asset_resolver_path(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let resolver = mdl_borrow_object(handle) as? MDLPathAssetResolver else { return nil }
    return mdl_string(resolver.path)
}

@_cdecl("mdl_path_asset_resolver_set_path")
public func mdl_path_asset_resolver_set_path(_ handle: UnsafeMutableRawPointer?, _ path: UnsafePointer<CChar>?) {
    guard let resolver = mdl_borrow_object(handle) as? MDLPathAssetResolver,
          let path
    else {
        return
    }
    resolver.path = String(cString: path)
}

@_cdecl("mdl_bundle_asset_resolver_new")
public func mdl_bundle_asset_resolver_new(
    _ path: UnsafePointer<CChar>?,
    _ outResolver: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let path, let outResolver else {
            throw ModelIOBridgeError.invalidArgument("missing path or output resolver pointer")
        }
        outResolver.pointee = mdl_retain(MDLBundleAssetResolver(bundle: String(cString: path)))
    }
}

@_cdecl("mdl_bundle_asset_resolver_path")
public func mdl_bundle_asset_resolver_path(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let resolver = mdl_borrow_object(handle) as? MDLBundleAssetResolver else { return nil }
    return mdl_string(resolver.path)
}

@_cdecl("mdl_bundle_asset_resolver_set_path")
public func mdl_bundle_asset_resolver_set_path(_ handle: UnsafeMutableRawPointer?, _ path: UnsafePointer<CChar>?) {
    guard let resolver = mdl_borrow_object(handle) as? MDLBundleAssetResolver,
          let path
    else {
        return
    }
    resolver.path = String(cString: path)
}

@_cdecl("mdl_relative_asset_resolver_new")
public func mdl_relative_asset_resolver_new(
    _ assetHandle: UnsafeMutableRawPointer?,
    _ outResolver: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    mdl_run(outError) {
        guard let asset = mdl_borrow_object(assetHandle) as? MDLAsset,
              let outResolver
        else {
            throw ModelIOBridgeError.invalidArgument("missing asset or output resolver pointer")
        }
        outResolver.pointee = mdl_retain(MDLRelativeAssetResolver(asset: asset))
    }
}

@_cdecl("mdl_relative_asset_resolver_asset")
public func mdl_relative_asset_resolver_asset(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let resolver = mdl_borrow_object(handle) as? MDLRelativeAssetResolver,
          let asset = resolver.asset
    else {
        return nil
    }
    return mdl_retain(asset)
}

@_cdecl("mdl_relative_asset_resolver_set_asset")
public func mdl_relative_asset_resolver_set_asset(
    _ handle: UnsafeMutableRawPointer?,
    _ assetHandle: UnsafeMutableRawPointer?
) {
    guard let resolver = mdl_borrow_object(handle) as? MDLRelativeAssetResolver else { return }
    resolver.asset = mdl_borrow_object(assetHandle) as? MDLAsset
}
