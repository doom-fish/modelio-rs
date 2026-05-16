use modelio::prelude::*;

#[test]
fn transform_and_transform_stack_round_trip() {
    let object = Object::new().expect("object");
    let transform = Transform::new().expect("transform");
    transform.set_translation([1.0, 2.0, 3.0]);
    transform.set_scale([2.0, 2.0, 2.0]);
    transform.set_rotation_for_time([0.0, 0.25, 0.0], 1.0);
    transform.set_translation_for_time([4.0, 5.0, 6.0], 1.0);

    assert!(transform.translation().iter().all(|value| value.is_finite()));
    assert!(transform
        .scale()
        .iter()
        .zip([2.0, 2.0, 2.0])
        .all(|(left, right)| (*left - right).abs() < f32::EPSILON));
    assert!(transform
        .translation_at_time(1.0)
        .iter()
        .zip([4.0, 5.0, 6.0])
        .all(|(left, right)| (*left - right).abs() < f32::EPSILON));
    assert!(transform
        .rotation_matrix_at_time(1.0)
        .iter()
        .all(|value| value.is_finite()));

    object.set_transform_component(Some(&transform.as_transform_component()));
    let component = object.transform_component().expect("transform component");
    assert!(component.as_transform().is_some());
    assert!(Transform::from_component(&component)
        .expect("copy")
        .translation()
        .iter()
        .all(|value| value.is_finite()));
    assert!(TransformComponent::global_transform_with_object(&object, 1.0)
        .iter()
        .all(|value| value.is_finite()));

    let stack = TransformStack::new().expect("stack");
    let translate_op = stack.add_translate_op("translate", false).expect("translate op");
    translate_op
        .animated_value()
        .expect("translate animated value")
        .set_float3([2.0, 0.0, 0.0], 0.0);

    let rotate_x = stack.add_rotate_x_op("rotate_x", false).expect("rotate x");
    rotate_x
        .animated_value()
        .expect("rotate x animated value")
        .set_float(0.1, 0.0);

    let rotate_y = stack.add_rotate_y_op("rotate_y", false).expect("rotate y");
    rotate_y
        .animated_value()
        .expect("rotate y animated value")
        .set_float(0.2, 0.0);

    let rotate_z = stack.add_rotate_z_op("rotate_z", false).expect("rotate z");
    rotate_z
        .animated_value()
        .expect("rotate z animated value")
        .set_float(0.3, 0.0);

    let rotate = stack
        .add_rotate_op("rotate", TransformOpRotationOrder::Xyz, true)
        .expect("rotate op");
    rotate
        .animated_value()
        .expect("rotate animated value")
        .set_float3([0.0, 0.5, 0.0], 0.0);

    let scale = stack.add_scale_op("scale", false).expect("scale op");
    scale
        .animated_value()
        .expect("scale animated value")
        .set_float3([1.0, 1.0, 1.0], 0.0);

    let matrix = stack.add_matrix_op("matrix", false).expect("matrix op");
    matrix
        .animated_value()
        .expect("matrix animated value")
        .set_float4x4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ], 0.0);

    let orient = stack.add_orient_op("orient", false).expect("orient op");
    orient
        .animated_value()
        .expect("orient animated value")
        .set_float_quaternion([0.0, 0.0, 0.0, 1.0], 0.0);

    assert_eq!(stack.count(), 8);
    let ops = stack.transform_ops().expect("transform ops");
    assert_eq!(ops.len(), 8);
    assert_eq!(ops[0].name().as_deref(), Some("translate"));
    assert!(ops[4].is_inverse());
    assert!(stack.float4x4_at_time(0.0).iter().all(|value| value.is_finite()));

    let animated_value = stack
        .animated_value_named("translate")
        .expect("animated value lookup")
        .expect("animated value");
    let animated_info = animated_value.info().expect("animated value info");
    assert!(animated_info.minimum_time <= animated_info.maximum_time);
    assert!(!animated_info.key_times.is_empty());

    object.set_transform_component(Some(&stack.as_transform_component()));
    let stack_component = object.transform_component().expect("stack component");
    assert!(stack_component.as_transform_stack().is_some());
    assert!(stack_component
        .local_transform_at_time(0.0)
        .iter()
        .all(|value| value.is_finite()));
}
