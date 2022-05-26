mod components;
mod scroll_scene;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use scroll_scene::*;

const PIPE_SPRITE: &str = "pipe.png";
const PIPE_SIZE: (f32, f32) = (32., 128.);

const WIN_WIDTH: f32 = 600.0;
const WIN_HEIGHT: f32 = 800.0;

struct WinSize {
    w: f32,
    h: f32,
}
struct GameTextures {
    pipe: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // add window size to resource
    let window = windows.get_primary_mut().unwrap();
    let win_size = WinSize {
        w: window.width(),
        h: window.height(),
    };
    commands.insert_resource(win_size);

    // load all game texture
    let game_textures = GameTextures {
        pipe: asset_server.load(PIPE_SPRITE),
    };
    commands.insert_resource(game_textures);
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
            title: String::from("Map Example"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ScrollScenePlugin)
        .add_startup_system(setup)
        .run();
}
