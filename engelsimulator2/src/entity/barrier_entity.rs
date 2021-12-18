use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::RectangleOrigin};
use heron::prelude::*;

pub fn rect_barrier(commands: &mut Commands, (x, y, width, height): (f32, f32, f32, f32)) {
    let shape = shapes::Rectangle {
        width,
        height,
        origin: RectangleOrigin::Center,
    };
    let mut entity = commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::RED, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(2.0),
        },
        Transform::from_xyz(x + width / 2.0, y + height / 2.0, 10.0),
    ));
    entity.insert(RigidBody::Static)
          .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(width / 2.0, height / 2.0, 1000.0),
            border_radius: None,
        });
}
pub fn circle_barrier(commands: &mut Commands, (x, y, radius): (f32, f32, f32)) {
    let shape = shapes::Circle {
            radius: radius,
            center: Vec2::ZERO,
    };
    let mut entity = commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::RED, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(2.0),
        },
        Transform::from_xyz(x, y, 10.0),
    ));
    entity.insert(RigidBody::Static)
          .insert(CollisionShape::Sphere {
              radius
        });
}