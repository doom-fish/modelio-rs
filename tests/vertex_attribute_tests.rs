use modelio::prelude::*;

#[test]
fn vertex_attribute_and_descriptor_surfaces_are_available() {
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)
        .expect("box mesh");
    let descriptor = mesh.vertex_descriptor().expect("vertex descriptor");
    let descriptor_info = descriptor.info().expect("descriptor info");
    assert!(descriptor_info.attribute_count > 0);

    let attribute =
        VertexAttribute::new("custom", vertex_format::FLOAT3, 0, 0).expect("custom attribute");
    attribute.set_initialization_value([0.0, 0.0, 0.0, 1.0]);
    assert_eq!(attribute.info().expect("attribute info").buffer_index, 0);

    let layout = VertexBufferLayout::new(32).expect("vertex buffer layout");
    layout.set_stride(48);
    assert_eq!(layout.stride(), 48);
    assert!(!descriptor.layouts().is_empty());
    assert!(vertex_attribute_name::position().is_some());
}

#[test]
fn vertex_formats_are_typed_and_sized() {
    assert_eq!(vertex_format::FLOAT3.byte_size(), 12);
    assert_eq!(vertex_format::FLOAT3.component_count(), 3);
    assert_eq!(vertex_format::HALF2.byte_size(), 4);
    assert_eq!(vertex_format::UCHAR4_NORMALIZED.byte_size(), 4);
    assert_eq!(vertex_format::SHORT3.byte_size(), 6);
    assert_eq!(vertex_format::INT1010102_NORMALIZED.byte_size(), 4);
    assert!(vertex_format::UINT1010102_NORMALIZED.is_packed());
    assert!(!vertex_format::FLOAT4.is_packed());
    assert_eq!(vertex_format::INVALID.byte_size(), 0);
    assert_eq!(
        VertexFormat::from_raw(vertex_format::FLOAT3.as_raw()),
        Some(vertex_format::FLOAT3)
    );
    assert_eq!(VertexFormat::from_raw(0), Some(vertex_format::INVALID));
    for raw in [
        0xDEAD,
        vertex_format::FLOAT_BITS,
        vertex_format::FLOAT_BITS | 5,
        0xD0001,
        vertex_format::FLOAT_BITS | vertex_format::PACKED_BIT | 4,
    ] {
        assert_eq!(VertexFormat::from_raw(raw), None, "raw {raw:#x}");
    }

    let attribute =
        VertexAttribute::new("typed", vertex_format::HALF4, 8, 1).expect("typed attribute");
    let info = attribute.info().expect("attribute info");
    assert_eq!(info.format_enum(), Some(vertex_format::HALF4));
    attribute.set_format(vertex_format::UCHAR4);
    assert_eq!(
        attribute.info().expect("attribute info").format_enum(),
        Some(vertex_format::UCHAR4)
    );
}

fn position_only_descriptor(offset: usize, buffer_index: usize, stride: usize) -> VertexDescriptor {
    let descriptor = VertexDescriptor::new().expect("vertex descriptor");
    let position = VertexAttribute::new(
        &vertex_attribute_name::position().expect("position name"),
        vertex_format::FLOAT3,
        offset,
        buffer_index,
    )
    .expect("position attribute");
    descriptor.add_or_replace_attribute(&position);
    descriptor
        .layout_at(0)
        .expect("layout 0")
        .set_stride(stride);
    descriptor
}

#[test]
fn applying_a_valid_descriptor_relays_out_the_mesh() {
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)
        .expect("box mesh");
    let vertex_count = mesh.vertex_count();

    mesh.set_vertex_descriptor(&position_only_descriptor(0, 0, 12))
        .expect("apply descriptor");

    let info = mesh
        .vertex_descriptor()
        .expect("vertex descriptor")
        .info()
        .expect("descriptor info");
    let active = info
        .attributes
        .iter()
        .filter(|attribute| attribute.format_enum() != Some(vertex_format::INVALID))
        .collect::<Vec<_>>();
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].format_enum(), Some(vertex_format::FLOAT3));
    assert_eq!(info.layout_strides[0], 12);
    assert_eq!(
        mesh.vertex_buffer(0)
            .expect("vertex buffer")
            .info()
            .expect("info")
            .length,
        12 * vertex_count
    );
}

#[test]
fn applying_an_inconsistent_descriptor_is_rejected() {
    let mesh = Mesh::new_box([1.0, 1.0, 1.0], [1, 1, 1], false, GeometryType::Triangles)
        .expect("box mesh");
    let original_length = mesh
        .vertex_buffer(0)
        .expect("vertex buffer")
        .info()
        .expect("info")
        .length;

    for (offset, buffer_index, stride) in [
        (4096, 0, 12),
        (0, 0, 4),
        (4, 0, 12),
        (0, 20, 12),
        (0, 40, 12),
    ] {
        let error = mesh
            .set_vertex_descriptor(&position_only_descriptor(offset, buffer_index, stride))
            .expect_err("inconsistent descriptor must be rejected");
        assert_eq!(
            error.code(),
            -1,
            "offset {offset} buffer {buffer_index} stride {stride}"
        );
    }
    assert_eq!(
        mesh.vertex_buffer(0)
            .expect("vertex buffer")
            .info()
            .expect("info")
            .length,
        original_length
    );
}
