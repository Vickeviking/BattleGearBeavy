use bevy::{prelude::*, window::PrimaryWindow};
use crate::components::{Camera, MyAssets};
use crate::DEBUG;

pub fn spawn_2dcamera (mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..Default::default()
        },
        Camera{}
    ));
}


/*
Camera3dBundle {
                transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            }
*/

pub fn switch_to_3d_camera(
    mut commands: Commands,
    mut query: Query<(Entity, &Camera)>,
) {
    // Remove the existing 2D camera bundle and insert a new 3D camera bundle
    for (camera, _) in query.iter_mut() {
        commands.entity(camera)
            .remove::<Camera2dBundle>()
            .insert(Camera3dBundle {
                transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            });
    }

    if DEBUG {
        info!("Switched to 3D camera");
    }
}

pub fn switch_to_2d_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &Camera)>
) {
    let window = window_query.get_single().unwrap();

    // Remove the existing 3D camera bundle and insert a new 2D camera bundle
    for (entity, _) in query.iter_mut() {
        commands.entity(entity)
            .remove::<Camera3dBundle>()
            .insert(Camera2dBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                ..Default::default()
            });
    }

    if DEBUG {
        info!("Switched to 2D camera");
    }
}

// asset loading

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hero_image = asset_server.load("branding/heroimage.png");
    let fira_sans_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let irish_grover = asset_server.load("fonts/IrishGrover-Regular.ttf");
    let anton = asset_server.load("fonts/Anton-Regular.ttf");

    commands.insert_resource(MyAssets { hero_image, fira_sans_bold, irish_grover, anton });
}