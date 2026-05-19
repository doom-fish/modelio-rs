# modelio-rs coverage audit (vs MacOSX26.5.sdk)

Method: unique top-level ModelIO public classes, protocols, enums, structs, and exported SDK constants from the macOS 26.5 SDK headers.

SDK_PUBLIC_SYMBOLS: 117  
VERIFIED: 117  
GAPS: 0  
EXEMPT: 0  
COVERAGE_PCT: 100.0%

## Result

Top-level coverage remains complete in `v0.3.0`, and the last protocol/class abstractions that were only covered indirectly are now exposed as first-class Rust wrappers.

## v0.3.0 wrapper promotions

### Protocol adapters
- `MDLAssetResolver`
- `MDLMeshBufferAllocator`
- `MDLTransformComponent`
- `MDLTransformOp`

### Material scattering classes
- `MDLScatteringFunction`
- `MDLPhysicallyPlausibleScatteringFunction`

## Validation

- `tests/api_coverage.rs` verifies the active SDK header surface and the bridge symbols for callback-backed protocol constructors and scattering-function thunks
- Integration tests cover custom resolvers, custom mesh-buffer allocators, scattering functions, and callback-backed transform components/ops

## Remaining gaps

_None._
