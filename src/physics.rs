use crate::{
    brid::{Brid, BridState},
    components::SpriteSize,
    pipe::Pipe,
};
use bevy::{
    math::{Vec2, Vec3Swizzles},
    prelude::{Commands, Component, Entity, Query, ResMut, Transform, With},
    sprite::collide_aabb::collide,
    utils::HashSet,
};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub fn collision_system(
    mut commands: Commands,
    mut brid_state: ResMut<BridState>,
    brid_query: Query<(Entity, &Transform, &SpriteSize), With<Brid>>,
    mut pipe_query: Query<(&mut Pipe, &Transform, &SpriteSize), With<Pipe>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (mut pipe, pipe_tf, pipe_size) in pipe_query.iter_mut() {
        if !pipe.pass {
            for (brid_entity, brid_tf, brid_size) in brid_query.iter() {
                // Skip if the brid despawned
                if despawned_entities.contains(&brid_entity) {
                    continue;
                }
                let brid_scale = Vec2::from(brid_tf.scale.xy());
                let pipe_scale = Vec2::from(pipe_tf.scale.xy());
                // determine if collision
                let collision = collide(
                    pipe_tf.translation,
                    pipe_size.size() * pipe_scale,
                    brid_tf.translation,
                    brid_size.size() * brid_scale,
                );

                // perform collision
                if let Some(_) = collision {
                    // remove brid
                    commands.entity(brid_entity).despawn();
                    brid_state.reset();
                    despawned_entities.insert(brid_entity);
                }
                // if brid passes pipe
                if pipe_tf.translation.x < brid_tf.translation.x {
                    brid_state.score += 0.5;
                    pipe.pass = true;
                }
            }
        }
    }
}
