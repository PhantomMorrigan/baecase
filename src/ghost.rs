use bevy::prelude::*;
use bevy_bae::prelude::*;
use fake::{Fake, locales::EN};

use crate::{
    berries::{Berry, NewBerry},
    sample_arena,
};

const SPEED: f32 = 100.0;

pub fn ghost_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

#[derive(Component)]
pub struct Ghost;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let mut rng = rand::rng();

    commands.spawn((
        Sprite::from_image(asset_server.load("arena.png")),
        Transform::from_scale(Vec2::splat(18.0).extend(0.0)),
    ));

    for _ in 1..100 {
        commands.spawn((
            Plan::new(),
            Ghost,
            BerriesEaten(0),
            Name::new(fake::faker::name::raw::FirstName(EN).fake::<String>()),
            Sprite::from_image(asset_server.load("ghost.png")),
            Sequence,
            tasks!(
                Operator::new(find_closest_berry),
                Operator::new(go_to_berry),
                Operator::new(collect_berry)
            ),
            Transform::from_translation(sample_arena(&mut rng).extend(0.1)),
        ));
    }
}

#[derive(Component)]
#[relationship(relationship_target = TargetedBerry)]
pub struct TargetBerry(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = TargetBerry)]
pub struct TargetedBerry(Entity);

#[derive(Component)]
pub struct BerriesEaten(pub usize);

fn find_closest_berry(
    In(input): In<OperatorInput>,
    mut commands: Commands,
    berries: Query<(Entity, &Transform), (With<Berry>, Without<TargetedBerry>)>,
    ghosts: Query<&Transform, With<Ghost>>,
) -> OperatorStatus {
    let pos = ghosts.get(input.entity).unwrap().translation.xy();
    let mut closest: Option<Entity> = None;
    let mut closest_dist = 0.0;
    for (entity, transform) in berries {
        let dist = transform.translation.xy().distance_squared(pos);
        if closest.is_none() || dist < closest_dist {
            closest = Some(entity);
            closest_dist = dist;
        }
    }

    if let Some(entity) = closest {
        commands.entity(input.entity).insert(TargetBerry(entity));
        return OperatorStatus::Success;
    }
    OperatorStatus::Ongoing
}

fn go_to_berry(
    In(input): In<OperatorInput>,
    mut ghosts: Query<(&mut Transform, &TargetBerry, &BerriesEaten), With<Ghost>>,
    berries: Query<&Transform, (With<Berry>, Without<Ghost>)>,
    new_berries: Query<&Transform, (With<Berry>, Without<TargetedBerry>, Without<Ghost>)>,
    time: Res<Time>,
    mut news: MessageReader<NewBerry>,
    mut commands: Commands,
) -> Result<OperatorStatus> {
    let (mut trans, target_entity, eaten) = ghosts.get_mut(input.entity)?;

    let target = berries.get(target_entity.0)?;

    for new in news.read() {
        let Ok(new_trans) = new_berries.get(new.0) else {
            continue;
        };

        if (new_trans
            .translation
            .xy()
            .distance_squared(trans.translation.xy())
            - target
                .translation
                .xy()
                .distance_squared(trans.translation.xy()))
            < 30.0
        {
            commands.entity(input.entity).insert(TargetBerry(new.0));
            return Ok(OperatorStatus::Ongoing);
        }
    }

    let dir = (target.translation.xy() - trans.translation.xy()).normalize();
    let mov = dir * (SPEED * (1.0 + (eaten.0 as f32 + 1.0).log10())) * time.delta_secs();
    if (target.translation.xy() - (trans.translation.xy() + mov)).length() < mov.length() {
        trans.translation = target.translation;
        return Ok(OperatorStatus::Success);
    }
    trans.translation += mov.extend(0.0);

    Ok(OperatorStatus::Ongoing)
}

fn collect_berry(
    In(input): In<OperatorInput>,
    mut ghosts: Query<(&TargetBerry, &mut BerriesEaten), With<Ghost>>,
    mut commands: Commands,
) -> Result<OperatorStatus> {
    let (berry, mut eaten) = ghosts.get_mut(input.entity)?;
    eaten.0 += 1;
    commands.entity(berry.0).despawn();
    commands.entity(input.entity).remove::<TargetBerry>();
    Ok(OperatorStatus::Success)
}
