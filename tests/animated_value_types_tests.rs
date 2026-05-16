use modelio::prelude::*;

fn assert_close(left: f32, right: f32) {
    assert!((left - right).abs() < 1e-6, "left={left} right={right}");
}

fn assert_close_slice(left: &[f32], right: &[f32]) {
    assert_eq!(left.len(), right.len());
    for (left, right) in left.iter().zip(right.iter()) {
        assert_close(*left, *right);
    }
}

#[test]
fn animated_value_types_store_and_return_samples() {
    let scalar = AnimatedScalar::new().expect("scalar");
    scalar.set_float(1.5, 0.5);
    assert_close(scalar.float_value(0.5), 1.5);

    let vector2 = AnimatedVector2::new().expect("vector2");
    vector2.set_float2([1.0, 2.0], 0.0);
    assert_close_slice(&vector2.float2_value(0.0), &[1.0, 2.0]);

    let vector3 = AnimatedVector3::new().expect("vector3");
    vector3.set_float3([1.0, 2.0, 3.0], 0.0);
    assert_close_slice(&vector3.float3_value(0.0), &[1.0, 2.0, 3.0]);

    let vector4 = AnimatedVector4::new().expect("vector4");
    vector4.set_float4([1.0, 2.0, 3.0, 4.0], 0.0);
    assert_close_slice(&vector4.float4_value(0.0), &[1.0, 2.0, 3.0, 4.0]);

    let quaternion = AnimatedQuaternion::new().expect("quaternion");
    quaternion.set_float_quaternion([0.0, 0.0, 0.0, 1.0], 0.0);
    assert_close_slice(
        &quaternion.float_quaternion_value(0.0),
        &[0.0, 0.0, 0.0, 1.0],
    );

    let matrix = AnimatedMatrix4x4::new().expect("matrix");
    matrix.set_float4x4(
        [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
        0.0,
    );
    assert_close(matrix.float4x4_value(0.0)[0], 1.0);

    let scalar_array = AnimatedScalarArray::new(2).expect("scalar array");
    scalar_array.set_float_array(&[1.0, 2.0], 0.0);
    assert_close_slice(
        &scalar_array.float_array_at_time(0.0).expect("values"),
        &[1.0, 2.0],
    );

    let vector3_array = AnimatedVector3Array::new(2).expect("vector3 array");
    vector3_array.set_float3_array(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]], 0.0);
    let vector3_values = vector3_array.float3_array_at_time(0.0).expect("values");
    assert_eq!(vector3_values.len(), 2);
    assert_close_slice(&vector3_values[0], &[1.0, 2.0, 3.0]);
    assert_close_slice(&vector3_values[1], &[4.0, 5.0, 6.0]);

    let quaternion_array = AnimatedQuaternionArray::new(1).expect("quaternion array");
    quaternion_array.set_float_quaternion_array(&[[0.0, 0.0, 0.0, 1.0]], 0.0);
    let quaternion_values = quaternion_array
        .float_quaternion_array_at_time(0.0)
        .expect("values");
    assert_eq!(quaternion_values.len(), 1);
    assert_close_slice(&quaternion_values[0], &[0.0, 0.0, 0.0, 1.0]);
}
