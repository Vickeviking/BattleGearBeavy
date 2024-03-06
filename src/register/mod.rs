mod components;
mod styles;
mod systems;

use bevy::prelude::*;
use systems::layout::*;
use systems::interactions::*;

use crate::AppState;


pub struct RegisterPlugin;

impl Plugin for RegisterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Register), (spawn_register, setup_text_input).chain())
            .add_systems(
                Update, 
                (
                    listener, 
                    focus, 
                    on_submit,
                    interact_with_go_to_login_button,
                    interact_with_go_to_register_button,
                ))
            .add_systems(OnExit(AppState::Register), despawn_register);
    }
}