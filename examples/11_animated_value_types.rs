use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let scalar = AnimatedScalar::new()?;
    scalar.set_float(3.5, 1.0);

    let vector3 = AnimatedVector3::new()?;
    vector3.set_float3([1.0, 2.0, 3.0], 0.5);

    let quaternion = AnimatedQuaternion::new()?;
    quaternion.set_float_quaternion([0.0, 0.0, 0.0, 1.0], 0.0);

    let matrix = AnimatedMatrix4x4::new()?;
    matrix.set_float4x4(
        [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
        0.0,
    );

    let scalar_array = AnimatedScalarArray::new(2)?;
    scalar_array.set_float_array(&[1.0, 2.0], 0.0);

    println!("scalar={}", scalar.float_value(1.0));
    println!("vector3={:?}", vector3.float3_value(0.5));
    println!("quaternion={:?}", quaternion.float_quaternion_value(0.0));
    println!("matrix={:?}", matrix.float4x4_value(0.0));
    println!("scalar_array={:?}", scalar_array.float_array_at_time(0.0)?);
    println!("✅ modelio animated value types OK");
    Ok(())
}
