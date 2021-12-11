
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;

use crate::component;

pub struct NoRadius;

pub struct SanityDrainGenerator<RADIUS> {
    radius: RADIUS,
    x: f32,
    y: f32,
}

impl SanityDrainGenerator<NoRadius> {
    pub fn new() -> Self {
        SanityDrainGenerator {
            radius: NoRadius,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl<RADIUS> SanityDrainGenerator<RADIUS> {
    pub fn with_radius(self, radius: impl Into<f32>) -> SanityDrainGenerator<f32> {
        SanityDrainGenerator {
            radius: radius.into(),
            x: self.x,
            y: self.y,
        }
    }
    pub fn with_position(self, x: impl Into<f32>, y: impl Into<f32>) -> Self {
        SanityDrainGenerator {
            radius: self.radius,
            x: x.into(), 
            y: y.into(),
        }
    }
}

impl SanityDrainGenerator<f32> {
    pub fn build(self, commands: &mut Commands) {
        let shape = shapes::Circle {
            radius: self.radius,
            center: Vec2::ZERO,
        };
        let mut entity = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(Color::RED, Color::BLACK),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(2.0),
                },
                Transform::from_xyz(self.x, self.y, 10.0),
            ));
        entity.insert(RigidBody::Sensor)
            .insert(CollisionShape::Sphere { radius: self.radius })
            .insert(Velocity::from_linear(Vec3::X * 30.0))
            .insert(RotationConstraints::lock())
            .insert(component::SanityDrain::with_strength(10.0));
        println!("Sanity Drain id: {:?}", entity.id());
    }
}
