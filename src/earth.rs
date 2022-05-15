use crate::camera::Focused;
use crate::simulation::{PhysicalProperties, Simulated};
use bevy::prelude::*;
use rug::Float;

/// Approximate radius of the earth in meters.
const RADIUS: f32 = 6.371e+6_f32;

/// Approximate mass of the earth in kg.
const MASS: f32 = 5.972e+24_f32; // 5.972e+24_f32;

/// Approximate distance from the sun to earth in meters.
pub const DISTANCE_FROM_SUN: f32 = 150_000_000_000.;

//1989000000000000000000000000000
//5972000000000000000000000
#[derive(Component)]
pub struct Earth;

pub fn setup_earth(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform {
                translation: Vec3::new(DISTANCE_FROM_SUN, 0., 0.),
                rotation: Quat::from_rotation_z(0.4101524),
                scale: Vec3::new(RADIUS * 2., RADIUS * 2., RADIUS * 2.),
                ..default()
            },
            ..default()
        })
        .insert(Earth)
        .insert(Simulated("Earth".to_string()))
        .insert(PhysicalProperties {
            mass: Float::with_val(128, MASS.clone()),
            estimated_radius: Float::with_val(128, RADIUS.clone()),
            acceleration: Vec3::ZERO,
        })
        .insert(Focused)
        .with_children(|earth| {
            earth.spawn_scene(asset_server.load("models/earth_1x.glb#Scene0"));
        });
}
