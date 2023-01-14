use crate::Barrier;
use crate::GameTextures;
use crate::SpriteSize;
use crate::WALL_SPRITE_SIZE;
use crate::DOOR_SPRITE_SIZE;
use bevy::prelude::*;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, add_walls);
    }
}

fn add_walls(mut commands: Commands, game_textures: Res<GameTextures>) {
    let mut x = 0.0;
    let y = 0.0;
    for n in 1..6 {
        if n != 3 {
            commands
                .spawn(SpriteBundle {
                    texture: game_textures.wall.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(SpriteSize::from(WALL_SPRITE_SIZE))
                .insert(Barrier {
                    destructible: false,
                });
        }
        if n ==3 {
            x = x + DOOR_SPRITE_SIZE.0;
        } else {
            x = x + WALL_SPRITE_SIZE.0;
        }
    }
}
