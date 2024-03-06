mod components;
mod styles;
mod systems;

use bevy::prelude::*;
use systems::layout::*;
use crate::AppState;

pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Dashboard), spawn_dashboard)
            .add_systems(OnExit(AppState::Dashboard), despawn_dashboard);
    }
}