use bevy::prelude::*;
use bevy::window::{PresentMode, Windows};

mod bullet;
mod enemy;
mod player;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PLAYER_SPRITE: &str = "player.png";
const ENEMY_SPRITE: &str = "enemy.png";
const ENEMY_SPRITE_SIZE: (f32, f32) = (32.0, 32.0);
const BULLET_SPRITE: &str = "bullet.png";
const BULLET_SPRITE_SIZE: (f32, f32) = (32.0, 32.0);
const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Fantasy".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_textures)
        .add_startup_system(setup)
        .add_system(close_window)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn close_window(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if input.just_pressed(KeyCode::Escape) {
        let window = windows.primary_mut();
        window.close();
    }
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        bullet: asset_server.load(BULLET_SPRITE),
    };
    commands.insert_resource(game_textures);
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    enemy: Handle<Image>,
    bullet: Handle<Image>,
}

#[derive(Component)]
struct Player {
    facing: PlayerFace,
    shooting: bool,
}

enum PlayerFace {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct SpriteSize(Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}
