use bevy::prelude::Transform;

pub const WINDOW_WIDTH: f32 = 512.;
pub const WINDOW_HEIGHT: f32 = 384.;
pub const SPRITE_SIZE: f32 = 32. * 5.;

const arbitrary_y_val: f32 = 100.;
const arbitrary_x_val: f32 = 24.;

pub const RIGHT_FIGHTER_TRANSFORM: Transform =
    Transform::from_xyz((WINDOW_WIDTH/2. * -1. + SPRITE_SIZE/2. + arbitrary_x_val), (WINDOW_HEIGHT/2. * -1. + SPRITE_SIZE/2. + arbitrary_y_val), 1.);
pub const LEFT_FIGHTER_TRANSFORM: Transform =
    Transform::from_xyz((WINDOW_WIDTH/2. - SPRITE_SIZE/2. - arbitrary_x_val) , (WINDOW_HEIGHT/2. - SPRITE_SIZE/2.), 1.);
