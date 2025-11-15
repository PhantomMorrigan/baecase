use bevy::prelude::*;

use crate::BerriesEaten;

#[derive(Component)]
pub struct LeaderBoard;

pub fn setup(mut commands: Commands) {
    let id = commands
        .spawn((
            children![Text::new("Ghost Leaderboard")],
            Node {
                position_type: PositionType::Absolute,
                top: px(5),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .id();

    let list = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            LeaderBoard,
            ChildOf(id),
        ))
        .id();

    for _ in 0..10 {
        commands.spawn((
            Text::new("Waiting for ghosts..."),
            TextFont::from_font_size(17.5),
            ChildOf(list),
        ));
    }
}

pub fn update_leaderboard(
    board: Single<&Children, With<LeaderBoard>>,
    mut texts: Query<&mut Text>,
    ghosts: Query<(Entity, &BerriesEaten)>,
) {
    ghosts
        .iter()
        .sort_by_key::<(Entity, &BerriesEaten), usize>(|(_, e)| e.0)
        .zip(board.iter().rev())
        .for_each(|((ghost, eaten), text)| {
            texts.get_mut(text).unwrap().0 = format!(
                "Ghost {:>5} has eaten {} berries!",
                ghost.to_string(),
                eaten.0
            );
        });
}
