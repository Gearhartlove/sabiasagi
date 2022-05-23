use bevy::prelude::Transform;

pub const WINDOW_WIDTH: f32 = 512.;
pub const WINDOW_HEIGHT: f32 = 384.;
pub const SPRITE_SIZE: f32 = 160.;
pub const RIGHT_FIGHTER_TRANSFORM: Transform =
    Transform::from_xyz((WINDOW_WIDTH/2. * -1. + SPRITE_SIZE/2.), (WINDOW_HEIGHT/2. * -1. + SPRITE_SIZE/2.), 1.);
pub const LEFT_FIGHTER_TRANSFORM: Transform =
    Transform::from_xyz((WINDOW_WIDTH/2. - SPRITE_SIZE/2.), (WINDOW_HEIGHT/2. - SPRITE_SIZE/2.), 1.);
