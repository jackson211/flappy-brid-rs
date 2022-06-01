use crate::components::SpriteSize;
use crate::constants::*;
use crate::physics::Velocity;
use crate::{GameTextures, WinSize};
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::*;

#[derive(Component)]
pub struct Pipe {
    pub pass: bool,
}

impl Default for Pipe {
    fn default() -> Self {
        Self { pass: false }
    }
}

struct PipeState {
    size: usize,
}

impl Default for PipeState {
    fn default() -> Self {
        Self { size: 1 }
    }
}

impl PipeState {
    pub fn spawned(&mut self) {
        self.size += 1;
    }
    pub fn despawned(&mut self) {
        self.size -= 1;
    }
}

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(spawn_pipe),
            )
            .add_system(pipe_movement);
    }
}

fn spawn_pipe(
    mut commands: Commands,
    mut pipe_state: ResMut<PipeState>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let quad_h = win_size.h * 0.25;
    let mut rng = thread_rng();
    let offset_1 = rng.gen_range(BETWEEN_SCOLL_SPACE..quad_h);
    let offset_2 = rng.gen_range(BETWEEN_SCOLL_SPACE..quad_h);
    let x_pos = win_size.w * 0.5;
    let bottom_y_pos = -quad_h - offset_1;
    let top_y_pos = quad_h + offset_2;

    if pipe_state.size <= PIPE_NUMBER {
        // Spawn bottom Pipe
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.pipe.clone(),
                transform: Transform {
                    translation: Vec3::new(x_pos, bottom_y_pos, 0.),
                    scale: Vec3::new(PIPE_SCALE, PIPE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Pipe::default())
            .insert(Velocity { x: 1., y: 0. })
            .insert(SpriteSize::from(PIPE_SIZE));
        pipe_state.spawned();

        // Spawn top Pipe
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.pipe.clone(),
                transform: Transform {
                    translation: Vec3::new(x_pos, top_y_pos, 0.),
                    scale: Vec3::new(PIPE_SCALE, PIPE_SCALE, 1.),
                    ..Default::default()
                },
                sprite: Sprite {
                    flip_y: true,
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Pipe::default())
            .insert(Velocity { x: 1., y: 0. })
            .insert(SpriteSize::from(PIPE_SIZE));
        pipe_state.spawned();
    }
}

fn pipe_movement(
    mut commands: Commands,
    mut pipe_state: ResMut<PipeState>,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<Pipe>>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x -= velocity.x * TIME_STEP * BASE_SPEED;
        let h_bound = win_size.h * 0.5;
        let w_bound = win_size.w * 0.5;

        if translation.y > h_bound
            || translation.y < -h_bound
            || translation.x > w_bound
            || translation.x < -w_bound
        {
            commands.entity(entity).despawn();
            pipe_state.despawned();
        }
    }
}
