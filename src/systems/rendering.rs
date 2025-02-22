use crate::components::*;
use crate::constants::*;

use hecs::{Entity, World};
use macroquad::prelude::*;
use std::time::Duration;

// ANCHOR: run_rendering
pub fn run_rendering(world: &World) {
    // Clearing the screen (this gives us the background colour)
    clear_background(LIGHTGRAY);

    // Get time
    let mut query = world.query::<&Time>();
    let time = query.iter().next().unwrap().1;

    // Get all the renderables with their positions and sort by the position z
    // This will allow us to have entities layered visually.
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    // Iterate each of the renderables, determine which image path should be rendered
    // at which drawparams, and then add that to the rendering_batches.
    for (_, (position, renderable)) in rendering_data.iter() {
        // Load the image
        let image = get_image(renderable, time.delta);
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        draw_texture(&image, x, y, WHITE);
    }

    // Render any text
    let mut query = world.query::<&Gameplay>();
    let gameplay = query.iter().next().unwrap().1;

    draw_text_ex(
        &gameplay.state.to_string(),
        525.0,
        80.0,
        TextParams {
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
    );

    draw_text_ex(
        &gameplay.moves_count.to_string(),
        525.0,
        100.0,
        TextParams {
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
    );

    // Render FPS
    let fps = format!("FPS: {:.0}", get_fps());
    draw_text_ex(
        &fps,
        525.0,
        120.0,
        TextParams {
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
    );
}

pub fn get_image(renderable: &Renderable, delta: Duration) -> Texture2D {
    let path_index = match renderable.kind() {
        RenderableKind::Static => {
            // We only have one image, so we just return that
            0
        }
        RenderableKind::Animated => {
            // If we have multiple, we want to select the right one based on the delta time.
            // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
            // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
            // we technically are on the next iteration of the loop (or on 0), but we will let
            // the renderable handle this logic of wrapping frames.
            ((delta.as_millis() % 1000) / 250) as usize
        }
    };

    renderable.path(path_index)
}
