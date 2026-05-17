use modelio::prelude::*;

fn main() -> modelio::Result<()> {
    let object = Object::new()?;
    let transform = Transform::new()?;
    transform.set_translation([1.0, 2.0, 3.0]);
    object.set_transform_component(Some(&transform.as_transform_component()));

    let stack = TransformStack::new()?;
    let translate_op = stack.add_translate_op("translate", false)?;
    translate_op
        .animated_value()
        .expect("translate animated value")
        .set_float3([2.0, 0.0, 0.0], 0.0);
    let rotate_op = stack.add_rotate_op("rotate", TransformOpRotationOrder::Xyz, false)?;
    rotate_op
        .animated_value()
        .expect("rotate animated value")
        .set_float3([0.0, 0.5, 0.0], 0.0);

    println!(
        "object global matrix={:?}",
        TransformComponent::global_transform_with_object(&object, 0.0)
    );
    println!("stack op count={}", stack.count());
    println!("stack matrix={:?}", stack.float4x4_at_time(0.0));
    Ok(())
}
