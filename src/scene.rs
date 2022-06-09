use crate::brid::*;
use crate::pipe::*;
use bevy::{core::FixedTimestep, ecs::schedule::StateData, prelude::*};

/// ScenePlugin is a plugin that manages the game state.
/// and load all game components in one place.
pub struct ScenePlugin<T> {
    pub running_state: T,
}

impl<T: StateData> Plugin for ScenePlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeState::default())
            .insert_resource(BridState::default())
            // Add Game State
            .add_system_set(SystemSet::on_enter(self.running_state.clone()).with_system(spawn_pipe))
            // Spawn Pipe
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(spawn_pipe),
            )
            // Spawn brid
            .add_system_set(SystemSet::new().with_system(spawn_brid))
            .add_system(pipe_movement)
            .add_system(brid_animation)
            .add_system(brid_movement)
            .add_system(brid_keyboard_event_system);
    }
}
