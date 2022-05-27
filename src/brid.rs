use crate::{
    components::{GameTextures, Velocity},
    constants::BIRD_SPRITE_SIZE,
};
use bevy::{core::FixedTimestep, prelude::*, sprite::Anchor};

#[derive(Component)]
pub struct Brid;

pub struct BridState {
    pub on: bool,
}

impl Default for BridState {
    fn default() -> Self {
        Self { on: false }
    }
}

#[derive(Component, Deref, DerefMut)]
struct BridAnimationTimer(Timer);

pub struct BridPlugin;

impl Plugin for BridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BridState::default())
            .add_system_set(SystemSet::new().with_system(spawn_brid))
            .add_system(brid_animation);
    }
}

fn spawn_brid(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut brid_state: ResMut<BridState>,
) {
    if !brid_state.on {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.brid.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    scale: Vec3::new(2., 2., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Brid)
            .insert(BridAnimationTimer(Timer::from_seconds(0.2, true)));

        brid_state.on = true;
    }
}

fn brid_animation(
    time: Res<Time>,
    mut query: Query<(&mut BridAnimationTimer, &mut TextureAtlasSprite), With<Brid>>,
) {
    query.iter_mut().for_each(|(mut timer, mut sprite)| {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % BIRD_SPRITE_SIZE;
        }
    });
}
