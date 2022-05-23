use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, Inspectable};

// pub fn create_fighter(){}
//coulddo: make fighter a trait and then require that certain
//values are returned (would work with custom fighters)
#[derive(Component, Inspectable)]
pub struct Fighter{
    pub name: String,
    pub hit_points: f32,
    pub level: i32,
    // ability: Box<Vec<Ability>>,
}

struct Ability {
    target: Fighter,
    effect: Effect,
}
// coulddo: expand on this effect to make it
// expressive with what is happening in the game,
// but for now I can simply just define effects
// that interact with rock, paper, scissors
struct Effect{}

#[derive(Inspectable)]
pub struct Arena {
    fighter_left: Fighter,
    fighter_right: Fighter,
    background: Sprite,
}

pub struct ArenaAssets {
    pub left_fighter_sprite: Handle<Image>,
    pub right_figher_sprite: Handle<Image>,
    // pub arena: Handle<Image>, todo: add arena background loading
}

#[derive(Inspectable, Default)]
struct Data {
    should_render: bool,
    text: String,
    #[inspectable(min = 42.0, max = 100.0)]
    size: f32,
}
