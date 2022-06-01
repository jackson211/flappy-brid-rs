use crate::{
    components::{GameTextures, SpriteSize, WinSize},
    constants::{
        BASE_SPEED, BIRD_SPRITE_NUMBER, BIRD_SPRITE_SIZE, BRID_SCALE, JUMP_FORCE, TIME_STEP,
    },
    physics::Velocity,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Brid;

pub struct BridState {
    pub on: bool,
    pub score: f32,
}

impl Default for BridState {
    fn default() -> Self {
        Self {
            on: false,
            score: 0.0,
        }
    }
}

impl BridState {
    pub fn reset(&mut self) {
        self.on = false;
        self.score = 0.0;
    }
}

#[derive(Component, Deref, DerefMut)]
struct BridAnimationTimer(Timer);

pub struct BridPlugin;

impl Plugin for BridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BridState::default())
            .add_system_set(SystemSet::new().with_system(spawn_brid))
            .add_system(brid_animation)
            .add_system(brid_movement)
            .add_system(brid_keyboard_event_system);
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
                    scale: Vec3::new(BRID_SCALE, BRID_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Brid)
            .insert(BridAnimationTimer(Timer::from_seconds(0.2, true)))
            .insert(Velocity { x: 0., y: 1. })
            .insert(SpriteSize::from(BIRD_SPRITE_SIZE));
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
            sprite.index = (sprite.index + 1) % BIRD_SPRITE_NUMBER;
        }
    });
}

fn brid_movement(
    mut commands: Commands,
    mut bridstate: ResMut<BridState>,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<Brid>>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.y -= velocity.y * TIME_STEP * BASE_SPEED;

        let h_bound = win_size.h * 0.5;
        let w_bound = win_size.w * 0.5;

        if translation.y > h_bound
            || translation.y < -h_bound
            || translation.x > w_bound
            || translation.x < -w_bound
        {
            commands.entity(entity).despawn();
            bridstate.reset();
        }
    }
}

fn brid_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Brid>>,
) {
    if let Ok((velocity, mut transform)) = query.get_single_mut() {
        let translation = &mut transform.translation;
        if kb.pressed(KeyCode::Space) {
            translation.y += velocity.y * TIME_STEP * BASE_SPEED * JUMP_FORCE
        }
    }
}
