use anyhow::Context;
use bevy::prelude::*;

use crate::component;

#[allow(clippy::type_complexity)]
pub fn ui_update_system(
     mut q: QuerySet<(
        Query<&mut Text, With<component::UiSanity>>,
        Query<&component::Sanity, With<component::Player>>,
    )>,
) -> anyhow::Result<()> {
    let component::Sanity { max, current } = *q
        .q1()
        .single()
        .context("None or more than one Player Sanity was found")?;

    let mut sanity_text = q
        .q0_mut()
        .single_mut()
        .context("None or more than one UiSanity marker was found")?;

    sanity_text
        .sections
        .get_mut(0)
        .context("UiSanity Text should have one section")?
        .value = format!("Sanity: {}/{}", current, max);

    Ok(())
}
