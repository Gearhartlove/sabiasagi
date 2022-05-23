mod constants;
use constants::*;
use bevy::prelude::*;
use battle_plugin::*;
use battle_plugin::pokemon_roster::Pokemon;
use crate::StartupStage::PreStartup;

// reference : https://www.youtube.com/watch?v=s_4zaj8EbFI&t=757s

fn main() {
    let mut app = App::new();
    app
        .insert_resource(WindowDescriptor {
            title: "konkuRRenz".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        // .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(PreStartup, setup_assets)
        .add_startup_system(setup_arena)
        .add_startup_system(camera_setup)
        .add_startup_system(ui_camera_setup)
        // .add_system(debug_fighters)
        .add_startup_system(debug_UI)
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

fn debug_UI(query: Query<&Text>) {
    for text in query.iter() {
         println!("{:?}", text);
    }
}

fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // note: path starts from the /assets server
    commands.insert_resource(ArenaAssets {
        left_fighter_sprite: asset_server.load("sprites/charmeleon_sprite.png"),
        right_figher_sprite:  asset_server.load("sprites/weedle_sprite.png"),
        // arena:
    });
}
fn setup_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weedle = Fighter::new(
        Pokemon::Weedle, 25., 70
    );
    let charmeleon = Fighter::new(
        Pokemon::Charmeleon, 25., 70
    );

    // CREATE  FIGHTERS -------------------------------------------------------

    // create weedle fighter
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(SPRITE_SIZE.clone())),
            color: Color::Rgba {red: 1., green: 1., blue: 1., alpha: 1.},
            ..default()
        },
        transform: LEFT_FIGHTER_TRANSFORM.clone(),
        texture: asset_server.get_handle("sprites/weedle_sprite.png").clone(),
        ..default()
    })
        .insert(weedle);
    // create charmeleon fighter
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(SPRITE_SIZE.clone())),
            color: Color::Rgba {red: 1., green: 1., blue: 1., alpha: 1.},
            ..default()
        },
        transform: RIGHT_FIGHTER_TRANSFORM.clone(),
        texture: asset_server.get_handle("sprites/charmeleon_sprite.png").clone(),
        ..default()
    })
        .insert(charmeleon);

    // CREATE HEALTH, NAME, LEVEL -------------------------------------------------------
    // Note, I will not follow the picture exactly, I will have an above and
    // below variant to put the text into
    // todo: above or below variant
    // todo: think about how I want to connect all of the UI elements to the fighters
    //  could append the fighter to each tag, or make a p1 and p2 component and append it likewise,
    //  or ???
    // NAME
    // left text
    println!("spawning_text");
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            margin: Rect::all(Val::Px(10.0)),
            ..default()
            },
        text: Text::with_section(
            "pokemon_name",
            TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 25.,
                color: Color::BLACK,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..default()
            },
        ),
        ..default()
    });
}
