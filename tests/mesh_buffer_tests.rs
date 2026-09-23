use modelio::prelude::*;

#[test]
fn mesh_buffer_allocator_and_data_round_trip() {
    let allocator = MeshBufferDataAllocator::new().expect("allocator");
    let default_zone = allocator.new_default_zone(64).expect("default zone");
    assert_eq!(default_zone.capacity(), 64);
    assert!(default_zone.allocator().is_some());

    let empty_default_zone = MeshBufferZoneDefault::new().expect("empty default zone");
    assert_eq!(empty_default_zone.capacity(), 0);

    let generic_allocator = allocator.as_mesh_buffer_allocator();
    let buffer = generic_allocator
        .new_buffer_with_data(&[1, 2, 3, 4], MeshBufferType::Vertex)
        .expect("buffer");
    assert_eq!(
        buffer.info().expect("info").buffer_type_enum(),
        Some(MeshBufferType::Vertex)
    );
    assert!(buffer.allocator().is_some());

    let mapped = buffer.map().expect("map");
    assert_eq!(mapped.bytes(), vec![1, 2, 3, 4]);
    assert_eq!(mapped.write(1, &[9, 8]), 2);
    assert_eq!(buffer.bytes().expect("bytes"), vec![1, 9, 8, 4]);

    let data_buffer = MeshBufferData::new(8, MeshBufferType::Index).expect("data buffer");
    data_buffer
        .as_mesh_buffer()
        .fill_data(&[5, 6, 7, 8], 0)
        .expect("fill data");
    assert_eq!(&data_buffer.data()[..4], &[5, 6, 7, 8]);

    let zone = generic_allocator
        .new_zone_for_buffers(&[16, 8], &[MeshBufferType::Vertex, MeshBufferType::Index])
        .expect("zone");
    assert!(zone.as_default().is_some());
    let zone_buffer = generic_allocator
        .new_buffer_from_zone(Some(&zone), 4, MeshBufferType::Custom)
        .expect("zone buffer result")
        .expect("zone buffer");
    assert_eq!(
        zone_buffer.info().expect("zone info").buffer_type_enum(),
        Some(MeshBufferType::Custom)
    );
    assert!(zone_buffer.zone().is_some());
    assert!(zone_buffer.as_data_buffer().is_some());
}

#[test]
fn custom_mesh_buffer_allocator_callback_round_trip() {
    let allocator = MeshBufferAllocator::new(|event| {
        let fresh_allocator = MeshBufferDataAllocator::new()
            .expect("backing allocator")
            .as_mesh_buffer_allocator();
        match event {
            MeshBufferAllocatorEvent::NewZone { capacity } => {
                MeshBufferAllocatorResponse::Zone(Some(
                    fresh_allocator.new_zone(capacity).expect("new zone"),
                ))
            }
            MeshBufferAllocatorEvent::NewZoneForBuffers { sizes, types } => {
                MeshBufferAllocatorResponse::Zone(Some(
                    fresh_allocator
                        .new_zone_for_buffers(&sizes, &types)
                        .expect("new zone for buffers"),
                ))
            }
            MeshBufferAllocatorEvent::NewBuffer {
                length,
                buffer_type,
            } => MeshBufferAllocatorResponse::Buffer(Some(
                fresh_allocator
                    .new_buffer(length, buffer_type)
                    .expect("new buffer"),
            )),
            MeshBufferAllocatorEvent::NewBufferWithData { data, buffer_type } => {
                MeshBufferAllocatorResponse::Buffer(Some(
                    fresh_allocator
                        .new_buffer_with_data(&data, buffer_type)
                        .expect("new buffer with data"),
                ))
            }
            MeshBufferAllocatorEvent::NewBufferFromZone {
                zone,
                length,
                buffer_type,
            } => MeshBufferAllocatorResponse::Buffer(
                zone.and_then(|zone| {
                    zone.allocator()
                        .and_then(|allocator| {
                            allocator
                                .new_buffer_from_zone(Some(&zone), length, buffer_type)
                                .expect("new buffer from zone")
                        })
                }),
            ),
            MeshBufferAllocatorEvent::NewBufferFromZoneWithData {
                zone,
                data,
                buffer_type,
            } => MeshBufferAllocatorResponse::Buffer(
                zone.and_then(|zone| {
                    zone.allocator()
                        .and_then(|allocator| {
                            allocator
                                .new_buffer_from_zone_with_data(Some(&zone), &data, buffer_type)
                                .expect("new buffer from zone with data")
                        })
                }),
            ),
        }
    })
    .expect("custom allocator");

    let zone = allocator
        .new_zone_for_buffers(&[8], &[MeshBufferType::Vertex])
        .expect("custom zone");
    let buffer = allocator
        .new_buffer_from_zone_with_data(Some(&zone), &[1, 2, 3, 4], MeshBufferType::Vertex)
        .expect("buffer result")
        .expect("buffer");
    assert_eq!(buffer.bytes().expect("bytes"), vec![1, 2, 3, 4]);
    assert!(buffer.zone().is_some());
}

#[test]
fn fill_data_rejects_ranges_past_the_end_of_the_buffer() {
    let data_buffer = MeshBufferData::new(8, MeshBufferType::Vertex).expect("data buffer");
    let buffer = data_buffer.as_mesh_buffer();

    for (bytes, offset) in [
        (vec![1_u8; 16], 0),
        (vec![1_u8; 4], 6),
        (vec![1_u8; 1], 9),
        (vec![1_u8; 2], usize::MAX),
    ] {
        let error = buffer
            .fill_data(&bytes, offset)
            .expect_err("out-of-range fill must fail");
        assert_eq!(error.code(), -1);
    }
    assert_eq!(data_buffer.data(), vec![0_u8; 8]);

    buffer.fill_data(&[], 8).expect("empty fill at the end");
    buffer.fill_data(&[7, 7], 6).expect("fill the last two bytes");
    assert_eq!(data_buffer.data(), vec![0, 0, 0, 0, 0, 0, 7, 7]);
}
