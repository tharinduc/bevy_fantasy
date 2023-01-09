use bevy::prelude::*;
use bevy::window::{PresentMode, Windows};

mod enemy;
mod player;

const PLAYER_SPRITE: &str = "player.png";
const ENEMY_SPRITE: &str = "enemy.png";
const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Fantasy".to_string(),
                width: 640.0,
                height: 480.0,
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_textures)
        .add_startup_system(setup)
        .add_system(hanlde_input)
        .add_system(change_clear_color)
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

fn hanlde_input(input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut velocity) = query.get_single_mut() {
        if input.pressed(KeyCode::Left) {
            velocity.x = -1.0;
        } else if input.pressed(KeyCode::Right) {
            velocity.x = 1.0;
        } else if input.pressed(KeyCode::Up) {
            velocity.y = 1.0;
        } else if input.pressed(KeyCode::Down) {
            velocity.y = -1.0;
        } else {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

fn change_clear_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = Color::PURPLE;
    }
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
    };
    commands.insert_resource(game_textures);
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    enemy: Handle<Image>,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}
