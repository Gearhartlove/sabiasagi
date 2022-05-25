// resource: https://www.pokencyclopedia.info/en/index.php?id=sprites/gen1
// spreadsheet: https://docs.google.com/spreadsheets/d/1DYnYR1bu1oRQ5ZMNwl5emfYzoYkr6P-JvULtYUTGtSg/edit#gid=0

// Consider each pokemon in Generation 1.
// Create a file in /battle_plugin/pokemon/pokemon_name
// file structure:
// /pokemon_name
// ---- name_back_sprite.png
// ---- name_front_sprite.png
// ---- stats
// ---- pokemon.rs // pub Fighter struct instantiation

// Connect the file to file_structure

// go to website and consider each image.
use std::env;
use std::fs;

pub fn parse_pokemon_spreadsheet() {
    let file_name = "Pokedex.csv";
    let contents = fs::read_to_string(file_name)
        .expect(format!("The file {} does not exist", file_name).as_str());
    println!("With text: \n{}", contents);
}

fn download_sprites() {

}