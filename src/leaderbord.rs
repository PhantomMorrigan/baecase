use bevy::prelude::*;

use crate::BerriesEaten;

#[derive(Component)]
pub struct LeaderBoard;

#[derive(Component)]
pub struct Lowest;

pub fn setup(mut commands: Commands) {
    let id = commands
        .spawn((
            children![Text::new("Ghost Leaderboard")],
            Node {
                position_type: PositionType::Absolute,
                top: px(10),
                left: px(10),
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

    commands.spawn((
        Text::new("Waiting for ghosts..."),
        TextFont::from_font_size(17.5),
        TextColor(Color::oklch(0.7, 0.2, 0.0)),
        Lowest,
        ChildOf(id),
    ));

    for n in 0..10 {
        let color = Color::oklch(0.7, 0.2, 90.0.lerp(135.0, (10 - n) as f32 / 10.0));

        commands.spawn((
            Text::new("Waiting for ghosts..."),
            TextFont::from_font_size(17.5),
            TextColor(color),
            ChildOf(list),
        ));
    }
}

pub fn update_leaderboard(
    board: Single<&Children, With<LeaderBoard>>,
    mut lowest: Single<&mut Text, With<Lowest>>,
    mut texts: Query<&mut Text, Without<Lowest>>,
    ghosts: Query<(&Name, &BerriesEaten)>,
) {
    ghosts
        .iter()
        .sort_by_key::<(&Name, &BerriesEaten), usize>(|(_, e)| e.0)
        .rev()
        .zip(board.iter())
        .for_each(|((ghost, eaten), text)| {
            texts.get_mut(text).unwrap().0 =
                format!("{:<10} ate {} berries!", ghost.to_string(), eaten.0);
        });

    let low = ghosts
        .iter()
        .sort_by_key::<(Entity, &BerriesEaten), usize>(|(_, e)| e.0)
        .next()
        .unwrap();

    lowest.0 = format!("{:<10} ate {} berries!", low.0.to_string(), low.1.0);
}
