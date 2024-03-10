use bevy::prelude::*;

#[derive(Component)]
pub struct Dashboard {}


#[derive(Component)]
pub struct LightAnimator {
    pub light_max: f32,
    pub light_min: f32,
    pub light_value: f32,
    pub light_direction: f32,
    pub light_speed: f32,
}