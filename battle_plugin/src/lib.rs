pub mod pokemon_roster;
mod pokemon_generation;

use bevy::prelude::*;
use crate::pokemon_roster::Pokemon;
use bevy_inspector_egui::{Inspectable};

#[derive(Debug, Inspectable)]
pub enum Allegiance {
    Ally,
    Enemy,
}

// pub fn create_fighter(){}
//coulddo: make fighter a trait and then require that certain
//values are returned (would work with custom fighters)
// #[derive(Component, Inspectable, Debug)]
#[derive(Component, Debug, Inspectable)]
pub struct Fighter{
    pub name: String,
    pub total_hit_points: f32,
    pub current_hit_points: f32,
    pub level: i32,
    pub allegiance: Option<Allegiance>,
    // ability: Box<Vec<Ability>>,
}



impl Default for Fighter {
    fn default() -> Self {
        Fighter {
            name: "default_name".to_string(),
            total_hit_points: -1.,
            current_hit_points: -1.,
            level: 1000,
            allegiance: None,
        }
    }
}

impl Fighter {
    pub fn new(pokemon: Pokemon, hit_points: f32, level: i32, allegiance: Allegiance) -> Self {
        Fighter {
            name: pokemon.to_string(),
            total_hit_points: hit_points.clone(),
            current_hit_points: hit_points.clone(),
            level,
            allegiance: Some(allegiance),
        }
    }
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

// #[derive(Inspectable)]
pub struct Arena {
    fighter_left: Fighter,
    fighter_right: Fighter,
    background: Sprite,
}

pub struct ArenaAssets {
    pub left_fighter_sprite: Handle<Image>,
    pub right_figher_sprite: Handle<Image>,
    // couldo: retro theme on top of the whole project -> imagine this has to do with shaders . . .
    // pub arena: Handle<Image>, todo: add arena background loading
}
