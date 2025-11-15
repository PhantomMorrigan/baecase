use bevy::prelude::*;
use rand::Rng;

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
                Transform::from_translation(
                    (rand::rng().random::<Vec2>() - Vec2::splat(0.5)).extend(0.0) * 1000.0,
                ),
                Sprite::from_image(asset_server.load("berry.png")),
            ))
            .id();
        new_berries.write(NewBerry(new));
    }
}
