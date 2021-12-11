use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;

use crate::component;

pub struct PlayerEntityGenerator {}

impl PlayerEntityGenerator {
    pub fn new() -> Self {
        PlayerEntityGenerator {}
    }

    pub fn build(self, commands: &mut Commands) {
        let shape = shapes::Circle {
            radius: 20.0,
            center: Vec2::ZERO,
        };
        commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::TEAL, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(2.0),
            },
            Transform::default(),
        ))
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 10.0 })
        .insert(Velocity::from_linear(Vec3::X * 30.0))
        .insert(component::Player);
    }
}
