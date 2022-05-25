mod constants;

use constants::*;
use bevy::prelude::*;
use battle_plugin::*;
use battle_plugin::pokemon_roster::Pokemon;
use crate::StartupStage::PreStartup;
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use crate::AlignContent::Center;

// reference : https://www.youtube.com/watch?v=s_4zaj8EbFI&t=757s

// IDEA: press a button to generate a different match up with
// different levels and everything :)

fn main() {
    let mut app = App::new();
    app
        // todo: register fighter as inspectable
        // .register_inspectable::<Fighter>()
        .insert_resource(WindowDescriptor {
            title: "konkuRRenz".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<Fighter>::new())
        .add_startup_system_to_stage(PreStartup, setup_assets)
        .add_startup_system(setup_arena)
        .add_startup_system(camera_setup)
        .add_startup_system(ui_camera_setup)
        // .add_system(debug_fighters)
        .run();
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

fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // note: path starts from the /assets server
    commands.insert_resource(ArenaAssets {
        left_fighter_sprite: asset_server.load("sprites/charmeleon_sprite.png"),
        right_figher_sprite: asset_server.load("sprites/weedle_sprite.png"),
        // arena:
    });
}

// todo: think about how i want to connect all of the ui elements to the fighters
//  could append the fighter to each tag, or make a p1 and p2 component and append it likewise,
//  or ???
fn setup_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    // create fighters
    let weedle = Fighter::new(
        Pokemon::Weedle, 25., 6, Allegiance::Enemy,
    );
    let charmeleon = Fighter::new(
        Pokemon::Charmeleon, 145., 25, Allegiance::Ally,
    );

    // spawn ui
    spawn_pokemon_ui(&mut commands, &asset_server, &weedle);
    spawn_pokemon_ui(&mut commands, &asset_server, &charmeleon);
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
            let vir_adj: Val = Val::Px(205.);
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
    let id: String = format!("sprites/{}_sprite.png", fighter.name.to_lowercase().clone());
    println!("id: {}", id);
    let mut spawn_sprite = |transform: Transform| {
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
            spawn_sprite(RIGHT_FIGHTER_TRANSFORM);
        }
        Allegiance::Enemy => {
            spawn_sprite(LEFT_FIGHTER_TRANSFORM);
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
                bottom: Val::Px(180.),
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
            let translation = Vec3::new(128.0, -28.5, 0.);
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
                bottom: Val::Px(153.),
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