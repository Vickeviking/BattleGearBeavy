mod components;
mod styles;
mod systems;

use bevy::prelude::*;
use systems::layout::*;
use crate::AppState;
use crate::systems::*;

use self::systems::animations::animate_light_system;

pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Dashboard), (switch_to_3d_camera,spawn_dashboard).chain())
            .add_systems(Update, animate_light_system)
            .add_systems(OnExit(AppState::Dashboard), despawn_dashboard);
    }
}

