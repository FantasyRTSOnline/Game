use bevy::app::{App, Plugin};
use bevy::ecs::system::Commands;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::transform::components::Transform;

const CAMERA_DISTANCE: f32 = 80.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands)
{
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}
