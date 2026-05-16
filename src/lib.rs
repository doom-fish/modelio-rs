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
mod skeleton;
mod submesh;
mod texture;
mod transform;
mod types;
mod util;
mod vertex_attribute;
mod voxel_array;

pub use animated_value_types::{
    AnimatedMatrix4x4, AnimatedQuaternion, AnimatedQuaternionArray, AnimatedScalar,
    AnimatedScalarArray, AnimatedValue, AnimatedVector2, AnimatedVector3, AnimatedVector3Array,
    AnimatedVector4,
};
pub use animation::{AnimationBindComponent, PackedJointAnimation};
pub use asset::Asset;
pub use asset_resolver::{AssetResolver, BundleAssetResolver, PathAssetResolver, RelativeAssetResolver};
pub use camera::Camera;
pub use error::{ModelIoError, Result};
pub use light::Light;
pub use light_probe::{LightProbe, LightProbeIrradianceDataSource};
pub use material::{Material, MaterialProperty};
pub use mesh::{Mesh, MeshBuffer, VertexAttributeData};
pub use mesh_buffer::{
    MeshBufferAllocator, MeshBufferData, MeshBufferDataAllocator, MeshBufferMap, MeshBufferZone,
    MeshBufferZoneDefault,
};
pub use object::Object;
pub use physically_plausible_light::PhysicallyPlausibleLight;
pub use skeleton::Skeleton;
pub use submesh::Submesh;
pub use texture::Texture;
pub use transform::{
    Transform, TransformComponent, TransformMatrixOp, TransformOp, TransformOrientOp,
    TransformRotateOp, TransformRotateXOp, TransformRotateYOp, TransformRotateZOp, TransformScaleOp,
    TransformStack, TransformTranslateOp,
};
pub use types::{
    vertex_format, AnimatedValueInfo, AnimatedValueInterpolation, AssetInfo, BoundingBox,
    CameraInfo, CameraProjection, DataPrecision, GeometryType, IndexBitDepth, LightInfo, LightType,
    MaterialFace, MaterialInfo, MaterialPropertyInfo, MaterialPropertyType, MaterialSemantic,
    MeshBufferInfo, MeshBufferType, ObjectInfo, ObjectKind, PackedJointAnimationInfo,
    PhysicallyPlausibleLightInfo, ProbePlacement, SkeletonInfo, TextureChannelEncoding, TextureInfo,
    TransformOpRotationOrder, VertexAttributeDescriptorInfo, VertexAttributeInfo,
    VertexDescriptorInfo, VoxelArrayInfo, VoxelIndexExtent,
};
pub use vertex_attribute::{VertexAttribute, VertexDescriptor};
pub use voxel_array::VoxelArray;

pub mod prelude {
    pub use crate::{
        vertex_format, AnimatedMatrix4x4, AnimatedQuaternion, AnimatedQuaternionArray,
        AnimatedScalar, AnimatedScalarArray, AnimatedValue, AnimatedValueInfo,
        AnimatedValueInterpolation, AnimatedVector2, AnimatedVector3, AnimatedVector3Array,
        AnimatedVector4, AnimationBindComponent, Asset, AssetInfo, AssetResolver, BoundingBox,
        BundleAssetResolver, Camera, CameraInfo, CameraProjection, DataPrecision, GeometryType,
        IndexBitDepth, Light, LightInfo, LightProbe, LightProbeIrradianceDataSource, LightType,
        Material, MaterialFace, MaterialInfo, MaterialProperty, MaterialPropertyInfo,
        MaterialPropertyType, MaterialSemantic, Mesh, MeshBuffer, MeshBufferAllocator,
        MeshBufferData, MeshBufferDataAllocator, MeshBufferInfo, MeshBufferMap, MeshBufferType,
        MeshBufferZone, MeshBufferZoneDefault, ModelIoError, Object, ObjectInfo, ObjectKind,
        PackedJointAnimation, PackedJointAnimationInfo, PathAssetResolver,
        PhysicallyPlausibleLight, PhysicallyPlausibleLightInfo, ProbePlacement, RelativeAssetResolver,
        Result, Skeleton, SkeletonInfo, Submesh, Texture, TextureChannelEncoding, TextureInfo,
        Transform, TransformComponent, TransformMatrixOp, TransformOp, TransformOpRotationOrder,
        TransformOrientOp, TransformRotateOp, TransformRotateXOp, TransformRotateYOp,
        TransformRotateZOp, TransformScaleOp, TransformStack, TransformTranslateOp,
        VertexAttribute, VertexAttributeData, VertexAttributeDescriptorInfo, VertexAttributeInfo,
        VertexDescriptor, VertexDescriptorInfo, VoxelArray, VoxelArrayInfo, VoxelIndexExtent,
    };
}
