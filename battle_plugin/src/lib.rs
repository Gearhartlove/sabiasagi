mod pokemon_generation;
use std::collections::HashMap;
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable};

#[derive(Debug, Inspectable, Clone)]
pub enum Allegiance {
    Ally,
    Enemy,
}

/// Primary logic for what a fighter can have and cannot have in the arena.
#[derive(Component, Debug, Inspectable, Clone)]
pub struct Fighter{
    pub name: String,
    pub total_hit_points: f32,
    pub current_hit_points: f32,
    pub level: i32,
    pub allegiance: Option<Allegiance>,
}

impl Fighter {
    /// Make a fighter an ally.
    pub fn ally(mut self) -> Self {
        self.allegiance = Some(Allegiance::Ally);
        self
    }

    /// Make a fighter an enemy.
    pub fn enemy(mut self) -> Self {
        self.allegiance = Some(Allegiance::Enemy);
        self
    }

}

pub fn generate_fighters_map() -> HashMap<i32, Fighter> {
    pokemon_generation::generator_driver()
}
