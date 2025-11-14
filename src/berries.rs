use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Berry;

pub fn spawn_berries(
    mut commands: Commands,
    mut timer: Local<Timer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    timer.set_duration(Duration::from_secs(2));
    timer.tick(time.delta());
    if timer.is_finished() {
        commands.spawn((
            Berry,
            Transform::from_translation(
                (rand::rng().random::<Vec2>() - Vec2::splat(0.5)).extend(0.0) * 1000.0,
            ),
            Sprite::from_image(asset_server.load("berry.png")),
        ));
        timer.reset();
    }
}
