use bevy::prelude::*;

use crate::{brid::BridState, constants::FONT_PATH};

#[derive(Component)]
struct Score;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set(
            SystemSet::new()
                // .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_score),
        )
        .add_system(score_update_system);
    }
}

fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT_PATH);
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score:".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(Score);
}

fn score_update_system(brid_state: Res<BridState>, mut query: Query<&mut Text, With<Score>>) {
    for mut text in query.iter_mut() {
        // Update the value of the second section
        text.sections[1].value = format!("{}", brid_state.score);
    }
}
