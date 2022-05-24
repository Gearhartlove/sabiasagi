use bevy::prelude::Transform;

pub const WINDOW_WIDTH: f32 = 512.;
pub const WINDOW_HEIGHT: f32 = 384.;
pub const SPRITE_SIZE: f32 = 32. * 5.;

// UI Related
pub const FONT_PATH: &str = "fonts/JetBrainsMono-SemiBold.ttf";
pub const TEXT_OFFSET: f32 = 20.;
pub const FONT_SIZE: f32 = 40.;

const ARBITRARY_Y_VAL: f32 = 100.;
const ARBITRARY_X_VAL: f32 = 30.;

pub const RIGHT_FIGHTER_TRANSFORM: Transform =
    Transform::from_xyz((WINDOW_WIDTH/2. * -1. + SPRITE_SIZE/2. + ARBITRARY_X_VAL), (WINDOW_HEIGHT/2. * -1. + SPRITE_SIZE/2. + ARBITRARY_Y_VAL), 1.);
pub const LEFT_FIGHTER_TRANSFORM: Transform =
    Transform::from_xyz((WINDOW_WIDTH/2. - SPRITE_SIZE/2. - ARBITRARY_X_VAL), (WINDOW_HEIGHT/2. - SPRITE_SIZE/2.), 1.);
