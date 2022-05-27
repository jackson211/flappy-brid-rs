/// Game Constants
pub const TIME_STEP: f32 = 1. / 60.;
pub const BASE_SPEED: f32 = 100.;

// Sprite
pub const PIPE_SPRITE: &str = "pipe.png";
pub const PIPE_SIZE: (f32, f32) = (32., 256.);
pub const BIRD_SPRITE: &str = "bridsheet.png";
pub const BIRD_SPRITE_SIZE: usize = 4;

// Window size
pub const WIN_WIDTH: f32 = 600.0;
pub const WIN_HEIGHT: f32 = 800.0;

// Scroll Scene
pub const SCOLL_SPACE: f32 = 150.0;
pub const BETWEEN_SCOLL_SPACE: f32 = 50.0;
pub const SCROLL_SIZE: usize = (WIN_WIDTH * 4. / SCOLL_SPACE) as usize; // screen size: win_width * 2, times 2 scroll component on top and bottom

pub const JUMP_FORCE: f32 = 7.0;
