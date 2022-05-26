mod constants;

use std::collections::HashMap;
use constants::*;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use battle_plugin::*;
use battle_plugin::pokemon_roster::Pokemon;
use crate::StartupStage::PreStartup;
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use crate::AlignContent::Center;
use rand::Rng;
use crate::CoreStage::First;

// reference : https://www.youtube.com/watch?v=s_4zaj8EbFI&t=757s

// IDEA: press a button to generate a different match up with
// different levels and everything :)

fn main() {
    let mut app = App::new();
    app
        .insert_resource(WindowDescriptor {
            title: "konkuRRenz".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .insert_resource(battle_plugin::generate_fighters_map())
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system_to_stage(PreStartup, setup_assets)
        .add_startup_system(setup_arena)
        .add_startup_system(camera_setup)
        .add_startup_system(ui_camera_setup)
        .add_startup_system(button_setup)
        .add_system(button_style_system)
        // .add_system(debug_fighters)
        .run();
}

// fn mouse_click_system(mouse_button_input: Res<Input<MouseButton>>) {
//     if mouse_button_input.pressed(MouseButton::Left) {
//         info!("left mouse currently pressed");
//     }
// }

#[derive(Component)]
struct CustomButton;

fn button_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.), Val::Px(40.)),
                //center button
                margin: Rect {
                    bottom: Val::Px(70.),
                    left: Val::Px(310.),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: bevy::prelude::UiColor(Color::GRAY),
            ..default()
        })
        .insert(CustomButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Randomize",
                    TextStyle {
                        font: asset_server.load(FONT_PATH.clone()),
                        font_size: 40.,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..default()
            });
        });

}

fn button_style_system(mut interaction_query: Query<(&Interaction, &mut UiColor, &Children),
    (Changed<Interaction>, With<CustomButton>)>, mut text_query: Query<&mut Text>) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = bevy::prelude::UiColor(Color::LIME_GREEN).into();

            }
            Interaction::Hovered => {
                *color = bevy::prelude::UiColor(Color::SEA_GREEN).into();
            }
            Interaction::None => {
                *color = bevy::prelude::UiColor(Color::GRAY).into();
            }
        }
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn ui_camera_setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn debug_fighters(query: Query<&Fighter>) {
    for fighter in query.iter() {
        println!("{:?}", fighter);
    }
}


fn spawn_random_new_fighters(fighter_map: Res<HashMap<i32, Fighter>>) -> (Fighter, Fighter) {
    let mut rng = rand::thread_rng();
    let ally_num = rng.gen_range(1..152);
    let enemy_num = rng.gen_range(1..152);

    let ally_fighter = fighter_map.get(&ally_num).cloned().unwrap().ally();
    let enemy_fighter = fighter_map.get(&enemy_num).cloned().unwrap().enemy();
    (ally_fighter, enemy_fighter)
}

fn setup_arena(mut commands: Commands, asset_server: Res<AssetServer>,
               fighter_map: Res<HashMap<i32, Fighter>>) {
    let (ally_fighter, enemy_fighter) = spawn_random_new_fighters(fighter_map);

    // explicit fighters, numbers to pokemon
    // couldo: write a converter . . .
    // let ally_figher = fighter_map.get(&ally_num).cloned().ally();
    // let enemy_fighter = fighter_map.get(&ally_num).cloned().ally();
    // spawn ui

    spawn_pokemon_ui(&mut commands, &asset_server, &ally_fighter);
    spawn_pokemon_ui(&mut commands, &asset_server, &enemy_fighter);
}


fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>, fighter_map: Res<HashMap<i32, Fighter>>) {
    for fighter in fighter_map.values() {
        let back_id: String = format!("sprites/back_sprites/{}_back.png", fighter.name.clone());
        let front_id: String = format!("sprites/front_sprites/{}_front.png", fighter.name.clone());
        let back_fighter_sprite: Handle<Image> = asset_server.load(&back_id);
        let front_fighter_sprite: Handle<Image> = asset_server.load(&front_id);
        commands.insert_resource(back_fighter_sprite);
        commands.insert_resource(front_fighter_sprite);
    }
}

fn spawn_pokemon_ui(mut commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    // Static Assets
    spawn_pokemon_sprite(&mut commands, &asset_server, &fighter);
    spawn_pokemon_name(&mut commands, &asset_server, &fighter);
    spawn_pokemon_level(&mut commands, &asset_server, &fighter);
    spawn_health_bar_text(&mut commands, &asset_server, &fighter);
    // Dynamic Assets
    spawn_health_bar(&mut commands, &asset_server, &fighter);
    spawn_health_bar_number(&mut commands, &asset_server, &fighter);
    // . . .
}

// POKEMON SPECIFIC UI
fn spawn_pokemon_name(mut commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    let mut spawn_ui = |position: Rect<Val>| {
        commands.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position,
                ..default()
            },
            // left pokemon name text
            text: Text::with_section(
                fighter.name.to_uppercase().clone(),
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: NAME_FONT_SIZE,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        });
    };
    match fighter.allegiance.as_ref().unwrap() {
        Allegiance::Enemy => {
            let hor_adj: Val = Val::Px(43.);
            let vir_adj: Val = Val::Px(205. - 15.);
            let pos = Rect {
                right: hor_adj,
                bottom: vir_adj,
                ..default()
            };
            spawn_ui(pos)
        }
        Allegiance::Ally => {
            let hor_adj: Val = Val::Px(30.);
            let vir_adj: Val = Val::Px(5.);
            let pos = Rect {
                left: hor_adj,
                top: vir_adj,
                ..default()
            };
            spawn_ui(pos)
        }
    }
}

fn spawn_pokemon_sprite(mut commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    let mut spawn_sprite = |transform: Transform, id: String| {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SPRITE_SIZE.clone())),
                ..default()
            },
            transform: transform.clone(),
            texture: asset_server.get_handle(id.as_str()).clone(),
            ..default()
        });
    };
    match fighter.allegiance.as_ref().unwrap() {
        Allegiance::Ally => {
            let id: String = format!("sprites/back_sprites/{}_back.png", fighter.name.clone());
            println!("{}", id);
            spawn_sprite(RIGHT_FIGHTER_TRANSFORM, id);
        }
        Allegiance::Enemy => {
            let id: String = format!("sprites/front_sprites/{}_front.png", fighter.name.clone());
            println!("{}", id);
            spawn_sprite(LEFT_FIGHTER_TRANSFORM, id);
        }
    }
}

fn spawn_pokemon_level(mut commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    let mut spawn_ui = |position: Rect<Val>| {
        commands.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position,
                ..default()
            },
            // left pokemon name text
            text: Text::with_section(
                format!(":L{}", fighter.level.to_string().clone()),
                TextStyle {
                    font: asset_server.load(BOLD_FONT_PATH),
                    font_size: LEVEL_FONT_SIZE,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        });
    };
    match fighter.allegiance.as_ref().unwrap() {
        Allegiance::Ally => {
            let pos = Rect {
                left: Val::Px(100.),
                top: Val::Px(40.),
                ..default()
            };
            spawn_ui(pos)
        }
        Allegiance::Enemy => {
            let pos = Rect {
                right: Val::Px(70.),
                bottom: Val::Px(165.),
                ..default()
            };
            spawn_ui(pos)
        }
    }
}

#[derive(Component)]
struct HpBar;

fn spawn_health_bar(mut commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    println!("health bar spawned");
    let mut spawn_health_bar = |translation: Vec3, scale: Vec3| {
        commands.spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(105, 152, 105),
                    ..default()
                },
                transform: Transform {
                    translation,
                    scale,
                    ..default()
                },
                ..default()
            }
        )
            .insert(HpBar);
    };
    match fighter.allegiance.as_ref().unwrap() {
        Allegiance::Ally => {
            let translation = Vec3::new(-108.8, 102.1, 0.);
            let scale = Vec3::new(168.2, 10.7, UI_LEVEL);
            spawn_health_bar(translation, scale);
        }
        Allegiance::Enemy => {
            let translation = Vec3::new(128.0, -28.5 - 15., 0.);
            let scale = Vec3::new(168.2, 10.7, UI_LEVEL);
            spawn_health_bar(translation, scale);
        }
    };
}

#[derive(Component)]
struct HpText;

fn spawn_health_bar_text(mut commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    let mut spawn_text = |position: Rect<Val>| {
        commands.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position,
                ..default()
            },
            // left pokemon name text
            text: Text::with_section(
                format!("HP:"),
                TextStyle {
                    font: asset_server.load(BOLD_FONT_PATH),
                    font_size: HP_WORD_FONT_SIZE,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
            .insert(HpText);
    };

    match fighter.allegiance.as_ref().unwrap() {
        Allegiance::Ally => {
            let pos = Rect {
                left: Val::Px(22.),
                top: Val::Px(75.),
                ..default()
            };
            spawn_text(pos)
        }
        Allegiance::Enemy => {
            let pos = Rect {
                right: Val::Px(218.),
                bottom: Val::Px(153. - 15.),
                ..default()
            };
            spawn_text(pos)
        }
    }
}

#[derive(Component)]
struct HpNumber;

fn spawn_health_bar_number(mut commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    let mut spawn_text = |position: Rect<Val>| {
        commands.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position,
                ..default()
            },
            // left pokemon name text
            text: Text::with_section(
                format!("{}/{}", fighter.current_hit_points.clone().round(), fighter.total_hit_points.clone().round()),
                TextStyle {
                    font: asset_server.load(BOLD_FONT_PATH),
                    font_size: HP_NUMBER_FONT_SIZE,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
            .insert(HpNumber);
    };

    match fighter.allegiance.as_ref().unwrap() {
        Allegiance::Ally => {
            let pos = Rect {
                left: Val::Px(64.),
                top: Val::Px(100.),
                ..default()
            };
            spawn_text(pos)
        }
        // Don't spawn health info for enemy
        Allegiance::Enemy => {}
    }
}
// couldo : implement the non-pokemon specific ui
// NON-POKEMON SPECIFIC UI
// fn spawn_border_arrow(mut commands: &mut commands, asset_server: &res<AssetServer>, fighter: &fighter) {}
// fn spawn_lower_menus(mut commands: &mut commands, asset_server: &res<AssetServer>, fighter: &fighter) {}