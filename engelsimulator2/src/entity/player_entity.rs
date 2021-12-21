use bevy::prelude::*;
use heron::prelude::*;

use crate::component;
use crate::resource;

pub struct NoTexture;

pub struct PlayerEntityGenerator<TEXTURE> {
    texture: TEXTURE,
    position: (f32, f32),
    sanity_max: f32,
    sanity_current: f32,
}

impl PlayerEntityGenerator<NoTexture> {
    pub fn new() -> Self {
        PlayerEntityGenerator {
            texture: NoTexture,
            position: (0.0, 0.0),
            sanity_max: 120.0,
            sanity_current: 120.0,
        }
    }
}

impl<TEXTURE> PlayerEntityGenerator<TEXTURE> {
    pub fn with_sprites(
        self,
        sprites: &resource::Sprites,
    ) -> PlayerEntityGenerator<Handle<ColorMaterial>> {
        PlayerEntityGenerator {
            texture: sprites.player.clone(),
            position: self.position,
            sanity_current: self.sanity_current,
            sanity_max: self.sanity_max,
        }
    }

    pub fn with_position(mut self, position: (f32, f32)) -> Self {
        self.position = position;
        self
    }

    pub fn with_sanity_current(mut self, sanity: f32) -> Self {
        self.sanity_current = sanity;
        self
    }
}

impl PlayerEntityGenerator<Handle<ColorMaterial>> {
    pub fn build(self, commands: &mut Commands) {
        let mut entity = commands.spawn_bundle(SpriteBundle {
            material: self.texture,
            sprite: Sprite::new(Vec2::new(64.0, 64.0)),
            transform: Transform::from_xyz(self.position.0, self.position.1, 100.0),
            ..Default::default()
        });

        entity
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: 10.0 })
            .insert(Velocity::from_linear(Vec3::X * 30.0))
            .insert(RotationConstraints::lock())
            .insert(component::Player)
            .insert(component::Sanity::new(self.sanity_current, self.sanity_max));
        println!("Player id: {:?}", entity.id());
    }
}
