use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
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
#[repr(i32)]
pub enum ProbePlacement {
    UniformGrid = 0,
    IrradianceDistribution = 1,
}

impl ProbePlacement {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::UniformGrid),
            1 => Some(Self::IrradianceDistribution),
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
    pub const fn as_raw(self) -> u32 {
        self as u32
    }

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
    pub const fn as_raw(self) -> u32 {
        self as u32
    }

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
#[repr(u32)]
pub enum MaterialFace {
    Front = 0,
    Back = 1,
    DoubleSided = 2,
}

impl MaterialFace {
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self as u32
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Front),
            1 => Some(Self::Back),
            2 => Some(Self::DoubleSided),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LightType {
    Unknown = 0,
    Ambient = 1,
    Directional = 2,
    Spot = 3,
    Point = 4,
    Linear = 5,
    DiscArea = 6,
    RectangularArea = 7,
    SuperElliptical = 8,
    Photometric = 9,
    Probe = 10,
    Environment = 11,
}

impl LightType {
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self as u32
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Unknown),
            1 => Some(Self::Ambient),
            2 => Some(Self::Directional),
            3 => Some(Self::Spot),
            4 => Some(Self::Point),
            5 => Some(Self::Linear),
            6 => Some(Self::DiscArea),
            7 => Some(Self::RectangularArea),
            8 => Some(Self::SuperElliptical),
            9 => Some(Self::Photometric),
            10 => Some(Self::Probe),
            11 => Some(Self::Environment),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CameraProjection {
    Perspective = 0,
    Orthographic = 1,
}

impl CameraProjection {
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self as u32
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Perspective),
            1 => Some(Self::Orthographic),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DataPrecision {
    Undefined = 0,
    Float = 1,
    Double = 2,
}

impl DataPrecision {
    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Undefined),
            1 => Some(Self::Float),
            2 => Some(Self::Double),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AnimatedValueInterpolation {
    Constant = 0,
    Linear = 1,
}

impl AnimatedValueInterpolation {
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        self as u32
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0 => Some(Self::Constant),
            1 => Some(Self::Linear),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum TransformOpRotationOrder {
    Xyz = 1,
    Xzy = 2,
    Yxz = 3,
    Yzx = 4,
    Zxy = 5,
    Zyx = 6,
}

impl TransformOpRotationOrder {
    #[must_use]
    pub const fn as_raw(self) -> u64 {
        self as u64
    }

    #[must_use]
    pub const fn from_raw(raw: u64) -> Option<Self> {
        match raw {
            1 => Some(Self::Xyz),
            2 => Some(Self::Xzy),
            3 => Some(Self::Yxz),
            4 => Some(Self::Yzx),
            5 => Some(Self::Zxy),
            6 => Some(Self::Zyx),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ObjectKind {
    Unknown = 0,
    Object = 1,
    Mesh = 2,
    Light = 3,
    PhysicallyPlausibleLight = 4,
    Camera = 5,
    VoxelArray = 6,
    Skeleton = 7,
    PackedJointAnimation = 8,
}

impl ObjectKind {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Unknown),
            1 => Some(Self::Object),
            2 => Some(Self::Mesh),
            3 => Some(Self::Light),
            4 => Some(Self::PhysicallyPlausibleLight),
            5 => Some(Self::Camera),
            6 => Some(Self::VoxelArray),
            7 => Some(Self::Skeleton),
            8 => Some(Self::PackedJointAnimation),
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
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

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
    pub mip_level_count: usize,
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

#[derive(Debug, Clone, Deserialize)]
pub struct MaterialInfo {
    pub name: String,
    pub count: usize,
    pub material_face: u32,
}

impl MaterialInfo {
    #[must_use]
    pub fn material_face_enum(&self) -> Option<MaterialFace> {
        MaterialFace::from_raw(self.material_face)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssetInfo {
    pub count: usize,
    pub frame_interval: f64,
    pub start_time: f64,
    pub end_time: f64,
    pub bounding_box: BoundingBox,
    pub up_axis: [f32; 3],
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ObjectInfo {
    pub kind: i32,
    pub name: String,
    pub path: String,
    pub hidden: bool,
    pub component_count: usize,
    pub child_count: usize,
    pub has_parent: bool,
    pub has_instance: bool,
    pub bounding_box: BoundingBox,
}

impl ObjectInfo {
    #[must_use]
    pub fn kind_enum(&self) -> Option<ObjectKind> {
        ObjectKind::from_raw(self.kind)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LightInfo {
    pub light_type: u32,
    pub color_space: String,
}

impl LightInfo {
    #[must_use]
    pub fn light_type_enum(&self) -> Option<LightType> {
        LightType::from_raw(self.light_type)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PhysicallyPlausibleLightInfo {
    pub light_type: u32,
    pub color_space: String,
    pub color: Option<[f32; 4]>,
    pub lumens: f32,
    pub inner_cone_angle: f32,
    pub outer_cone_angle: f32,
    pub attenuation_start_distance: f32,
    pub attenuation_end_distance: f32,
}

impl PhysicallyPlausibleLightInfo {
    #[must_use]
    pub fn light_type_enum(&self) -> Option<LightType> {
        LightType::from_raw(self.light_type)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CameraInfo {
    pub projection: u32,
    pub projection_matrix: [f32; 16],
    pub near_visibility_distance: f32,
    pub far_visibility_distance: f32,
    pub world_to_meters_conversion_scale: f32,
    pub barrel_distortion: f32,
    pub fisheye_distortion: f32,
    pub optical_vignetting: f32,
    pub chromatic_aberration: f32,
    pub focal_length: f32,
    pub focus_distance: f32,
    pub field_of_view: f32,
    pub f_stop: f32,
    pub aperture_blade_count: usize,
    pub maximum_circle_of_confusion: f32,
    pub shutter_open_interval: f64,
    pub sensor_vertical_aperture: f32,
    pub sensor_aspect: f32,
    pub sensor_enlargement: [f32; 2],
    pub sensor_shift: [f32; 2],
    pub flash: [f32; 3],
    pub exposure_compression: [f32; 2],
    pub exposure: [f32; 3],
}

impl CameraInfo {
    #[must_use]
    pub fn projection_enum(&self) -> Option<CameraProjection> {
        CameraProjection::from_raw(self.projection)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoxelIndexExtent {
    pub minimum_extent: [i32; 4],
    pub maximum_extent: [i32; 4],
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoxelArrayInfo {
    pub count: usize,
    pub bounding_box: BoundingBox,
    pub voxel_index_extent: VoxelIndexExtent,
    pub is_valid_signed_shell_field: bool,
    pub shell_field_interior_thickness: f32,
    pub shell_field_exterior_thickness: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnimatedValueInfo {
    pub is_animated: bool,
    pub precision: u32,
    pub time_sample_count: usize,
    pub minimum_time: f64,
    pub maximum_time: f64,
    pub interpolation: u32,
    pub key_times: Vec<f64>,
    pub element_count: Option<usize>,
}

impl AnimatedValueInfo {
    #[must_use]
    pub fn precision_enum(&self) -> Option<DataPrecision> {
        DataPrecision::from_raw(self.precision)
    }

    #[must_use]
    pub fn interpolation_enum(&self) -> Option<AnimatedValueInterpolation> {
        AnimatedValueInterpolation::from_raw(self.interpolation)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackedJointAnimationInfo {
    pub name: String,
    pub path: String,
    pub joint_paths: Vec<String>,
    pub joint_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnimationBindComponentInfo {
    pub has_skeleton: bool,
    pub has_joint_animation: bool,
    pub joint_paths: Option<Vec<String>>,
    pub geometry_bind_transform: [f32; 16],
}

#[derive(Debug, Clone, Deserialize)]
pub struct SkeletonInfo {
    pub name: String,
    pub path: String,
    pub joint_paths: Vec<String>,
    pub joint_count: usize,
    pub joint_bind_transform_count: usize,
    pub joint_rest_transform_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VertexAttributeDescriptorInfo {
    pub name: String,
    pub format: u32,
    pub offset: usize,
    pub buffer_index: usize,
    pub time: f64,
    pub initialization_value: [f32; 4],
}

#[derive(Debug, Clone, Deserialize)]
pub struct VertexDescriptorInfo {
    pub attribute_count: usize,
    pub attributes: Vec<VertexAttributeDescriptorInfo>,
    pub layout_strides: Vec<usize>,
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
