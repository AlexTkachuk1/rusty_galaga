use std::f32::consts::PI;

use self::formation::{FormationMaker, Formation};
use crate::{
    components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity},
    EnemyCount, GameTexture, WindowSize, BASE_SPEED, ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE,
    TIME_STEP,
};
use bevy::{ecs::query, prelude::*, transform::commands};
use rand::{thread_rng, Rng};

mod formation;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FormationMaker::default())
            .add_systems(FixedUpdate, enemy_spawn_system)
            .insert_resource(Time::<Fixed>::from_seconds(1.0))
            .add_systems(Update, enemy_fire_system.run_if(enemy_fire_criteria))
            .add_systems(Update, enemy_movement_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTexture>,
    mut enemy_count: ResMut<EnemyCount>,
    mut formation_maker: ResMut<FormationMaker>, 
    win_size: Res<WindowSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;

        commands
            .spawn(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    scale: Vec3::new(0.5, 0.5, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(formation)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

fn enemy_fire_criteria() -> bool {
    if thread_rng().gen_bool(1. / 120.) {
        true
    } else {
        false
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTexture>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);

        commands
            .spawn(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15., 10.0),
                    rotation: Quat::from_rotation_x(PI),
                    scale: Vec3::new(1., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable { auto_despown: true })
            .insert(Velocity { x: 0., y: -1. });
    }
}

fn enemy_movement_system(mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>, time: Res<Time>) {
    let now = time.elapsed_seconds_f64() as f32;

    for (mut transform, mut formation) in query.iter_mut() {
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);

        let max_distance = TIME_STEP * formation.speed;

        // fixtures (hardcode for now)
        let dir: f32 = if formation.start.0 < 0. { -1. } else { 1. }; // 1 for countre clockwise, -1 clockwise
        let (x_pivot, y_pivot) = formation.pivot;
        let (x_radius, y_radius) = formation.radius;

        let angle = formation.angle + dir * formation.speed * TIME_STEP * (x_radius.min(x_radius * PI / 2.));
        let x_dst = x_radius * angle.cos() + x_pivot;
        let y_dst = y_radius * angle.sin() + y_pivot;

        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();

        let distance_ratio = if distance != 0. {
            max_distance / distance
        } else {
            0.
        };

        let x = x_org - dx * distance_ratio;
        let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
        let y = y_org - dy * distance_ratio;
        let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

        if distance < max_distance * formation.speed / 20. {
            formation.angle = angle;
        }

        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
    }
}
