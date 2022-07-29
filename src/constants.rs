use bevy::prelude::Transform;

/// constants used throughout the project. Includes window and sprite + font size information

// Window information
pub const WINDOW_WIDTH: f32 = 512.;
pub const WINDOW_HEIGHT: f32 = 384.;

// UI Related
pub const UI_LEVEL: f32 = 100.;
pub const SPRITE_SIZE: f32 = 32. * 5.;
pub const FONT_PATH: &str = "fonts/JetBrainsMono-SemiBold.ttf";
pub const BOLD_FONT_PATH: &str = "fonts/JetBrainsMono-ExtraBold.ttf";
pub const TEXT_OFFSET: f32 = 20.;
pub const NAME_FONT_SIZE: f32 = 40.;
pub const LEVEL_FONT_SIZE: f32 = 30.;
pub const HP_WORD_FONT_SIZE: f32 = 26.;
pub const HP_NUMBER_FONT_SIZE: f32 = 26.;

const ARBITRARY_Y_VAL: f32 = 100.;
const ARBITRARY_X_VAL: f32 = 30.;

pub const RIGHT_FIGHTER_TRANSFORM: Transform =
    Transform::from_xyz((WINDOW_WIDTH/2. * -1. + SPRITE_SIZE/2. + ARBITRARY_X_VAL), (WINDOW_HEIGHT/2. * -1. + SPRITE_SIZE/2. + ARBITRARY_Y_VAL), 1.);
pub const LEFT_FIGHTER_TRANSFORM: Transform =
    Transform::from_xyz((WINDOW_WIDTH/2. - SPRITE_SIZE/2. - ARBITRARY_X_VAL), (WINDOW_HEIGHT/2. - SPRITE_SIZE/2.), 1.);

pub const BORDER_SCALE: f32 = 32. * 5.;
pub const RIGHT_BORDER_ARROW_TRANSFORM: Transform =
    Transform::from_xyz((WINDOW_WIDTH/2. * -1. + SPRITE_SIZE/2. + ARBITRARY_X_VAL), (WINDOW_HEIGHT/2. * -1. + SPRITE_SIZE/2. + ARBITRARY_Y_VAL), 2.);
pub const LEFT_BORDER_ARROW_TRANSFORM: Transform =
    Transform::from_xyz((WINDOW_WIDTH/2. - SPRITE_SIZE/2. - ARBITRARY_X_VAL), (WINDOW_HEIGHT/2. - SPRITE_SIZE/2.), 2.);

// todo: understand UI scaling --> get each pixel to be the same size