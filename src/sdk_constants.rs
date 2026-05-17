use crate::ffi;
use crate::util::take_string;

const UTTYPE_ALEMBIC: u32 = 1;
const UTTYPE_3D_OBJECT: u32 = 2;
const UTTYPE_POLYGON: u32 = 3;
const UTTYPE_STEREOLITHOGRAPHY: u32 = 4;
const UTTYPE_UNIVERSAL_SCENE_DESCRIPTION: u32 = 5;
const UTTYPE_UNIVERSAL_SCENE_DESCRIPTION_MOBILE: u32 = 6;
const VERTEX_ATTRIBUTE_ANISOTROPY: u32 = 101;
const VERTEX_ATTRIBUTE_BINORMAL: u32 = 102;
const VERTEX_ATTRIBUTE_BITANGENT: u32 = 103;
const VERTEX_ATTRIBUTE_COLOR: u32 = 104;
const VERTEX_ATTRIBUTE_EDGE_CREASE: u32 = 105;
const VERTEX_ATTRIBUTE_JOINT_INDICES: u32 = 106;
const VERTEX_ATTRIBUTE_JOINT_WEIGHTS: u32 = 107;
const VERTEX_ATTRIBUTE_NORMAL: u32 = 108;
const VERTEX_ATTRIBUTE_OCCLUSION_VALUE: u32 = 109;
const VERTEX_ATTRIBUTE_POSITION: u32 = 110;
const VERTEX_ATTRIBUTE_SHADING_BASIS_U: u32 = 111;
const VERTEX_ATTRIBUTE_SHADING_BASIS_V: u32 = 112;
const VERTEX_ATTRIBUTE_SUBDIVISION_STENCIL: u32 = 113;
const VERTEX_ATTRIBUTE_TANGENT: u32 = 114;
const VERTEX_ATTRIBUTE_TEXTURE_COORDINATE: u32 = 115;

fn sdk_constant(code: u32) -> Option<String> {
    take_string(unsafe { ffi::mdl_sdk_constant_string(code) })
}

pub mod ut_type {
    use super::{
        sdk_constant, UTTYPE_3D_OBJECT, UTTYPE_ALEMBIC, UTTYPE_POLYGON, UTTYPE_STEREOLITHOGRAPHY,
        UTTYPE_UNIVERSAL_SCENE_DESCRIPTION, UTTYPE_UNIVERSAL_SCENE_DESCRIPTION_MOBILE,
    };

    #[must_use]
    pub fn alembic() -> Option<String> {
        sdk_constant(UTTYPE_ALEMBIC)
    }

    #[must_use]
    pub fn object_3d() -> Option<String> {
        sdk_constant(UTTYPE_3D_OBJECT)
    }

    #[must_use]
    pub fn polygon() -> Option<String> {
        sdk_constant(UTTYPE_POLYGON)
    }

    #[must_use]
    pub fn stereolithography() -> Option<String> {
        sdk_constant(UTTYPE_STEREOLITHOGRAPHY)
    }

    #[must_use]
    pub fn universal_scene_description() -> Option<String> {
        sdk_constant(UTTYPE_UNIVERSAL_SCENE_DESCRIPTION)
    }

    #[must_use]
    pub fn universal_scene_description_mobile() -> Option<String> {
        sdk_constant(UTTYPE_UNIVERSAL_SCENE_DESCRIPTION_MOBILE)
    }
}

pub mod vertex_attribute_name {
    use super::{
        sdk_constant, VERTEX_ATTRIBUTE_ANISOTROPY, VERTEX_ATTRIBUTE_BINORMAL,
        VERTEX_ATTRIBUTE_BITANGENT, VERTEX_ATTRIBUTE_COLOR, VERTEX_ATTRIBUTE_EDGE_CREASE,
        VERTEX_ATTRIBUTE_JOINT_INDICES, VERTEX_ATTRIBUTE_JOINT_WEIGHTS, VERTEX_ATTRIBUTE_NORMAL,
        VERTEX_ATTRIBUTE_OCCLUSION_VALUE, VERTEX_ATTRIBUTE_POSITION,
        VERTEX_ATTRIBUTE_SHADING_BASIS_U, VERTEX_ATTRIBUTE_SHADING_BASIS_V,
        VERTEX_ATTRIBUTE_SUBDIVISION_STENCIL, VERTEX_ATTRIBUTE_TANGENT,
        VERTEX_ATTRIBUTE_TEXTURE_COORDINATE,
    };

    #[must_use]
    pub fn anisotropy() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_ANISOTROPY)
    }
    #[must_use]
    pub fn binormal() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_BINORMAL)
    }
    #[must_use]
    pub fn bitangent() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_BITANGENT)
    }
    #[must_use]
    pub fn color() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_COLOR)
    }
    #[must_use]
    pub fn edge_crease() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_EDGE_CREASE)
    }
    #[must_use]
    pub fn joint_indices() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_JOINT_INDICES)
    }
    #[must_use]
    pub fn joint_weights() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_JOINT_WEIGHTS)
    }
    #[must_use]
    pub fn normal() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_NORMAL)
    }
    #[must_use]
    pub fn occlusion_value() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_OCCLUSION_VALUE)
    }
    #[must_use]
    pub fn position() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_POSITION)
    }
    #[must_use]
    pub fn shading_basis_u() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_SHADING_BASIS_U)
    }
    #[must_use]
    pub fn shading_basis_v() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_SHADING_BASIS_V)
    }
    #[must_use]
    pub fn subdivision_stencil() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_SUBDIVISION_STENCIL)
    }
    #[must_use]
    pub fn tangent() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_TANGENT)
    }
    #[must_use]
    pub fn texture_coordinate() -> Option<String> {
        sdk_constant(VERTEX_ATTRIBUTE_TEXTURE_COORDINATE)
    }
}
