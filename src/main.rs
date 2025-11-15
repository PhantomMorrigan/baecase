#![allow(clippy::type_complexity)]

use std::f32::consts::TAU;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    image::ImageSamplerDescriptor,
    prelude::*,
};
use bevy_bae::prelude::*;
use rand::Rng;

use crate::{berries::berry_plugin, ghost::ghost_plugin, leaderbord::leaderboard_plugin};

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
            berry_plugin,
            leaderboard_plugin,
        ))
        .run();
}
