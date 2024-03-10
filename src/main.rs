
use bevy::prelude::*;
use bevy_simple_text_input::TextInputPlugin;
use bevy_round_ui::prelude::*;

mod login;
mod dashboard;
mod register;
mod systems;
mod components;
mod api;

use login::LoginPlugin;
use dashboard::DashboardPlugin;
use register::RegisterPlugin;
use systems::*;

const API_URL_INTERNAL: &str = "http://192.168.0.215:8000";
const API_URL: &str = "http://battlegear.ddns.net:81";
const DEBUG: bool = true;

// cache key for the login token
#[derive(Resource)]
struct CacheKey(String);

fn main() {
    App::new()
        // bevy plugins
        .add_plugins((DefaultPlugins, RoundUiPlugin, TextInputPlugin))
        .init_state::<AppState>()
        // my plugins
        .add_plugins((LoginPlugin, DashboardPlugin, RegisterPlugin))
        // startup systems
        .add_systems(PreStartup, (spawn_2dcamera, load_assets))
        .insert_resource(CacheKey("".to_string()))
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Login,
    Dashboard,
    Register,
}


