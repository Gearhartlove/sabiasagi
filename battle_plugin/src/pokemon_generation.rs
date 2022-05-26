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
use std::io::Read;
use std::process::exit;
use bevy::math::i32;
use bevy::utils::HashMap;
use crate::{Fighter, Pokemon};
use rand::Rng;
use scraper::{Html, Selector};

pub fn generator_driver() {
    let mut fighter_map: HashMap<String, Fighter> = HashMap::default();
    parse_pokedex(&mut fighter_map);
    // q: do I want to put images on fighters?, then pass fighter_map as a resource?
    // called: Web Scraping for Images > I don't have to use rust ...
    scrape_pokemon_web_images();
    // look into web api ? and then grabbing the sprites based on the names

}

fn parse_pokedex(mut fighter_map: &mut HashMap<String, Fighter>) {
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

         fn get_random_level() -> i32 {
            let mut rng = rand::thread_rng();
             let number = rng.gen_range(1..101);
             number
         }

        let fighter: Fighter = Fighter {
            name: name.clone(),
            total_hit_points: health.parse::<f32>().unwrap(),
            current_hit_points: health.parse::<f32>().unwrap(),
            level: get_random_level(), // todo: fix this if this is a problem . . .
            allegiance: None
        };

        fighter_map.insert(name, fighter);

        let num = number.parse::<usize>().unwrap();
        if num > GEN1_STOP {
            // Note: be careful with exits, as it will stop the entire program :)
            break
        }
    }

    // logic break -------------------------------
    // for pair in fighter_map.into_iter() {
    //     println!("{:?}", pair);
    // }

}

// Note: deciding not to get html using reqwest or other libraries because of async problems . . .
// outside the scope of what I need to do
const SPRITES_BASE_URL: &str = "https://www.pokencyclopedia.info";
fn scrape_pokemon_web_images()
    -> Result<(), reqwest::Error>{
    let back_file_path = "assets/pokemon_back_sprites.html";
    let front_file_path = "assets/pokemon_front_sprites.html";
    let html = fs::read_to_string(back_file_path)
        .expect(format!("The file {} does not exist", back_file_path).as_str());

    let fragment = Html::parse_fragment(html.as_str()) ;
    let selector = Selector::parse("img").unwrap();

    for element in fragment.select(&selector) {
        let src = element.value().attr("src").unwrap();
        if src.contains("../sprites") {
            let alt = element.value().attr("alt").unwrap();
            let number = &alt[..4];
            let name = &alt[5..];
            // get rid of first two ".." from src file path
            let src = &src[2..];
            // construct url to internet image
            let url = format!("{}{}", SPRITES_BASE_URL.clone(), src.clone());

            // get image from web
            let img_bytes = reqwest::blocking::get(url)?.bytes()?;
            let image = image::load_from_memory(&img_bytes);
            let save_path = format!("assets/sprites/back_sprites/{}_back.png", name.clone());
            image.unwrap().save(save_path);
            println!("saved: {} {}", number, name);
        }
    }

    Ok(())
}
