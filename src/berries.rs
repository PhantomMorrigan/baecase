use bevy::prelude::*;

use crate::{ghost::TargetedBerry, sample_arena};

pub fn berry_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, spawn_berries)
        .add_message::<NewBerry>()
        .add_observer(
            |trigger: On<Add, TargetedBerry>, mut sprite: Query<&mut Sprite>| {
                sprite.get_mut(trigger.entity).unwrap().color = Color::linear_rgb(2.0, 1.0, 0.8);
            },
        )
        .add_observer(
            |trigger: On<Remove, TargetedBerry>, mut sprite: Query<&mut Sprite>| {
                sprite.get_mut(trigger.entity).unwrap().color = Color::WHITE;
            },
        );
}

#[derive(Component)]
pub struct Berry;

#[derive(Message)]
pub struct NewBerry(pub Entity);

pub fn spawn_berries(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut new_berries: MessageWriter<NewBerry>,
) {
    for _ in 0..5 {
        let new = commands
            .spawn((
                Berry,
                Transform::from_translation(sample_arena(&mut rand::rng()).extend(0.0)),
                Sprite::from_image(asset_server.load("berry.png")),
            ))
            .id();
        new_berries.write(NewBerry(new));
    }
}
