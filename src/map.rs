use crate::{
    components::{BoxColour, Position},
    entities::{create_box, create_box_spot, create_floor, create_player, create_wall},
};
use hecs::World;
use macroquad::{audio::Sound, texture::Texture2D};
use std::collections::HashMap;

pub fn initialize_level(
    world: &mut World,
    texture_atlas: HashMap<String, Texture2D>,
    sounds_atlas: HashMap<String, Sound>,
) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string(), texture_atlas);
    load_sounds(world, sounds_atlas);
}

pub fn load_map(world: &mut World, map_string: String, texture_atlas: HashMap<String, Texture2D>) {
    // read all lines
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            // Create the position at which to create something on the map
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we will get the z from the factory functions
            };

            // Figure out what object we should create
            match *column {
                "." => {
                    create_floor(world, texture_atlas.clone(), position);
                }
                "W" => {
                    create_floor(world, texture_atlas.clone(), position);
                    create_wall(world, texture_atlas.clone(), position);
                }
                "P" => {
                    create_floor(world, texture_atlas.clone(), position);
                    create_player(world, texture_atlas.clone(), position);
                }
                "BB" => {
                    create_floor(world, texture_atlas.clone(), position);
                    create_box(world, texture_atlas.clone(), position, BoxColour::Blue);
                }
                "RB" => {
                    create_floor(world, texture_atlas.clone(), position);
                    create_box(world, texture_atlas.clone(), position, BoxColour::Red);
                }
                "BS" => {
                    create_floor(world, texture_atlas.clone(), position);
                    create_box_spot(world, texture_atlas.clone(), position, BoxColour::Blue);
                }
                "RS" => {
                    create_floor(world, texture_atlas.clone(), position);
                    create_box_spot(world, texture_atlas.clone(), position, BoxColour::Red);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}

pub fn load_sounds(world: &mut World, sounds_atlas: HashMap<String, Sound>) {
    let mut query = world.query::<&mut crate::components::AudioStore>();
    let audio_store = query.iter().next().unwrap().1;

    let sounds = ["correct", "incorrect", "wall"];

    for sound in sounds.iter() {
        let sound_name = *sound;
        audio_store.sounds.insert(
            sound_name.to_string(),
            Box::new(sounds_atlas.get(sound_name).unwrap().clone()),
        );
    }
}
