use constants::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use hecs::World;
use macroquad::audio::{self, Sound};
use macroquad::conf::UpdateTrigger;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::env;

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
    set_pc_assets_folder(
        std::path::Path::new(&env::current_dir().unwrap())
            .join("resources")
            .to_str()
            .unwrap(),
    );

    let texture_atlas = make_texture_atlas().await;
    let sounds_atlas = make_sound_atlas().await;
    let mut world = World::new();

    entities::create_gameplay(&mut world);
    entities::create_time(&mut world);
    entities::create_event_queue(&mut world);
    entities::create_audio_store(&mut world);

    map::initialize_level(&mut world, &texture_atlas, &sounds_atlas);

    loop {
        systems::input::run_input(&world);
        systems::gameplay::run_gameplay_state(&world);
        systems::events::run_process_events(&mut world);
        systems::rendering::run_rendering(&world);

        let mut query = world.query::<&mut crate::components::Time>();
        let time = query.iter().next().unwrap().1;
        time.delta += std::time::Duration::from_secs_f32(get_frame_time());

        next_frame().await;
    }
}

async fn make_texture_atlas() -> HashMap<String, Texture2D> {
    let texture_atlas = HashMap::from([
        (
            String::from("box_blue_1"),
            load_texture("images/box_blue_1.png")
                .await
                .expect("Failed to load texture."),
        ),
        (
            String::from("box_blue_2"),
            load_texture("images/box_blue_2.png")
                .await
                .expect("Failed to load texture."),
        ),
        (
            String::from("box_red_1"),
            load_texture("images/box_red_1.png")
                .await
                .expect("Failed to load texture."),
        ),
        (
            String::from("box_red_2"),
            load_texture("images/box_red_2.png")
                .await
                .expect("Failed to load texture."),
        ),
        (
            String::from("box_spot_blue"),
            load_texture("images/box_spot_blue.png")
                .await
                .expect("Failed to load texture."),
        ),
        (
            String::from("box_spot_red"),
            load_texture("images/box_spot_red.png")
                .await
                .expect("Failed to load texture."),
        ),
        (
            String::from("floor"),
            load_texture("images/floor.png")
                .await
                .expect("Failed to load texture."),
        ),
        (
            String::from("player_1"),
            load_texture("images/player_1.png")
                .await
                .expect("Failed to load texture."),
        ),
        (
            String::from("player_2"),
            load_texture("images/player_2.png")
                .await
                .expect("Failed to load texture."),
        ),
        (
            String::from("player_3"),
            load_texture("images/player_3.png")
                .await
                .expect("Failed to load texture."),
        ),
        (
            String::from("wall"),
            load_texture("images/wall.png")
                .await
                .expect("Failed to load texture."),
        ),
    ]);

    build_textures_atlas();
    return texture_atlas;
}

async fn make_sound_atlas() -> HashMap<String, Sound> {
    let sound_atlas = HashMap::from([
        (
            String::from("correct"),
            audio::load_sound("sounds/correct.wav")
                .await
                .expect("Failed to load sound."),
        ),
        (
            String::from("incorrect"),
            audio::load_sound("sounds/incorrect.wav")
                .await
                .expect("Failed to load sound."),
        ),
        (
            String::from("wall"),
            audio::load_sound("sounds/wall.wav")
                .await
                .expect("Failed to load sound."),
        ),
    ]);

    return sound_atlas;
}
