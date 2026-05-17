import Foundation
import ModelIO

private let mdlConstantUTTypeAlembic: UInt32 = 1
private let mdlConstantUTType3dObject: UInt32 = 2
private let mdlConstantUTTypePolygon: UInt32 = 3
private let mdlConstantUTTypeStereolithography: UInt32 = 4
private let mdlConstantUTTypeUniversalSceneDescription: UInt32 = 5
private let mdlConstantUTTypeUniversalSceneDescriptionMobile: UInt32 = 6
private let mdlConstantVertexAttributeAnisotropy: UInt32 = 101
private let mdlConstantVertexAttributeBinormal: UInt32 = 102
private let mdlConstantVertexAttributeBitangent: UInt32 = 103
private let mdlConstantVertexAttributeColor: UInt32 = 104
private let mdlConstantVertexAttributeEdgeCrease: UInt32 = 105
private let mdlConstantVertexAttributeJointIndices: UInt32 = 106
private let mdlConstantVertexAttributeJointWeights: UInt32 = 107
private let mdlConstantVertexAttributeNormal: UInt32 = 108
private let mdlConstantVertexAttributeOcclusionValue: UInt32 = 109
private let mdlConstantVertexAttributePosition: UInt32 = 110
private let mdlConstantVertexAttributeShadingBasisU: UInt32 = 111
private let mdlConstantVertexAttributeShadingBasisV: UInt32 = 112
private let mdlConstantVertexAttributeSubdivisionStencil: UInt32 = 113
private let mdlConstantVertexAttributeTangent: UInt32 = 114
private let mdlConstantVertexAttributeTextureCoordinate: UInt32 = 115

private func mdl_sdk_constant(_ code: UInt32) -> String? {
    switch code {
    case mdlConstantUTTypeAlembic:
        return kUTTypeAlembic as String
    case mdlConstantUTType3dObject:
        return kUTType3dObject as String
    case mdlConstantUTTypePolygon:
        return kUTTypePolygon as String
    case mdlConstantUTTypeStereolithography:
        return kUTTypeStereolithography as String
    case mdlConstantUTTypeUniversalSceneDescription:
        return kUTTypeUniversalSceneDescription as String
    case mdlConstantUTTypeUniversalSceneDescriptionMobile:
        return kUTTypeUniversalSceneDescriptionMobile as String
    case mdlConstantVertexAttributeAnisotropy:
        return MDLVertexAttributeAnisotropy as String
    case mdlConstantVertexAttributeBinormal:
        return MDLVertexAttributeBinormal as String
    case mdlConstantVertexAttributeBitangent:
        return MDLVertexAttributeBitangent as String
    case mdlConstantVertexAttributeColor:
        return MDLVertexAttributeColor as String
    case mdlConstantVertexAttributeEdgeCrease:
        return MDLVertexAttributeEdgeCrease as String
    case mdlConstantVertexAttributeJointIndices:
        return MDLVertexAttributeJointIndices as String
    case mdlConstantVertexAttributeJointWeights:
        return MDLVertexAttributeJointWeights as String
    case mdlConstantVertexAttributeNormal:
        return MDLVertexAttributeNormal as String
    case mdlConstantVertexAttributeOcclusionValue:
        return MDLVertexAttributeOcclusionValue as String
    case mdlConstantVertexAttributePosition:
        return MDLVertexAttributePosition as String
    case mdlConstantVertexAttributeShadingBasisU:
        return MDLVertexAttributeShadingBasisU as String
    case mdlConstantVertexAttributeShadingBasisV:
        return MDLVertexAttributeShadingBasisV as String
    case mdlConstantVertexAttributeSubdivisionStencil:
        return MDLVertexAttributeSubdivisionStencil as String
    case mdlConstantVertexAttributeTangent:
        return MDLVertexAttributeTangent as String
    case mdlConstantVertexAttributeTextureCoordinate:
        return MDLVertexAttributeTextureCoordinate as String
    default:
        return nil
    }
}

@_cdecl("mdl_sdk_constant_string")
public func mdl_sdk_constant_string(_ code: UInt32) -> UnsafeMutablePointer<CChar>? {
    guard let constant = mdl_sdk_constant(code) else { return nil }
    return mdl_string(constant)
}
