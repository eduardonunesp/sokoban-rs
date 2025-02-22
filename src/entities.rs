use crate::components::*;
use hecs::{Entity, World};
use macroquad::texture::Texture2D;
use std::collections::HashMap;

pub fn create_wall(
    world: &mut World,
    texture_atlas: HashMap<String, Texture2D>,
    position: Position,
) -> Entity {
    let texture = texture_atlas
        .get("wall")
        .expect("failed to load wall texture");
    world.spawn((
        Position { z: 10, ..position },
        Renderable::new_static(texture.clone()),
        Wall {},
        Immovable {},
    ))
}

pub fn create_floor(
    world: &mut World,
    texture_atlas: HashMap<String, Texture2D>,
    position: Position,
) -> Entity {
    let texture = texture_atlas
        .get("floor")
        .expect("failed to load floor texture");
    world.spawn((
        Position { z: 5, ..position },
        Renderable::new_static(texture.clone()),
    ))
}

pub fn create_box(
    world: &mut World,
    texture_atlas: HashMap<String, Texture2D>,
    position: Position,
    colour: BoxColour,
) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable::new_animated(vec![
            texture_atlas
                .get(&format!("box_{}_1", colour))
                .expect("failed to load box texture")
                .clone(),
            texture_atlas
                .get(&format!("box_{}_2", colour))
                .expect("failed to load box texture")
                .clone(),
        ]),
        Box { colour },
        Movable {},
    ))
}

pub fn create_box_spot(
    world: &mut World,
    texture_atlas: HashMap<String, Texture2D>,
    position: Position,
    colour: BoxColour,
) -> Entity {
    let texture = texture_atlas
        .get(&format!("box_spot_{}", colour))
        .expect("failed to load box spot texture");
    world.spawn((
        Position { z: 9, ..position },
        Renderable::new_static(texture.clone()),
        BoxSpot { colour },
    ))
}

pub fn create_player(
    world: &mut World,
    texture_atlas: HashMap<String, Texture2D>,
    position: Position,
) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable::new_animated(vec![
            texture_atlas
                .get("player_1")
                .expect("failed to load player texture")
                .clone(),
            texture_atlas
                .get("player_2")
                .expect("failed to load player texture")
                .clone(),
            texture_atlas
                .get("player_3")
                .expect("failed to load player texture")
                .clone(),
        ]),
        Player {},
        Movable {},
    ))
}

pub fn create_gameplay(world: &mut World) -> Entity {
    world.spawn((Gameplay::default(),))
}

pub fn create_time(world: &mut World) -> Entity {
    world.spawn((Time::default(),))
}

pub fn create_event_queue(world: &mut World) -> Entity {
    world.spawn((EventQueue::default(),))
}

pub fn create_audio_store(world: &mut World) -> Entity {
    world.spawn((AudioStore::default(),))
}
