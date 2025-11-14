#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_bae::prelude::*;

use crate::berries::{Berry, NewBerry};

mod berries;

const SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BaePlugin::default()))
        .add_systems(Startup, setup)
        .add_message::<NewBerry>()
        .add_systems(Update, berries::spawn_berries)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Plan::new(),
        Sprite::from_image(asset_server.load("collector.png")),
        Sequence,
        tasks!(
            Operator::new(find_closest_berry),
            Operator::new(go_to_berry),
            Operator::new(collect_berry)
        ),
    ));
    commands.spawn((
        Plan::new(),
        Sprite::from_image(asset_server.load("collector.png")),
        Sequence,
        tasks!(
            Operator::new(find_closest_berry),
            Operator::new(go_to_berry),
            Operator::new(collect_berry)
        ),
        Transform::from_translation(Vec3::splat(100.0)),
    ));
}

#[derive(Component)]
pub struct TargetBerry(pub Entity);

fn find_closest_berry(
    In(input): In<OperatorInput>,
    mut commands: Commands,
    berries: Query<(Entity, &Transform), With<Berry>>,
    planner: Query<&Transform, With<Plan>>,
) -> OperatorStatus {
    let pos = planner.get(input.entity).unwrap().translation.xy();
    let mut closest: Option<Entity> = None;
    let mut closest_dist = 0.0;
    for (entity, transform) in berries {
        let dist = transform.translation.xy().distance(pos);
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
    mut planners: Query<(&mut Transform, &TargetBerry), With<Plan>>,
    berries: Query<&Transform, (With<Berry>, Without<Plan>)>,
    time: Res<Time>,
    mut news: MessageReader<NewBerry>,
    mut commands: Commands,
) -> OperatorStatus {
    let Ok((mut trans, target_entity)) = planners.get_mut(input.entity) else {
        return OperatorStatus::Failure;
    };

    if let Ok(target) = berries.get(target_entity.0) {
        for new in news.read() {
            let new_trans = berries.get(new.0).unwrap();

            if new_trans
                .translation
                .xy()
                .distance_squared(trans.translation.xy())
                < target
                    .translation
                    .xy()
                    .distance_squared(trans.translation.xy())
            {
                commands.entity(input.entity).insert(TargetBerry(new.0));
                return OperatorStatus::Ongoing;
            }
        }

        let dir = (target.translation.xy() - trans.translation.xy()).normalize();
        let mov = dir * SPEED * time.delta_secs();
        if (target.translation.xy() - (trans.translation.xy() + mov)).length() < mov.length() {
            trans.translation = target.translation;
            return OperatorStatus::Success;
        }
        trans.translation += mov.extend(0.0);
    } else {
        return OperatorStatus::Failure;
    }
    OperatorStatus::Ongoing
}

fn collect_berry(
    In(input): In<OperatorInput>,
    planners: Query<&TargetBerry, With<Plan>>,
    mut commands: Commands,
) -> OperatorStatus {
    let berry = planners.get(input.entity).unwrap().0;
    commands.entity(berry).despawn();
    info!("Collected berry {:?}!", berry);
    commands.entity(input.entity).remove::<TargetBerry>();
    OperatorStatus::Success
}
