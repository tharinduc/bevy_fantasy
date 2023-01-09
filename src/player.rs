use crate::GameTextures;
use crate::Player;
use crate::Velocity;
use crate::{BASE_SPEED, TIME_STEP};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, add_player)
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
        .insert(Player)
        .insert(Velocity { x: 0.0, y: 0.0 });
}

fn move_player(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
