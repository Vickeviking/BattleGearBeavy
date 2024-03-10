use bevy::prelude::*;
use crate::dashboard::components::*;


pub fn spawn_dashboard(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let dashboard_entity = build_dashboard(&mut commands, &asset_server, meshes, materials);
}

pub fn despawn_dashboard(mut commands: Commands, dashboard_query: Query<Entity, With<Dashboard>>) {
    if let Ok(dashboard_entity) = dashboard_query.get_single() {
        commands.entity(dashboard_entity).despawn_recursive(); // recursive to despawn all child ui nodes
    }
}


pub fn build_dashboard(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) -> Entity {
    let dashboard_entity = commands.spawn((Dashboard{}, TransformBundle::default(), VisibilityBundle::default())).id();

    // island 
    let mesh = meshes.add(Circle::new(4.0));
    let material = materials.add(Color::WHITE);
    let island_entity = commands.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..Default::default()
    }).id();

    // cube
    let box_mesh_handle = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let box_material_handle = materials.add(Color::rgb_u8(124, 144, 255));

    let cube_entity = commands.spawn(PbrBundle {
        mesh: box_mesh_handle,
        material: box_material_handle,
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    }).id();

    // lights 
    let point_light = PointLight {
        intensity: 200000.0,
        shadows_enabled: true,
        range: 20.0,
        ..Default::default()
    };
    let point_light_bundle = PointLightBundle {
        point_light,
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    };
    let light_animator=  LightAnimator 
        {
            light_max: 4000.0 * 1000.0,
            light_min: 200.0 * 1000.0,
            light_speed: 2000.0,
            light_value: 200.0 * 1000.0,
            light_direction: 1.0,
        };

    let point_light_entity = commands.spawn((point_light_bundle, light_animator))
    .id();


    commands.entity(dashboard_entity).push_children(&[cube_entity, island_entity, point_light_entity]);

    dashboard_entity
}