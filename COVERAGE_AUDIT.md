# modelio-rs coverage audit (vs MacOSX26.2.sdk)

Method: unique top-level ModelIO public classes, protocols, enums, structs, and exported SDK constants from the macOS 26.2 SDK headers.

SDK_PUBLIC_SYMBOLS: 117  
VERIFIED: 117  
GAPS: 0  
EXEMPT: 0  
COVERAGE_PCT: 100.0%

## Result

All 41 previously-audited gaps are now wrapped in `v0.2.2`.

## Newly closed gap groups

### Protocols and shared types
- `MDLJointAnimation`
- `MDLComponent`
- `MDLNamed`
- `MDLObjectContainerComponent`
- `MDLMatrix4x4Array`

### Cameras and lights
- `MDLStereoscopicCamera`
- `MDLAreaLight`
- `MDLPhotometricLight`

### Materials
- `MDLMaterialTextureWrapMode`
- `MDLMaterialTextureFilterMode`
- `MDLMaterialMipMapFilterMode`
- `MDLTextureFilter`
- `MDLTextureSampler`
- `MDLMaterialPropertyConnection`
- `MDLMaterialPropertyNode`
- `MDLMaterialPropertyGraph`

### Objects, submeshes, and vertex layout
- `MDLObjectContainer`
- `MDLSubmeshTopology`
- `MDLVertexBufferLayout`

### SDK constants and utility helpers
- `kUTType3dObject`
- `kUTTypeAlembic`
- `kUTTypePolygon`
- `kUTTypeStereolithography`
- `kUTTypeUniversalSceneDescription`
- `kUTTypeUniversalSceneDescriptionMobile`
- `MDLVertexAttributeAnisotropy`
- `MDLVertexAttributeBinormal`
- `MDLVertexAttributeBitangent`
- `MDLVertexAttributeColor`
- `MDLVertexAttributeEdgeCrease`
- `MDLVertexAttributeJointIndices`
- `MDLVertexAttributeJointWeights`
- `MDLVertexAttributeNormal`
- `MDLVertexAttributeOcclusionValue`
- `MDLVertexAttributePosition`
- `MDLVertexAttributeShadingBasisU`
- `MDLVertexAttributeShadingBasisV`
- `MDLVertexAttributeSubdivisionStencil`
- `MDLVertexAttributeTangent`
- `MDLVertexAttributeTextureCoordinate`
- `MDLUtility`

## Validation

- `tests/api_coverage.rs` verifies the bridge symbols and SDK header surface
- Integration tests cover the newly-added sampler/filter, material-property, matrix-array, constant, and utility APIs

## Remaining gaps

_None._
