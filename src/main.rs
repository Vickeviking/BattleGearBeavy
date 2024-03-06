
use bevy::prelude::*;
use bevy_simple_text_input::TextInputPlugin;
use bevy_round_ui::prelude::*;

mod login;
mod dashboard;
mod register;
mod systems;
mod components;

use login::LoginPlugin;
use dashboard::DashboardPlugin;
use register::RegisterPlugin;
use systems::load_assets;
use systems::spawn_camera;

const API_URL: &str = "http://192.168.0.215:8000";

fn main() {
    App::new()
        // bevy plugins
        .add_plugins((DefaultPlugins, RoundUiPlugin, TextInputPlugin))
        .init_state::<AppState>()
        // my plugins
        .add_plugins((LoginPlugin, DashboardPlugin, RegisterPlugin))
        // startup systems
        .add_systems(Startup, (spawn_camera, load_assets))
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Login,
    Dashboard,
    Register,
}


