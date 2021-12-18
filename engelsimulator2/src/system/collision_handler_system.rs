use bevy::prelude::*;
use heron::prelude as heron;

use crate::component;
use crate::event::collision_events;

pub fn collision_handler_system(
    mut collisions: EventReader<heron::CollisionEvent>,
    mut sanity_drains: EventWriter<collision_events::SanityDrainEvent>,
    q: QuerySet<(Query<&component::SanityDrain>, Query<&component::Sanity>)>,
) {
    for event in collisions.iter() {
        match event {
            heron::CollisionEvent::Started(data1, data2) => {
                let entity1 = data1.collision_shape_entity();
                let entity2 = data2.collision_shape_entity();
                println!("Collision: {:?} and {:?}", entity1, entity2);
                if let (Ok(_), Ok(_)) = (
                    q.q0().get_component::<component::SanityDrain>(entity1),
                    q.q1().get_component::<component::Sanity>(entity2),
                ) {
                    println!("Collision");
                    sanity_drains.send(collision_events::SanityDrainEvent(
                        entity1,
                        entity2,
                        collision_events::CollisionState::Started,
                    ));
                }
                if let (Ok(_), Ok(_)) = (
                    q.q0().get_component::<component::SanityDrain>(entity2),
                    q.q1().get_component::<component::Sanity>(entity1),
                ) {
                    println!("Collision");
                    sanity_drains.send(collision_events::SanityDrainEvent(
                        entity2,
                        entity1,
                        collision_events::CollisionState::Started,
                    ));
                }
            }
            heron::CollisionEvent::Stopped(data1, data2) => {
                let entity1 = data1.rigid_body_entity();
                let entity2 = data2.rigid_body_entity();
                if let (Ok(_), Ok(_)) = (
                    q.q0().get_component::<component::SanityDrain>(entity1),
                    q.q1().get_component::<component::Sanity>(entity2),
                ) {
                    sanity_drains.send(collision_events::SanityDrainEvent(
                        entity1,
                        entity2,
                        collision_events::CollisionState::Stopped,
                    ));
                }
                if let (Ok(_), Ok(_)) = (
                    q.q0().get_component::<component::SanityDrain>(entity2),
                    q.q1().get_component::<component::Sanity>(entity1),
                ) {
                    sanity_drains.send(collision_events::SanityDrainEvent(
                        entity2,
                        entity1,
                        collision_events::CollisionState::Stopped,
                    ));
                }
            }
        }
    }
}
