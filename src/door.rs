use crate::Barrier;
use crate::GameTextures;
use crate::SpriteSize;
use crate::DOOR_SPRITE_SIZE;
use bevy::prelude::*;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, add_doors);
    }
}

fn add_doors(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands
        .spawn(SpriteBundle {
            texture: game_textures.door.clone(),
            transform: Transform {
                translation: Vec3::new(72.0, 0.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpriteSize::from(DOOR_SPRITE_SIZE))
        .insert(Barrier {
            destructible: true,
        });
}
