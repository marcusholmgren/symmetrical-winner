use bevy::prelude::*;
use bevy::window::WindowResolution;

const TILE_SIZE: f32 = 64.0;

const LEVEL_1: &str = "\
######
#    #
#P B #
# T  #
######";

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct BoxItem;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct InitialPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Resource)]
pub struct LevelData {
    pub width: i32,
    pub height: i32,
    pub walls: Vec<GridPosition>,
    pub targets: Vec<GridPosition>,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub is_complete: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let lines: Vec<&str> = LEVEL_1.lines().filter(|l| !l.is_empty()).collect();
    let height = lines.len() as i32;
    let width = if height > 0 { lines[0].len() as i32 } else { 0 };

    let mut walls = Vec::new();
    let mut targets = Vec::new();

    let center_x = (width as f32 * TILE_SIZE) / 2.0 - TILE_SIZE / 2.0;
    let center_y = (height as f32 * TILE_SIZE) / 2.0 - TILE_SIZE / 2.0;

    for y in 0..height {
        let line = lines[y as usize];
        let chars: Vec<char> = line.chars().collect();
        for x in 0..width {
            let ch = chars[x as usize];
            let pos = GridPosition { x, y };

            let wx = (x as f32 * TILE_SIZE) - center_x;
            let wy = center_y - (y as f32 * TILE_SIZE);

            // Spawn Floor Tile
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.1, 0.1, 0.14),
                    custom_size: Some(Vec2::new(TILE_SIZE - 2.0, TILE_SIZE - 2.0)),
                    ..default()
                },
                Transform::from_xyz(wx, wy, 0.0),
            ));

            match ch {
                '#' => {
                    walls.push(pos);
                    commands.spawn((
                        Wall,
                        pos,
                        Sprite {
                            color: Color::srgb(0.17, 0.24, 0.31),
                            custom_size: Some(Vec2::new(TILE_SIZE - 2.0, TILE_SIZE - 2.0)),
                            ..default()
                        },
                        Transform::from_xyz(wx, wy, 2.0),
                    ));
                }
                'T' => {
                    targets.push(pos);
                    commands.spawn((
                        Target,
                        pos,
                        Sprite {
                            color: Color::srgb(0.91, 0.26, 0.57),
                            custom_size: Some(Vec2::new(TILE_SIZE - 24.0, TILE_SIZE - 24.0)),
                            ..default()
                        },
                        Transform::from_xyz(wx, wy, 1.0),
                    ));
                }
                'P' => {
                    commands.spawn((
                        Player,
                        pos,
                        InitialPosition { x, y },
                        Sprite {
                            color: Color::srgb(0.0, 0.8, 0.78),
                            custom_size: Some(Vec2::new(TILE_SIZE - 8.0, TILE_SIZE - 8.0)),
                            ..default()
                        },
                        Transform::from_xyz(wx, wy, 4.0),
                    ));
                }
                'B' => {
                    commands.spawn((
                        BoxItem,
                        pos,
                        InitialPosition { x, y },
                        Sprite {
                            color: Color::srgb(0.95, 0.61, 0.07),
                            custom_size: Some(Vec2::new(TILE_SIZE - 8.0, TILE_SIZE - 8.0)),
                            ..default()
                        },
                        Transform::from_xyz(wx, wy, 3.0),
                    ));
                }
                _ => {}
            }
        }
    }

    commands.insert_resource(LevelData {
        width,
        height,
        walls,
        targets,
    });
    commands.insert_resource(GameState::default());

    info!("Starting Bevy Sokoban!");
    info!("Controls: Arrow keys / WASD to move & push box, Spacebar to reset.");
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
    level: Res<LevelData>,
    mut player_query: Query<&mut GridPosition, With<Player>>,
    mut boxes_query: Query<(Entity, &mut GridPosition), (With<BoxItem>, Without<Player>)>,
) {
    if game_state.is_complete {
        return;
    }

    let mut delta = (0, 0);
    if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
        delta = (0, -1);
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown)
        || keyboard_input.just_pressed(KeyCode::KeyS)
    {
        delta = (0, 1);
    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft)
        || keyboard_input.just_pressed(KeyCode::KeyA)
    {
        delta = (-1, 0);
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight)
        || keyboard_input.just_pressed(KeyCode::KeyD)
    {
        delta = (1, 0);
    }

    if delta == (0, 0) {
        return;
    }

    let mut player_pos = match player_query.single_mut() {
        Ok(pos) => pos,
        Err(_) => return,
    };

    let target_x = player_pos.x + delta.0;
    let target_y = player_pos.y + delta.1;

    // Out of bounds check
    if target_x < 0 || target_x >= level.width || target_y < 0 || target_y >= level.height {
        return;
    }

    // Wall check
    if level
        .walls
        .iter()
        .any(|w| w.x == target_x && w.y == target_y)
    {
        return;
    }

    // Box check
    let box_at_target = boxes_query
        .iter()
        .find(|(_, pos)| pos.x == target_x && pos.y == target_y)
        .map(|(entity, _)| entity);

    if let Some(box_entity) = box_at_target {
        let beyond_x = target_x + delta.0;
        let beyond_y = target_y + delta.1;

        if beyond_x < 0 || beyond_x >= level.width || beyond_y < 0 || beyond_y >= level.height {
            return;
        }

        if level
            .walls
            .iter()
            .any(|w| w.x == beyond_x && w.y == beyond_y)
        {
            return;
        }

        let box_beyond = boxes_query
            .iter()
            .any(|(_, pos)| pos.x == beyond_x && pos.y == beyond_y);
        if box_beyond {
            return;
        }

        // Advance box and player
        if let Ok((_, mut box_pos)) = boxes_query.get_mut(box_entity) {
            box_pos.x = beyond_x;
            box_pos.y = beyond_y;
            player_pos.x = target_x;
            player_pos.y = target_y;
            info!("[Bevy Sokoban] Pushed Box to ({}, {})", beyond_x, beyond_y);
        }
    } else {
        // Move player
        player_pos.x = target_x;
        player_pos.y = target_y;
    }
}

fn grid_transform_system(level: Res<LevelData>, mut query: Query<(&GridPosition, &mut Transform)>) {
    let center_x = (level.width as f32 * TILE_SIZE) / 2.0 - TILE_SIZE / 2.0;
    let center_y = (level.height as f32 * TILE_SIZE) / 2.0 - TILE_SIZE / 2.0;

    for (pos, mut transform) in query.iter_mut() {
        let world_x = (pos.x as f32 * TILE_SIZE) - center_x;
        let world_y = center_y - (pos.y as f32 * TILE_SIZE);
        transform.translation.x = world_x;
        transform.translation.y = world_y;
    }
}

fn box_color_system(
    level: Res<LevelData>,
    mut query: Query<(&GridPosition, &mut Sprite), With<BoxItem>>,
) {
    for (pos, mut sprite) in query.iter_mut() {
        let on_target = level.targets.iter().any(|t| t.x == pos.x && t.y == pos.y);
        if on_target {
            sprite.color = Color::srgb(0.18, 0.8, 0.44);
        } else {
            sprite.color = Color::srgb(0.95, 0.61, 0.07);
        }
    }
}

fn check_win_condition(
    level: Res<LevelData>,
    mut game_state: ResMut<GameState>,
    boxes_query: Query<&GridPosition, With<BoxItem>>,
    mut player_query: Query<&mut Sprite, With<Player>>,
) {
    if game_state.is_complete {
        return;
    }

    let all_targets_covered = level
        .targets
        .iter()
        .all(|t| boxes_query.iter().any(|b| b.x == t.x && b.y == t.y));

    if all_targets_covered {
        game_state.is_complete = true;
        info!("======================================");
        info!("[Bevy Sokoban] LEVEL COMPLETE!");
        info!("Press SPACEBAR to reset and play again.");
        info!("======================================");

        if let Ok(mut sprite) = player_query.single_mut() {
            sprite.color = Color::srgb(0.6, 0.35, 0.7);
        }
    }
}

fn reset_level(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(&mut GridPosition, &InitialPosition)>,
    mut player_query: Query<&mut Sprite, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        game_state.is_complete = false;
        info!("[Bevy Sokoban] Resetting level to initial state...");

        for (mut pos, init_pos) in query.iter_mut() {
            pos.x = init_pos.x;
            pos.y = init_pos.y;
        }

        if let Ok(mut sprite) = player_query.single_mut() {
            sprite.color = Color::srgb(0.0, 0.8, 0.78);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Sokoban Game".into(),
                resolution: WindowResolution::new(800, 600),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                grid_transform_system,
                box_color_system,
                check_win_condition,
                reset_level,
            ),
        )
        .run();
}
