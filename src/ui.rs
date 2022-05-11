use std::collections::HashMap;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::simulation::Simulated;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct PositionText;

/// Plugin used to create and update UI components.
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_startup_system(setup_ui);
        app.add_system(update_fps);
        app.add_system(update_positions_of_gravity_components);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/AstroSpace.ttf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/AstroSpace.ttf"),
                            font_size: 24.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(FpsText);
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

fn update_positions_of_gravity_components(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gravity_query: Query<(&Transform, &Simulated)>,
    mut text_query: Query<&mut Text, With<PositionText>>,
) {
    let mut entities: HashMap<String, Transform> = HashMap::new();

    for (transform, simulated) in gravity_query.iter() {
        entities.insert(simulated.0.clone(), *transform);
    }

    for mut text in text_query.iter_mut() {
        if entities.contains_key(&text.sections[0].value) {
            let transform = entities[&text.sections[0].value];
            text.sections[1].value = format!("{:}", transform.translation.x);
            text.sections[2].value = format!("{:}", transform.translation.y);
            text.sections[3].value = format!("{:}", transform.translation.z);

            entities.remove(&text.sections[0].value);
        }
    }

    for (simulated, _transform) in entities.iter() {
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                // Use `Text` directly
                text: Text {
                    // Construct a `Vec` of `TextSection`s
                    sections: vec![
                        TextSection {
                            value: format!("{}", simulated),
                            style: TextStyle {
                                font: asset_server.load("fonts/AstroSpace.ttf"),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "-".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/AstroSpace.ttf"),
                                font_size: 24.0,
                                color: Color::RED,
                            },
                        },
                        TextSection {
                            value: "-".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/AstroSpace.ttf"),
                                font_size: 24.0,
                                color: Color::GREEN,
                            },
                        },
                        TextSection {
                            value: "-".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/AstroSpace.ttf"),
                                font_size: 24.0,
                                color: Color::BLUE,
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            })
            .insert(PositionText);
    }
}