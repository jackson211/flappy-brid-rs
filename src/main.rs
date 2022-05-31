mod brid;
mod components;
mod constants;
mod physics;
mod pipe;
mod score;

use bevy::{prelude::*};
use brid::*;
use components::*;
use constants::*;
use physics::*;
use pipe::*;
use score::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // add window size to resource
    let window = windows.get_primary_mut().unwrap();
    let win_size = WinSize {
        w: window.width(),
        h: window.height(),
    };
    commands.insert_resource(win_size);

    // load all game texture
    let texture_handle = asset_server.load(BIRD_SPRITE);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 16.0), 4, 1);
    let brid = texture_atlases.add(texture_atlas);
    let game_textures = GameTextures {
        pipe: asset_server.load(PIPE_SPRITE),
        brid,
    };
    commands.insert_resource(game_textures);

    // spawn_score(commands, asset_server);
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
            title: String::from("Flippy Bird rs"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BridPlugin)
        .add_plugin(PipePlugin)
        .add_plugin(ScorePlugin)
        .add_startup_system(setup)
        .add_system(collision_system)
        .run();
}
