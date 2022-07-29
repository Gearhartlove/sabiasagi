use bevy::prelude::*;
use bevy::prelude::Name as EntityName;
use battle_plugin::{Allegiance, Fighter};
use crate::{BOLD_FONT_PATH, BORDER_SCALE, FONT_PATH, HP_NUMBER_FONT_SIZE, HP_WORD_FONT_SIZE, LEFT_BORDER_ARROW_TRANSFORM, LEFT_FIGHTER_TRANSFORM, LEVEL_FONT_SIZE, NAME_FONT_SIZE, RIGHT_BORDER_ARROW_TRANSFORM, RIGHT_FIGHTER_TRANSFORM, SPRITE_SIZE, UI_LEVEL};


#[derive(Component)]
struct CustomButton;

// not using in app, saving in code for reference
// fn button_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands
//         .spawn_bundle(ButtonBundle {
//             style: Style {
//                 size: Size::new(Val::Px(200.), Val::Px(40.)),
//                 //center button
//                 margin: Rect {
//                     bottom: Val::Px(70.),
//                     left: Val::Px(310.),
//                     ..default()
//                 },
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 ..default()
//             },
//             color: bevy::prelude::UiColor(Color::GRAY),
//             ..default()
//         })
//         .insert(CustomButton)
//         .with_children(|parent| {
//             parent.spawn_bundle(TextBundle {
//                 text: Text::with_section(
//                     "Randomize",
//                     TextStyle {
//                         font: asset_server.load(FONT_PATH.clone()),
//                         font_size: 40.,
//                         color: Color::BLACK,
//                     },
//                     Default::default(),
//                 ),
//                 ..default()
//             });
//         });
// }

// not using in app, saving in code for reference
// fn button_style_system(mut interaction_query: Query<(&Interaction, &mut UiColor, &Children),
//     (Changed<Interaction>, With<CustomButton>)>, mut text_query: Query<&mut Text>) {
//     for (interaction, mut color, children) in interaction_query.iter_mut() {
//         let mut text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Clicked => {
//                 *color = bevy::prelude::UiColor(Color::LIME_GREEN).into();
//             }
//             Interaction::Hovered => {
//                 *color = bevy::prelude::UiColor(Color::SEA_GREEN).into();
//             }
//             Interaction::None => {
//                 *color = bevy::prelude::UiColor(Color::GRAY).into();
//             }
//         }
//     }
// }

/// Struct used as a component included as apart of Bevy Components which pertain to the fighter UI.
/// The primary use case clears all of the assets off the screan in "randomize arena" by looking
/// at every entity with the PokemonUI.
#[derive(Component)]
pub struct PokemonUI;

/// Spawns the sprite, name, level, health bar, and health bar number of the fighter
pub fn spawn_pokemon_ui(commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Static Assets
    spawn_pokemon_sprite(commands, &asset_server, &fighter);
    spawn_pokemon_name(commands, &asset_server, &fighter);
    spawn_pokemon_level(commands, &asset_server, &fighter);
    spawn_health_bar_text(commands, &asset_server, &fighter);
    spawn_border(commands, &asset_server, &fighter);
    // Dynamic Assets (would change in battle)
    spawn_health_bar(commands, &asset_server, &fighter);
    spawn_health_bar_number(commands, &asset_server, &fighter);
    // . . .
}

fn spawn_border(commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    let mut spawn_sprite = |transform: Transform, id: String| {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                // custom_size: Some(Vec2::splat(BORDER_SCALE.clone())),-
                // can have different sized sprites as long as their original size is the same AND
                // they are scaled by the same amount
                // custom_size: Some(Vec2::new(org_x * sprite_scale, org_y * sprite_scale),
                // todo: refactor code to save size as variable, instead of hard coding it as a constant
                custom_size: Some(Vec2::new(32. * 5., 48. * 5.)),
                ..default()
            },
            transform: transform.clone(),
            texture: asset_server.get_handle(id.as_str()).clone(),
            ..default()
        })
            .insert(PokemonUI)
            .insert(EntityName::from("border arrow"));
    };
    match fighter.allegiance.as_ref().unwrap() {
        Allegiance::Ally => {
            let id: String = format!("sprites/ui/dbg1.png");
            spawn_sprite(RIGHT_BORDER_ARROW_TRANSFORM, id);
        }
        Allegiance::Enemy => {
            let id: String = format!("sprites/front_sprites/right_facing_arrow.png");
            spawn_sprite(LEFT_BORDER_ARROW_TRANSFORM, id);
        }
    }
}

// POKEMON SPECIFIC UI
fn spawn_pokemon_name(commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
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
        })
            .insert(PokemonUI);
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

fn spawn_pokemon_sprite(commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    let mut spawn_sprite = |transform: Transform, id: String| {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SPRITE_SIZE.clone())),
                ..default()
            },
            transform: transform.clone(),
            texture: asset_server.get_handle(id.as_str()).clone(),
            ..default()
        })
            .insert(PokemonUI);
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

fn spawn_pokemon_level(commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
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
        })
            .insert(PokemonUI);
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

/// Struct used for debugging
#[derive(Component)]
struct HpBar;

fn spawn_health_bar(commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
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
            .insert(HpBar)
            .insert(PokemonUI);
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

/// Struct used for debugging
#[derive(Component)]
struct HpText;

fn spawn_health_bar_text(commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
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

/// Struct used for debugging
#[derive(Component)]
struct HpNumber;

fn spawn_health_bar_number(commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
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
            .insert(HpNumber)
            .insert(PokemonUI);
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
