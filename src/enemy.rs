use crate::Enemy;
use crate::GameTextures;
use crate::SpriteSize;
use crate::Velocity;
use crate::ENEMY_SPRITE_SIZE;
use crate::{BASE_SPEED, TIME_STEP};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, add_enemy)
            .add_system(move_enemy);
    }
}

fn add_enemy(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands
        .spawn(SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform {
                translation: Vec3::new(150.0, 200.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpriteSize::from(ENEMY_SPRITE_SIZE))
        .insert(Enemy)
        .insert(Velocity { x: 0.1, y: 0.0 });
}

fn move_enemy(mut query: Query<(&mut Velocity, &mut Transform), With<Enemy>>) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if translation.x >= 200.0 || translation.x <= 100.0 {
            velocity.x = velocity.x * -1.0;
        }
    }
}
