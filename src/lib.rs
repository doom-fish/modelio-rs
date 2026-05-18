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
mod asset_resolver;
mod camera;
mod error;
mod ffi;
mod handle;
mod light;
mod light_probe;
mod material;
mod mesh;
mod mesh_buffer;
mod object;
mod physically_plausible_light;
mod protocols;
mod sdk_constants;
mod skeleton;
mod submesh;
mod texture;
mod transform;
mod types;
mod util;
mod utility;
mod value_types;
mod vertex_attribute;
mod voxel_array;

pub use animated_value_types::{
    AnimatedMatrix4x4, AnimatedQuaternion, AnimatedQuaternionArray, AnimatedScalar,
    AnimatedScalarArray, AnimatedValue, AnimatedVector2, AnimatedVector3, AnimatedVector3Array,
    AnimatedVector4,
};
pub use animation::{AnimationBindComponent, PackedJointAnimation};
pub use asset::Asset;
pub use asset_resolver::{
    AssetResolver, BundleAssetResolver, PathAssetResolver, RelativeAssetResolver,
};
pub use camera::{Camera, StereoscopicCamera};
pub use error::{ModelIoError, Result};
pub use light::Light;
pub use light_probe::{LightProbe, LightProbeIrradianceDataSource};
pub use material::{
    Material, MaterialProperty, MaterialPropertyConnection, MaterialPropertyGraph,
    MaterialPropertyNode, TextureFilter, TextureSampler,
};
pub use mesh::{Mesh, MeshBuffer, VertexAttributeData};
pub use mesh_buffer::{
    MeshBufferAllocator, MeshBufferData, MeshBufferDataAllocator, MeshBufferMap, MeshBufferZone,
    MeshBufferZoneDefault,
};
pub use object::{Object, ObjectContainer};
pub use physically_plausible_light::{AreaLight, PhotometricLight, PhysicallyPlausibleLight};
pub use protocols::{Component, JointAnimation, Named, ObjectContainerComponent};
pub use sdk_constants::{ut_type, vertex_attribute_name};
pub use skeleton::Skeleton;
pub use submesh::{Submesh, SubmeshTopology};
pub use texture::Texture;
pub use transform::{
    Transform, TransformComponent, TransformMatrixOp, TransformOp, TransformOrientOp,
    TransformRotateOp, TransformRotateXOp, TransformRotateYOp, TransformRotateZOp,
    TransformScaleOp, TransformStack, TransformTranslateOp,
};
pub use types::{
    vertex_format, AnimatedValueInfo, AnimatedValueInterpolation, AreaLightInfo, AssetInfo,
    BoundingBox, CameraInfo, CameraProjection, DataPrecision, GeometryType, IndexBitDepth,
    LightInfo, LightType, MaterialFace, MaterialInfo, MaterialMipMapFilterMode,
    MaterialPropertyInfo, MaterialPropertyType, MaterialSemantic, MaterialTextureFilterMode,
    MaterialTextureWrapMode, Matrix4x4ArrayInfo, MeshBufferInfo, MeshBufferType, ObjectInfo,
    ObjectKind, PackedJointAnimationInfo, PhotometricLightInfo, PhysicallyPlausibleLightInfo,
    ProbePlacement, SkeletonInfo, StereoscopicCameraInfo, TextureChannelEncoding,
    TextureFilterInfo, TextureInfo, TextureSamplerInfo, TransformOpRotationOrder,
    VertexAttributeDescriptorInfo, VertexAttributeInfo, VertexDescriptorInfo, VoxelArrayInfo,
    VoxelIndexExtent,
};
pub use utility::Utility;
pub use value_types::Matrix4x4Array;
pub use vertex_attribute::{VertexAttribute, VertexBufferLayout, VertexDescriptor};
pub use voxel_array::VoxelArray;

/// Re-exports the primary Model I/O wrappers for convenient imports.
pub mod prelude {
    pub use crate::{
        ut_type, vertex_attribute_name, vertex_format, AnimatedMatrix4x4, AnimatedQuaternion,
        AnimatedQuaternionArray, AnimatedScalar, AnimatedScalarArray, AnimatedValue,
        AnimatedValueInfo, AnimatedValueInterpolation, AnimatedVector2, AnimatedVector3,
        AnimatedVector3Array, AnimatedVector4, AnimationBindComponent, AreaLight, AreaLightInfo,
        Asset, AssetInfo, AssetResolver, BoundingBox, BundleAssetResolver, Camera, CameraInfo,
        CameraProjection, Component, DataPrecision, GeometryType, IndexBitDepth, JointAnimation,
        Light, LightInfo, LightProbe, LightProbeIrradianceDataSource, LightType, Material,
        MaterialFace, MaterialInfo, MaterialMipMapFilterMode, MaterialProperty,
        MaterialPropertyConnection, MaterialPropertyGraph, MaterialPropertyInfo,
        MaterialPropertyNode, MaterialPropertyType, MaterialSemantic, MaterialTextureFilterMode,
        MaterialTextureWrapMode, Matrix4x4Array, Matrix4x4ArrayInfo, Mesh, MeshBuffer,
        MeshBufferAllocator, MeshBufferData, MeshBufferDataAllocator, MeshBufferInfo,
        MeshBufferMap, MeshBufferType, MeshBufferZone, MeshBufferZoneDefault, ModelIoError, Named,
        Object, ObjectContainer, ObjectContainerComponent, ObjectInfo, ObjectKind,
        PackedJointAnimation, PackedJointAnimationInfo, PathAssetResolver, PhotometricLight,
        PhotometricLightInfo, PhysicallyPlausibleLight, PhysicallyPlausibleLightInfo,
        ProbePlacement, RelativeAssetResolver, Result, Skeleton, SkeletonInfo, StereoscopicCamera,
        StereoscopicCameraInfo, Submesh, SubmeshTopology, Texture, TextureChannelEncoding,
        TextureFilter, TextureFilterInfo, TextureInfo, TextureSampler, TextureSamplerInfo,
        Transform, TransformComponent, TransformMatrixOp, TransformOp, TransformOpRotationOrder,
        TransformOrientOp, TransformRotateOp, TransformRotateXOp, TransformRotateYOp,
        TransformRotateZOp, TransformScaleOp, TransformStack, TransformTranslateOp, Utility,
        VertexAttribute, VertexAttributeData, VertexAttributeDescriptorInfo, VertexAttributeInfo,
        VertexBufferLayout, VertexDescriptor, VertexDescriptorInfo, VoxelArray, VoxelArrayInfo,
        VoxelIndexExtent,
    };
}
