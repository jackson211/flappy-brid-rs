use bevy::{core::FixedTimestep, prelude::*};

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
    let font = asset_server.load("fonts/Noto Mono for Powerline.ttf");
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
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

fn score_update_system(time: Res<Time>, mut query: Query<&mut Text, With<Score>>) {
    for mut text in query.iter_mut() {
        // Update the value of the second section
        text.sections[1].value = format!("{}", time.delta_seconds());
    }
}
