use crate::Bullet;
use crate::GameTextures;
use crate::Player;
use crate::PlayerFace;
use crate::SpriteSize;
use crate::Velocity;
use crate::BULLET_SPRITE_SIZE;
use crate::{BASE_SPEED, TIME_STEP};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_bullet).add_system(move_bullet);
    }
}

fn add_bullet(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(&Player, &Transform), With<Player>>,
    ) {
        for (player, transform) in query.iter() {
        let velocity_x: f32;
        let velocity_y: f32;
        match player.facing {
            PlayerFace::Up => {
                velocity_x = 0.0;
                velocity_y = 1.0;
            }
            PlayerFace::Down => {
                velocity_x = 0.0;
                velocity_y = -1.0;
            }
            PlayerFace::Right => {
                velocity_x = 1.0;
                velocity_y = 0.0;
            }
            PlayerFace::Left => {
                velocity_x = -1.0;
                velocity_y = 0.0;
            }
        }
        if player.shooting {
            commands
                .spawn(SpriteBundle {
                    texture: game_textures.bullet.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            transform.translation.x,
                            transform.translation.y,
                            10.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(SpriteSize::from(BULLET_SPRITE_SIZE))
                .insert(Bullet)
                .insert(Velocity {
                    x: velocity_x,
                    y: velocity_y,
                });
        }
    }
}

fn move_bullet(
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<Bullet>>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if translation.x > WINDOW_WIDTH / 2.0
            || translation.x < WINDOW_WIDTH / (-2.0)
            || translation.y > WINDOW_HEIGHT / 2.0
            || translation.y < WINDOW_HEIGHT / (-2.0)
        {
            //println!("->> despawn {entity:?}");
            commands.entity(entity).despawn();
        }
    }
}
