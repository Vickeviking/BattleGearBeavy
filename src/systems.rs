use bevy::{prelude::*, window::PrimaryWindow};
use crate::components::MyAssets;

pub fn spawn_camera (mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..Default::default()
        }
    );

}

// asset loading

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hero_image = asset_server.load("branding/heroimage.png");
    let fira_sans_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let irish_grover = asset_server.load("fonts/IrishGrover-Regular.ttf");
    let anton = asset_server.load("fonts/Anton-Regular.ttf");

    commands.insert_resource(MyAssets { hero_image, fira_sans_bold, irish_grover, anton });
}