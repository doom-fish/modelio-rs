#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate,
    clippy::return_self_not_must_use,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::too_many_lines
)]

mod animated_value_types;
mod animation;
mod asset;
mod camera;
mod error;
mod ffi;
mod handle;
mod light;
mod material;
mod mesh;
mod object;
mod physically_plausible_light;
mod skeleton;
mod submesh;
mod texture;
mod types;
mod util;
mod vertex_attribute;
mod voxel_array;

pub use animated_value_types::{
    AnimatedMatrix4x4, AnimatedQuaternion, AnimatedQuaternionArray, AnimatedScalar,
    AnimatedScalarArray, AnimatedVector2, AnimatedVector3, AnimatedVector3Array, AnimatedVector4,
};
pub use animation::{AnimationBindComponent, PackedJointAnimation};
pub use asset::Asset;
pub use camera::Camera;
pub use error::{ModelIoError, Result};
pub use light::Light;
pub use material::{Material, MaterialProperty};
pub use mesh::{Mesh, MeshBuffer, VertexAttributeData};
pub use object::Object;
pub use physically_plausible_light::PhysicallyPlausibleLight;
pub use skeleton::Skeleton;
pub use submesh::Submesh;
pub use texture::Texture;
pub use types::{
    vertex_format, AnimatedValueInfo, AnimatedValueInterpolation, AssetInfo, BoundingBox,
    CameraInfo, CameraProjection, DataPrecision, GeometryType, IndexBitDepth, LightInfo, LightType,
    MaterialFace, MaterialInfo, MaterialPropertyInfo, MaterialPropertyType, MaterialSemantic,
    MeshBufferInfo, MeshBufferType, ObjectInfo, ObjectKind, PackedJointAnimationInfo,
    PhysicallyPlausibleLightInfo, SkeletonInfo, TextureChannelEncoding, TextureInfo,
    VertexAttributeDescriptorInfo, VertexAttributeInfo, VertexDescriptorInfo, VoxelArrayInfo,
    VoxelIndexExtent,
};
pub use vertex_attribute::{VertexAttribute, VertexDescriptor};
pub use voxel_array::VoxelArray;

pub mod prelude {
    pub use crate::{
        vertex_format, AnimatedMatrix4x4, AnimatedQuaternion, AnimatedQuaternionArray,
        AnimatedScalar, AnimatedScalarArray, AnimatedValueInfo, AnimatedValueInterpolation,
        AnimatedVector2, AnimatedVector3, AnimatedVector3Array, AnimatedVector4,
        AnimationBindComponent, Asset, AssetInfo, BoundingBox, Camera, CameraInfo,
        CameraProjection, DataPrecision, GeometryType, IndexBitDepth, Light, LightInfo, LightType,
        Material, MaterialFace, MaterialInfo, MaterialProperty, MaterialPropertyInfo,
        MaterialPropertyType, MaterialSemantic, Mesh, MeshBuffer, MeshBufferInfo, MeshBufferType,
        ModelIoError, Object, ObjectInfo, ObjectKind, PackedJointAnimation,
        PackedJointAnimationInfo, PhysicallyPlausibleLight, PhysicallyPlausibleLightInfo, Result,
        Skeleton, SkeletonInfo, Submesh, Texture, TextureChannelEncoding, TextureInfo,
        VertexAttribute, VertexAttributeData, VertexAttributeDescriptorInfo, VertexAttributeInfo,
        VertexDescriptor, VertexDescriptorInfo, VoxelArray, VoxelArrayInfo, VoxelIndexExtent,
    };
}
