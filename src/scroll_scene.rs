use crate::components::*;
use crate::constants::*;
use crate::{GameTextures, WinSize};
use bevy::core::FixedTimestep;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScrollScene;

struct ScrollSceneState {
    size: usize,
}

impl Default for ScrollSceneState {
    fn default() -> Self {
        Self { size: 1 }
    }
}

impl ScrollSceneState {
    pub fn spawned(&mut self) {
        self.size += 1;
    }
    pub fn despawned(&mut self) {
        self.size -= 1;
    }
}

pub struct ScrollScenePlugin;

impl Plugin for ScrollScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScrollSceneState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(spawn_scroll_scene),
            )
            .add_system(scroll_scene_movement);
    }
}

fn spawn_scroll_scene(
    mut commands: Commands,
    mut scroll_scene_state: ResMut<ScrollSceneState>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let x_pos = (win_size.w - PIPE_SIZE.0) * 0.5;
    let y_pos = (-win_size.h + PIPE_SIZE.1) * 0.5;

    if scroll_scene_state.size <= SCROLL_SIZE {
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.pipe.clone(),
                transform: Transform {
                    translation: Vec3::new(x_pos, y_pos, 0.),
                    // scale: Vec3::new(1., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ScrollScene)
            .insert(Velocity { x: 1., y: 0. });
        scroll_scene_state.spawned();
    }
}

fn scroll_scene_movement(
    mut commands: Commands,
    mut scroll_scene_state: ResMut<ScrollSceneState>,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<ScrollScene>>,
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
            scroll_scene_state.despawned();
        }
    }
}
