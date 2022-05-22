use bevy::prelude::*;

// pub fn create_fighter(){}
//coulddo: make fighter a trait and then require that certain
//values are returned (would work with custom fighters)
#[derive(Component)]
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
