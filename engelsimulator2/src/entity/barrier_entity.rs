use std::f32::consts::PI;

use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::RectangleOrigin};

pub fn rect_barrier(commands: &mut Commands, (x, y, width, height, rotation): (f32, f32, f32, f32, f32)) {

    //let shape = shapes::Rectangle {
    //    width,
    //    height,
    //    origin: RectangleOrigin::Center,
    //};
    //let mut entity = commands.spawn_bundle(GeometryBuilder::build_as(
    //    &shape,
    //    ShapeColors::outlined(Color::RED, Color::BLACK),
    //    DrawMode::Outlined {
    //        fill_options: FillOptions::default(),
    //        outline_options: StrokeOptions::default().with_line_width(2.0),
    //    },
    //    Transform::from_xyz(x + width / 2.0, y + height / 2.0, 10.0),
    //));
    let transform = Transform::from_xyz(x + width / 2.0, y + height / 2.0, 10.0);

    let mut entity = commands.spawn();
    entity
        .insert(transform)
        .insert(GlobalTransform::default())
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(width / 2.0, height / 2.0, 1000.0),
            border_radius: None,
        });
    let entity_id = entity.id();

    let mut parent = commands.spawn();
    let mut parent_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    parent_transform.rotate(bevy::math::Quat::from_rotation_z(-rotation * PI / 180.0));
    parent.insert(parent_transform);
    parent.insert(GlobalTransform::default());
    parent.push_children(&[entity_id]);
}
pub fn circle_barrier(commands: &mut Commands, (x, y, radius): (f32, f32, f32)) {
    // let shape = shapes::Circle {
    //         radius: radius,
    //         center: Vec2::ZERO,
    // };
    // let mut entity = commands.spawn_bundle(GeometryBuilder::build_as(
    //     &shape,
    //     ShapeColors::outlined(Color::RED, Color::BLACK),
    //     DrawMode::Outlined {
    //         fill_options: FillOptions::default(),
    //         outline_options: StrokeOptions::default().with_line_width(2.0),
    //     },
    //     Transform::from_xyz(x, y, 10.0),
    // ));
    let mut entity = commands.spawn();
    entity
        .insert(Transform::from_xyz(x, y, 10.0))
        .insert(GlobalTransform::default())
        .insert(RigidBody::Static)
        .insert(CollisionShape::Sphere { radius });
}