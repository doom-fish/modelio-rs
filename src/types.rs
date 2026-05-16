use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GeometryType {
    Points = 0,
    Lines = 1,
    Triangles = 2,
    TriangleStrips = 3,
    Quads = 4,
    VariableTopology = 5,
}

impl GeometryType {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Points),
            1 => Some(Self::Lines),
            2 => Some(Self::Triangles),
            3 => Some(Self::TriangleStrips),
            4 => Some(Self::Quads),
            5 => Some(Self::VariableTopology),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum IndexBitDepth {
    UInt8 = 8,
    UInt16 = 16,
    UInt32 = 32,
}

impl IndexBitDepth {
    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            8 => Some(Self::UInt8),
            16 => Some(Self::UInt16),
            32 => Some(Self::UInt32),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MeshBufferType {
    Vertex = 1,
    Index = 2,
    Custom = 3,
}

impl MeshBufferType {
    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            1 => Some(Self::Vertex),
            2 => Some(Self::Index),
            3 => Some(Self::Custom),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MaterialSemantic {
    BaseColor = 0,
    Subsurface = 1,
    Metallic = 2,
    Specular = 3,
    SpecularExponent = 4,
    SpecularTint = 5,
    Roughness = 6,
    Anisotropic = 7,
    AnisotropicRotation = 8,
    Sheen = 9,
    SheenTint = 10,
    Clearcoat = 11,
    ClearcoatGloss = 12,
    Emission = 13,
    Bump = 14,
    Opacity = 15,
    InterfaceIndexOfRefraction = 16,
    MaterialIndexOfRefraction = 17,
    ObjectSpaceNormal = 18,
    TangentSpaceNormal = 19,
    Displacement = 20,
    DisplacementScale = 21,
    AmbientOcclusion = 22,
    AmbientOcclusionScale = 23,
    None = 0x8000,
    UserDefined = 0x8001,
}

impl MaterialSemantic {
    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::BaseColor),
            1 => Some(Self::Subsurface),
            2 => Some(Self::Metallic),
            3 => Some(Self::Specular),
            4 => Some(Self::SpecularExponent),
            5 => Some(Self::SpecularTint),
            6 => Some(Self::Roughness),
            7 => Some(Self::Anisotropic),
            8 => Some(Self::AnisotropicRotation),
            9 => Some(Self::Sheen),
            10 => Some(Self::SheenTint),
            11 => Some(Self::Clearcoat),
            12 => Some(Self::ClearcoatGloss),
            13 => Some(Self::Emission),
            14 => Some(Self::Bump),
            15 => Some(Self::Opacity),
            16 => Some(Self::InterfaceIndexOfRefraction),
            17 => Some(Self::MaterialIndexOfRefraction),
            18 => Some(Self::ObjectSpaceNormal),
            19 => Some(Self::TangentSpaceNormal),
            20 => Some(Self::Displacement),
            21 => Some(Self::DisplacementScale),
            22 => Some(Self::AmbientOcclusion),
            23 => Some(Self::AmbientOcclusionScale),
            0x8000 => Some(Self::None),
            0x8001 => Some(Self::UserDefined),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MaterialPropertyType {
    None = 0,
    String = 1,
    Url = 2,
    Texture = 3,
    Color = 4,
    Float = 5,
    Float2 = 6,
    Float3 = 7,
    Float4 = 8,
    Matrix44 = 9,
    Buffer = 10,
}

impl MaterialPropertyType {
    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::String),
            2 => Some(Self::Url),
            3 => Some(Self::Texture),
            4 => Some(Self::Color),
            5 => Some(Self::Float),
            6 => Some(Self::Float2),
            7 => Some(Self::Float3),
            8 => Some(Self::Float4),
            9 => Some(Self::Matrix44),
            10 => Some(Self::Buffer),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextureChannelEncoding {
    UInt8 = 1,
    UInt16 = 2,
    UInt24 = 3,
    UInt32 = 4,
    Float16 = 0x102,
    Float16Sr = 0x302,
    Float32 = 0x104,
}

impl TextureChannelEncoding {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            1 => Some(Self::UInt8),
            2 => Some(Self::UInt16),
            3 => Some(Self::UInt24),
            4 => Some(Self::UInt32),
            0x102 => Some(Self::Float16),
            0x302 => Some(Self::Float16Sr),
            0x104 => Some(Self::Float32),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeshBufferInfo {
    pub length: usize,
    pub buffer_type: u32,
}

impl MeshBufferInfo {
    #[must_use]
    pub fn buffer_type_enum(&self) -> Option<MeshBufferType> {
        MeshBufferType::from_raw(self.buffer_type)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VertexAttributeInfo {
    pub stride: usize,
    pub format: u32,
    pub buffer_size: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextureInfo {
    pub name: Option<String>,
    pub dimensions: [i32; 2],
    pub row_stride: isize,
    pub channel_count: usize,
    pub channel_encoding: i32,
    pub is_cube: bool,
    pub has_alpha_values: bool,
    pub url: Option<String>,
}

impl TextureInfo {
    #[must_use]
    pub fn channel_encoding_enum(&self) -> Option<TextureChannelEncoding> {
        TextureChannelEncoding::from_raw(self.channel_encoding)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MaterialPropertyInfo {
    pub name: String,
    pub semantic: u32,
    pub property_type: u32,
    pub string_value: Option<String>,
    pub url_value: Option<String>,
    pub float_value: Option<f32>,
    pub float2_value: Option<[f32; 2]>,
    pub float3_value: Option<[f32; 3]>,
    pub float4_value: Option<[f32; 4]>,
    pub matrix4x4: Option<[f32; 16]>,
    pub color: Option<[f32; 4]>,
    pub luminance: Option<f32>,
    pub texture: Option<TextureInfo>,
}

impl MaterialPropertyInfo {
    #[must_use]
    pub fn semantic_enum(&self) -> Option<MaterialSemantic> {
        MaterialSemantic::from_raw(self.semantic)
    }

    #[must_use]
    pub fn property_type_enum(&self) -> Option<MaterialPropertyType> {
        MaterialPropertyType::from_raw(self.property_type)
    }
}

pub mod vertex_format {
    pub const INVALID: u32 = 0;
    pub const PACKED_BIT: u32 = 0x1000;

    pub const UCHAR_BITS: u32 = 0x10000;
    pub const CHAR_BITS: u32 = 0x20000;
    pub const UCHAR_NORMALIZED_BITS: u32 = 0x30000;
    pub const CHAR_NORMALIZED_BITS: u32 = 0x40000;
    pub const USHORT_BITS: u32 = 0x50000;
    pub const SHORT_BITS: u32 = 0x60000;
    pub const USHORT_NORMALIZED_BITS: u32 = 0x70000;
    pub const SHORT_NORMALIZED_BITS: u32 = 0x80000;
    pub const UINT_BITS: u32 = 0x90000;
    pub const INT_BITS: u32 = 0xA0000;
    pub const HALF_BITS: u32 = 0xB0000;
    pub const FLOAT_BITS: u32 = 0xC0000;

    pub const UCHAR: u32 = UCHAR_BITS | 1;
    pub const UCHAR2: u32 = UCHAR_BITS | 2;
    pub const UCHAR3: u32 = UCHAR_BITS | 3;
    pub const UCHAR4: u32 = UCHAR_BITS | 4;

    pub const CHAR: u32 = CHAR_BITS | 1;
    pub const CHAR2: u32 = CHAR_BITS | 2;
    pub const CHAR3: u32 = CHAR_BITS | 3;
    pub const CHAR4: u32 = CHAR_BITS | 4;

    pub const UCHAR_NORMALIZED: u32 = UCHAR_NORMALIZED_BITS | 1;
    pub const UCHAR2_NORMALIZED: u32 = UCHAR_NORMALIZED_BITS | 2;
    pub const UCHAR3_NORMALIZED: u32 = UCHAR_NORMALIZED_BITS | 3;
    pub const UCHAR4_NORMALIZED: u32 = UCHAR_NORMALIZED_BITS | 4;

    pub const CHAR_NORMALIZED: u32 = CHAR_NORMALIZED_BITS | 1;
    pub const CHAR2_NORMALIZED: u32 = CHAR_NORMALIZED_BITS | 2;
    pub const CHAR3_NORMALIZED: u32 = CHAR_NORMALIZED_BITS | 3;
    pub const CHAR4_NORMALIZED: u32 = CHAR_NORMALIZED_BITS | 4;

    pub const USHORT: u32 = USHORT_BITS | 1;
    pub const USHORT2: u32 = USHORT_BITS | 2;
    pub const USHORT3: u32 = USHORT_BITS | 3;
    pub const USHORT4: u32 = USHORT_BITS | 4;

    pub const SHORT: u32 = SHORT_BITS | 1;
    pub const SHORT2: u32 = SHORT_BITS | 2;
    pub const SHORT3: u32 = SHORT_BITS | 3;
    pub const SHORT4: u32 = SHORT_BITS | 4;

    pub const USHORT_NORMALIZED: u32 = USHORT_NORMALIZED_BITS | 1;
    pub const USHORT2_NORMALIZED: u32 = USHORT_NORMALIZED_BITS | 2;
    pub const USHORT3_NORMALIZED: u32 = USHORT_NORMALIZED_BITS | 3;
    pub const USHORT4_NORMALIZED: u32 = USHORT_NORMALIZED_BITS | 4;

    pub const SHORT_NORMALIZED: u32 = SHORT_NORMALIZED_BITS | 1;
    pub const SHORT2_NORMALIZED: u32 = SHORT_NORMALIZED_BITS | 2;
    pub const SHORT3_NORMALIZED: u32 = SHORT_NORMALIZED_BITS | 3;
    pub const SHORT4_NORMALIZED: u32 = SHORT_NORMALIZED_BITS | 4;

    pub const UINT: u32 = UINT_BITS | 1;
    pub const UINT2: u32 = UINT_BITS | 2;
    pub const UINT3: u32 = UINT_BITS | 3;
    pub const UINT4: u32 = UINT_BITS | 4;

    pub const INT: u32 = INT_BITS | 1;
    pub const INT2: u32 = INT_BITS | 2;
    pub const INT3: u32 = INT_BITS | 3;
    pub const INT4: u32 = INT_BITS | 4;

    pub const HALF: u32 = HALF_BITS | 1;
    pub const HALF2: u32 = HALF_BITS | 2;
    pub const HALF3: u32 = HALF_BITS | 3;
    pub const HALF4: u32 = HALF_BITS | 4;

    pub const FLOAT: u32 = FLOAT_BITS | 1;
    pub const FLOAT2: u32 = FLOAT_BITS | 2;
    pub const FLOAT3: u32 = FLOAT_BITS | 3;
    pub const FLOAT4: u32 = FLOAT_BITS | 4;

    pub const INT1010102_NORMALIZED: u32 = INT_BITS | PACKED_BIT | 4;
    pub const UINT1010102_NORMALIZED: u32 = UINT_BITS | PACKED_BIT | 4;
}
