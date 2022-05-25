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
use std::process::exit;
use bevy::math::i32;

pub fn parse_pokedex() {
    let file_path = "assets/data/Pokedex.csv";
    let contents = fs::read_to_string(file_path)
        .expect(format!("The file {} does not exist", file_path).as_str());
    // println!("With text: \n{}", contents);

    // handle the contents
    // info index
    const GEN1_STOP: usize = 151;
    const NUM_INDEX: usize = 1;
    const NAME_INDEX: usize = 2;
    const HEALTH_INDEX: usize = 3;
    let split_contents: Vec<&str> = contents.split("\n").skip(1).collect();
    for line in split_contents {
        // Note: I can collect after a split to get a vector!
        let split_line: Vec<String> = line.split(",").map(
            |s| -> String {
                s.to_string()
            })
            .collect();
        let number: String = split_line[NUM_INDEX].clone();
        let name: String = split_line[NAME_INDEX].clone();
        let health: String = split_line[HEALTH_INDEX].clone();//.parse::<i32>().unwrap();
        // let health: String = vec_split_line.get(NAME_INDEX).unwrap());
        // let health: String = split_line(HEALTH_INDEX).unwrap();

        let num = number.parse::<usize>().unwrap();
        if num > GEN1_STOP {
            exit(111);
        }
        else {
            println!("number: {} name: {} health: {}", number, name, health);
        }
    }
}

fn download_sprites() {

}