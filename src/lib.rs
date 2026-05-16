#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API Documentation
//!
//! Safe Rust bindings for Apple's [ModelIO](https://developer.apple.com/documentation/modelio)
//! framework — 3D assets, meshes, materials, textures, and primitive generators on macOS.

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
    clippy::struct_excessive_bools
)]

mod asset;
mod error;
mod ffi;
mod handle;
mod material;
mod mesh;
mod texture;
mod types;
mod util;

pub use asset::Asset;
pub use error::{ModelIoError, Result};
pub use material::{Material, MaterialProperty};
pub use mesh::{Mesh, MeshBuffer, Submesh, VertexAttributeData};
pub use texture::Texture;
pub use types::{
    vertex_format, BoundingBox, GeometryType, IndexBitDepth, MaterialPropertyInfo,
    MaterialPropertyType, MaterialSemantic, MeshBufferInfo, MeshBufferType, TextureChannelEncoding,
    TextureInfo, VertexAttributeInfo,
};

pub mod prelude {
    pub use crate::{
        vertex_format, Asset, BoundingBox, GeometryType, IndexBitDepth, Material, MaterialProperty,
        MaterialPropertyInfo, MaterialPropertyType, MaterialSemantic, Mesh, MeshBuffer,
        MeshBufferInfo, MeshBufferType, ModelIoError, Result, Submesh, Texture,
        TextureChannelEncoding, TextureInfo, VertexAttributeData, VertexAttributeInfo,
    };
}
