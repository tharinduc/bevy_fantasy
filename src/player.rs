use crate::GameTextures;
use crate::Player;
use crate::Velocity;
use crate::{BASE_SPEED, TIME_STEP};
use crate::PlayerFace;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, add_player)
            .add_system(hanlde_input)
            .add_system(move_player);
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
        .insert(Player {facing: PlayerFace::Right, shooting: false })
        .insert(Velocity { x: 0.0, y: 0.0 });
}

fn hanlde_input(input: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &mut Player)>) {
    for (mut velocity, mut player) in query.iter_mut() {
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


fn move_player(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
