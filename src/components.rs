use bevy::{
    prelude::{Component, Handle, Image},
    sprite::TextureAtlas,
};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

pub struct GameTextures {
    pub pipe: Handle<Image>,
    pub brid: Handle<TextureAtlas>,
}
