use bevy::{prelude::*, utils::HashSet};

use crate::component;
use crate::event::{self, collision_events::SanityDrainEvent};

pub fn sanity_drain_system(
    time: Res<Time>,
    mut pairs: Local<HashSet<(Entity, Entity)>>,
    mut events: EventReader<event::collision_events::SanityDrainEvent>,
    mut q: QuerySet<(
        Query<&mut component::Sanity>,
        Query<&component::SanityDrain>,
    )>,
) {
    for event in events.iter() {
        match *event {
            SanityDrainEvent(
                offender,
                victim,
                event::collision_events::CollisionState::Started,
            ) => {
                pairs.insert((offender, victim));
            }
            SanityDrainEvent(
                offender,
                victim,
                event::collision_events::CollisionState::Stopped,
            ) => {
                pairs.remove(&(offender, victim));
            }
        }
    }

    for (offender, victim) in pairs.iter() {
        let drain = q
            .q1()
            .get_component::<component::SanityDrain>(*offender)
            .unwrap()
            .strength;
        let mut sanity = q
            .q0_mut()
            .get_component_mut::<component::Sanity>(*victim)
            .unwrap();
        sanity.current -= drain * time.delta_seconds();
    }
}
