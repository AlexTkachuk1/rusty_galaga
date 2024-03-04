use bevy::{prelude::*, transform::commands};
use rand::{thread_rng, Rng};
use crate::{components::{Enemy, SpriteSize}, GameTexture, WindowSize, ENEMY_SIZE};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, enemy_spawn_system);
    }
}

fn enemy_spawn_system(
      mut commands: Commands, 
      game_textures: Res<GameTexture>,
      win_size: Res<WindowSize>
) {
      let mut rng = thread_rng();
      let w_span = win_size.w / 2. - 100.0;
      let h_span = win_size.h / 2. - 100.0;
      let x = rng.gen_range(-w_span..w_span);
      let y = rng.gen_range(-h_span..h_span);

      commands.spawn(SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform {
                  translation: Vec3::new(x, y, 10.0),
                  scale: Vec3::new(0.5, 0.5, 0.0),
                  ..Default::default()
              },
            ..Default::default()
      })
      .insert(Enemy)
      .insert(SpriteSize::from(ENEMY_SIZE));

}