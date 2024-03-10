use crate::{
    components::{FromPlayer, Laser, Movable, Player, SpriteSize, Velocity}, GameTexture, PlayerState, WindowSize, BASE_SPEED, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE, TIME_STEP
};
use bevy::{ecs::query, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerState::default())
            .add_systems(FixedUpdate, player_spawn_system)
            .insert_resource(Time::<Fixed>::from_seconds(0.5))
            .add_systems(Update, player_keyboard_event_system)
            .add_systems(Update, player_fier_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    win_size: Res<WindowSize>,
    game_textures: Res<GameTexture>,
) {
    let now = time.elapsed_seconds_f64();
    let last_shot = player_state.last_shot;

    if !player_state.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_DELAY) {
        let bottom = -win_size.h / 2f32;
        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform {
                    scale: Vec3::new(0.06, 0.06, 0.0),
                    translation: Vec3::new(0f32, (bottom + PLAYER_SIZE.1 * 0.06 * 0.5) + 15f32, 10f32),
                    ..Default::default()
                },
                ..default()
            })
            .insert(Player)
            .insert(SpriteSize::from(PLAYER_SIZE))
            .insert(Movable {
                auto_despown: false,
            })
            .insert(Velocity { x: 0f32, y: 0f32 });

        player_state.spawned();
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1f32
        } else if kb.pressed(KeyCode::Right) {
            1f32
        } else {
            0f32
        }
    }
}

fn player_fier_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTexture>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 * 0.06 * 0.3 - 1.0;

            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            scale: Vec3::new(0.4, 0.4, 0.0),
                            translation: Vec3::new(x + x_offset, y, 0.0),
                            ..Default::default()
                        },
                        ..default()
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Movable { auto_despown: true })
                    .insert(Velocity { x: 0f32, y: 1f32 });
            };

            spawn_laser(x_offset);
            spawn_laser(-x_offset);
        }
    }
}
