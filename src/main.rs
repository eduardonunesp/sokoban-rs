use constants::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use hecs::World;
use macroquad::audio::{self, Sound};
use macroquad::conf::UpdateTrigger;
use macroquad::prelude::*;
use std::collections::HashMap;

mod components;
mod constants;
mod entities;
mod events;
mod map;
mod systems;

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: macroquad::prelude::Conf {
            window_title: WINDOW_TITLE.to_string(),
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
            high_dpi: false,
            fullscreen: false,
            sample_count: 1,
            window_resizable: false,
            icon: None,
            ..Default::default()
        },
        update_on: Some(UpdateTrigger::default()),
        default_filter_mode: FilterMode::Nearest,
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture_atlas = make_texture_atlas().await;
    let sounds_atlas = make_sound_atlas().await;
    let mut world = World::new();

    entities::create_gameplay(&mut world);
    entities::create_time(&mut world);
    entities::create_event_queue(&mut world);
    entities::create_audio_store(&mut world);

    map::initialize_level(&mut world, texture_atlas.clone(), sounds_atlas.clone());

    loop {
        systems::input::run_input(&world);
        systems::gameplay::run_gameplay_state(&world);
        systems::events::run_process_events(&mut world);
        systems::rendering::run_rendering(&world);

        next_frame().await;
    }
}

async fn make_texture_atlas() -> HashMap<String, Texture2D> {
    let wall: Texture2D = load_texture("resources/images/wall.png")
        .await
        .expect("Failed to load texture.");
    let floor: Texture2D = load_texture("resources/images/floor.png")
        .await
        .expect("Failed to load texture.");
    let texture_box_blue_1: Texture2D = load_texture("resources/images/box_blue_1.png")
        .await
        .expect("Failed to load texture.");
    let texture_box_blue_2: Texture2D = load_texture("resources/images/box_blue_2.png")
        .await
        .expect("Failed to load texture.");
    let texture_box_red_1: Texture2D = load_texture("resources/images/box_red_1.png")
        .await
        .expect("Failed to load texture.");
    let texture_box_red_2: Texture2D = load_texture("resources/images/box_red_2.png")
        .await
        .expect("Failed to load texture.");
    let texture_player_1: Texture2D = load_texture("resources/images/player_1.png")
        .await
        .expect("Failed to load texture.");
    let texture_player_2: Texture2D = load_texture("resources/images/player_2.png")
        .await
        .expect("Failed to load texture.");
    let texture_player_3: Texture2D = load_texture("resources/images/player_3.png")
        .await
        .expect("Failed to load texture.");
    let texture_box_spot_blue: Texture2D = load_texture("resources/images/box_spot_blue.png")
        .await
        .expect("Failed to load texture.");
    let texture_box_spot_red: Texture2D = load_texture("resources/images/box_spot_red.png")
        .await
        .expect("Failed to load texture.");
    let texture_atlas = HashMap::from([
        (String::from("box_blue_1"), texture_box_blue_1),
        (String::from("box_blue_2"), texture_box_blue_2),
        (String::from("box_red_1"), texture_box_red_1),
        (String::from("box_red_2"), texture_box_red_2),
        (String::from("box_spot_blue"), texture_box_spot_blue),
        (String::from("box_spot_red"), texture_box_spot_red),
        (String::from("floor"), floor),
        (String::from("player_1"), texture_player_1),
        (String::from("player_2"), texture_player_2),
        (String::from("player_3"), texture_player_3),
        (String::from("wall"), wall),
    ]);
    build_textures_atlas();
    return texture_atlas;
}

async fn make_sound_atlas() -> HashMap<String, Sound> {
    set_pc_assets_folder("resources");
    let correct = audio::load_sound("sounds/correct.wav").await.unwrap();
    let incorrect = audio::load_sound("sounds/incorrect.wav").await.unwrap();
    let wall = audio::load_sound("sounds/wall.wav").await.unwrap();
    let sound_atlas = HashMap::from([
        (String::from("correct"), correct),
        (String::from("incorrect"), incorrect),
        (String::from("wall"), wall),
    ]);
    return sound_atlas;
}
