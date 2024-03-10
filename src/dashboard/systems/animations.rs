use bevy::{prelude::*, utils::info};
use crate::dashboard::components::LightAnimator;

// on update animations


pub fn animate_light_system(
    mut query: Query<(&mut PointLight, &mut LightAnimator)>,
) {
    for (mut point_light, mut animator) in query.iter_mut() {
        animator.light_value += animator.light_speed * animator.light_direction;
        if animator.light_value > animator.light_max {
            animator.light_value = animator.light_max;
            animator.light_direction *= -1.0;
        } else if animator.light_value < animator.light_min {
            animator.light_value = animator.light_min;
            animator.light_direction *= -1.0;
        }
        point_light.intensity = animator.light_value;

    }

}