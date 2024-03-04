use bevy::{
    ecs::query, prelude::*, sprite::collide_aabb::collide, window::{WindowPosition, WindowTheme}
};
use components::{Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
mod enemy;
mod player;

// region: --- Assets Constants
const PLAYER_SPRITE: &str = "../assets/hero.png";
const PLAYER_SIZE: (f32, f32) = (1024f32, 1024f32);

const ENEMY_SPRITE: &str = "../assets/enemy.png";
const ENEMY_SIZE: (f32, f32) = (103f32, 84f32);

const PLAYER_LASER_SPRITE: &str = "../assets/laser.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9f32, 54f32);

const ENEMY_LASER_SPRITE: &str = "../assets/enemyLaser.png";
const ENEMY_LASER_SIZE: (f32, f32) = (9f32, 37f32);

const EXPLOSION_SHEET: &str = "../assets/explosion.png";
const EXPLOSION_LEN: usize = 16;
// endregion: --- Assets Constants

// region: --- Game Constants
const TIME_STEP: f32 = 1f32 / 60f32;
const BASE_SPEED: f32 = 200f32;
// endregion: --- Game Constants

// region: --- Resourses
pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

impl Resource for WindowSize {}

struct GameTexture {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

impl Resource for GameTexture {}

// endregion: --- Resourses

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.02, 0.02, 0.02)))
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Welcom to Rusty Galaga".into(),
                resolution: (600., 900.).into(),
                position: WindowPosition::At(IVec2::new(1300, 0)),
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    // maximize: false,
                    ..Default::default()
                },
                visible: true,
                // resizable: false,
                ..default()
            }),
            ..default()
        }),))
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(Update, movable_system)
        .add_systems(Update, player_laser_hit_enemy_system)
        .add_systems(Update, explosion_to_spawn_system)
        .add_systems(Update, explosion_animation_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: Query<&mut Window>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.single_mut();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WindowSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(64f32, 64f32),
        4,
        4,
        Some(Vec2::new(0f32, 0f32)), 
        Some(Vec2::new(0f32, 0f32))
    );
    let explosion = texture_atlases.add(texture_atlas);


    let game_textures = GameTexture {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
    };
    commands.insert_resource(game_textures);
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despown {
            const MARGIN: f32 = 200f32;
            if translation.y > win_size.h / 2f32 + MARGIN
                || translation.y < -win_size.h / 2f32 - MARGIN
                || translation.x > win_size.w / 2f32 + MARGIN
                || translation.x < -win_size.w / 2f32 - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                commands.entity(laser_entity).despawn();

                commands.spawn(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTexture>,
    query: Query<(Entity, &ExplosionToSpawn)>
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: game_textures.explosion.clone(),
            transform: Transform {
                translation: explosion_to_spawn.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Explosion)
        .insert(ExplosionTimer::default());

        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            sprite.index += 1;

            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}
