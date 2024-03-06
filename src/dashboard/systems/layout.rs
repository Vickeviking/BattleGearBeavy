use bevy::prelude::*;
use crate::dashboard::components::Dashboard;

pub fn spawn_dashboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let dashboard_entity = build_dashboard(&mut commands, &asset_server);
}

pub fn despawn_dashboard(mut commands: Commands, dashboard_query: Query<Entity, With<Dashboard>>) {
    if let Ok(dashboard_entity) = dashboard_query.get_single() {
        commands.entity(dashboard_entity).despawn_recursive(); // recursive to despawn all child ui nodes
    }
}


// not a system, a rust function that 
pub fn build_dashboard(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let dashboard_entity = commands.spawn(
    (
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::GREEN.into(),
                    ..default()
                },
                Dashboard{}, // mark this entity as a dashboard entity
            )
        )
        .id();
    dashboard_entity
}