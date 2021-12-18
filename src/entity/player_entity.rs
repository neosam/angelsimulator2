use bevy::prelude::*;
use heron::prelude::*;

use crate::component;
use crate::resource;

pub struct NoTexture;

pub struct PlayerEntityGenerator<TEXTURE> {
    texture: TEXTURE,
}

impl PlayerEntityGenerator<NoTexture> {
    pub fn new() -> Self {
        PlayerEntityGenerator {
            texture: NoTexture,
        }
    }
}

impl<TEXTURE> PlayerEntityGenerator<TEXTURE> {
    pub fn with_sprites(self, sprites: &resource::Sprites) -> PlayerEntityGenerator<Handle<ColorMaterial>> {
        PlayerEntityGenerator { texture: sprites.player.clone() }
    }
}

impl PlayerEntityGenerator<Handle<ColorMaterial>> {
    pub fn build(self, commands: &mut Commands) {
        let mut entity = commands.spawn_bundle(SpriteBundle {
            material: self.texture,
            sprite: Sprite::new(Vec2::new(64.0, 64.0)),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..Default::default()
        });

        entity
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: 10.0 })
            .insert(Velocity::from_linear(Vec3::X * 30.0))
            .insert(RotationConstraints::lock())
            .insert(component::Player)
            .insert(component::Sanity::new_full(120.0));
        println!("Player id: {:?}", entity.id());
    }
}
