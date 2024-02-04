use crate::{
    components::{Player, Velocity},
    GameTexture, WindowSize, BASE_SPEED, PLAYER_SIZE, TIME_STEP,
};
use bevy::{ecs::query, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, player_spawn_system)
            .add_systems(Update, player_keyboard_event_system)
            .add_systems(Update, player_movement_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    game_textures: Res<GameTexture>,
) {
    let bottom = -win_size.h / 2f32;
    commands
        .spawn(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                scale: Vec3::new(0.1, 0.1, 0.0),
                translation: Vec3::new(0f32, (bottom + PLAYER_SIZE.1 / 2f32 * 0.5) + 15f32, 10f32),
                ..Default::default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Velocity { x: 0f32, y: 0f32 });
}

fn player_keyboard_event_system(kb: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
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

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
