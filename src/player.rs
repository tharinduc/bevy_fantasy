use crate::Barrier;
use crate::Bullet;
use crate::Enemy;
use crate::GameTextures;
use crate::PlayerFace;
use crate::SpriteSize;
use crate::Velocity;
use crate::PLAYER_SPRITE_SIZE;
use crate::{Player, PlayerId};
use crate::{BASE_SPEED, TIME_STEP};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_egui::{egui, EguiContext};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, add_player)
            .add_system(hanlde_input)
            .add_system(move_player)
            .add_system(detect_path)
            .add_system(destroy_enemy);
    }
}

fn add_player(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands
        .spawn(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(-150.0, -200.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerId)
        .insert(Player {
            facing: PlayerFace::Right,
            shooting: false,
        })
        .insert(SpriteSize::from(PLAYER_SPRITE_SIZE))
        .insert(Velocity { x: 0.0, y: 0.0 });
}

fn hanlde_input(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Player), With<PlayerId>>,
) {
    for (mut velocity, mut player) in player_query.iter_mut() {
        if input.pressed(KeyCode::Left) {
            velocity.x = -1.0;
            player.facing = PlayerFace::Left;
        } else if input.pressed(KeyCode::Right) {
            velocity.x = 1.0;
            player.facing = PlayerFace::Right;
        } else if input.pressed(KeyCode::Up) {
            velocity.y = 1.0;
            player.facing = PlayerFace::Up;
        } else if input.pressed(KeyCode::Down) {
            velocity.y = -1.0;
            player.facing = PlayerFace::Down;
        } else {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }

        if input.just_pressed(KeyCode::Space) {
            player.shooting = true;
        } else {
            player.shooting = false;
        }
    }
}

fn detect_path(
    mut player_query: Query<(&mut Velocity, &Transform, &SpriteSize), With<PlayerId>>,
    wall_query: Query<(&Transform, &SpriteSize, &Barrier)>,
    input: Res<Input<KeyCode>>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    for (mut player_velocity, player_tf, player_size) in player_query.iter_mut() {
        for (wall_tf, wall_size, barrier) in wall_query.iter() {
            let collision = collide(
                wall_tf.translation,
                wall_size.0,
                player_tf.translation,
                player_size.0,
            );

            if let Some(face) = collision {
                match face {
                    Collision::Top => {
                        player_velocity.y = -1.0;
                    }
                    Collision::Bottom => {
                        player_velocity.y = 1.0;
                    }
                    Collision::Right => {
                        player_velocity.x = -1.0;
                    }
                    Collision::Left => {
                        player_velocity.x = 1.0;
                    }
                    _ => {}
                }

                if barrier.destructible {
                    egui::Window::new("")
                        .title_bar(false)
                        .fixed_pos((
                            player_tf.translation.x + WINDOW_WIDTH / 2.0,
                            player_tf.translation.y + WINDOW_HEIGHT / 2.0,
                        ))
                        .show(egui_ctx.ctx_mut(), |ui| {
                            ui.label("world");
                        });
                    if input.just_pressed(KeyCode::X) {
                        println!("open door");
                    }
                }
            }
        }
    }
}

fn move_player(mut player_query: Query<(&Velocity, &mut Transform), With<PlayerId>>) {
    for (player_velocity, mut player_tf) in player_query.iter_mut() {
        let translation = &mut player_tf.translation;
        translation.x += player_velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += player_velocity.y * TIME_STEP * BASE_SPEED;
    }
}

fn destroy_enemy(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &SpriteSize), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    for (bullet_entity, bullet_tf, bullet_size) in bullet_query.iter() {
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            let collision = collide(
                bullet_tf.translation,
                bullet_size.0,
                enemy_tf.translation,
                enemy_size.0,
            );
            if let Some(_) = collision {
                println!("despawn bullet");
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
