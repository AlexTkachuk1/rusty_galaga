use bevy::{prelude::*, window::WindowPosition, window::WindowTheme};
use player::PlayerPlugin;
mod components;
mod player;

// region: --- Assets Constants
const PLAYER_SPRITE: &str = "../assets/hero.png";
const PLAYER_SIZE: (f32, f32) = (200f32, 200f32);
const ENEMY_SPRITE: &str = "../assets/enemy.png";
const LASER_SPRITE: &str = "../assets/laser.png";
const ENEMY_LASER_SPRITE: &str = "../assets/enemyLaser.png";
const EXPLOSION_SPRITE: &str = "../assets/explosion.png";
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
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: Query<&mut Window>) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.single_mut();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WindowSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    let game_textures = GameTexture {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures);
}
