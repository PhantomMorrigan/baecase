#![allow(clippy::type_complexity)]

use std::f32::consts::TAU;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    image::ImageSamplerDescriptor,
    prelude::*,
};
use bevy_bae::prelude::*;
use rand::Rng;

use crate::{
    berries::NewBerry,
    ghost::{TargetedBerry, ghost_plugin},
};

mod berries;
mod ghost;
mod leaderbord;

const RANGE: f32 = 450.0;

pub fn sample_arena(rng: &mut impl Rng) -> Vec2 {
    let r: f32 = rng.random::<f32>() * TAU;
    Vec2::new(r.sin(), r.cos()) * (rng.random::<f32>() * RANGE)
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor::nearest(),
            }),
            BaePlugin::default(),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            ghost_plugin,
        ))
        .add_systems(Startup, leaderbord::setup.after(ghost::setup))
        .add_message::<NewBerry>()
        .add_systems(
            FixedUpdate,
            (berries::spawn_berries, leaderbord::update_leaderboard),
        )
        .add_observer(
            |trigger: On<Add, TargetedBerry>, mut sprite: Query<&mut Sprite>| {
                sprite.get_mut(trigger.entity).unwrap().color = Color::linear_rgb(2.0, 1.0, 0.8);
            },
        )
        .add_observer(
            |trigger: On<Remove, TargetedBerry>, mut sprite: Query<&mut Sprite>| {
                sprite.get_mut(trigger.entity).unwrap().color = Color::WHITE;
            },
        )
        .run();
}
