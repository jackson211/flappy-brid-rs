use bevy::{
    math::Vec2,
    prelude::{Component, Handle, Image},
    sprite::TextureAtlas,
};

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

pub struct GameTextures {
    pub pipe: Handle<Image>,
    pub brid: Handle<TextureAtlas>,
}

#[derive(Component, Copy, Clone)]
pub struct SpriteSize(pub Vec2);
impl SpriteSize {
    pub fn size(self) -> Vec2 {
        self.0
    }
}

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}
