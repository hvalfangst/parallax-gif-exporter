use minifb::{Window, WindowOptions};
use std::io::{BufRead, Read};
use std::env;

use crate::state::constants::graphics::{SCALED_WINDOW_HEIGHT, SCALED_WINDOW_WIDTH};
use crate::state::structs::State;
use crate::{
    graphics::sprites::SpriteMaps,
    state::core_logic::initialize_core_logic_map,
    state::event_loop::start_event_loop,
};

mod state;
mod graphics;


fn main() {
    let args: Vec<String> = env::args().collect();
    let experimental_mode = args.get(1).map(|s| s == "--experimental").unwrap_or(false);
    let sprites = SpriteMaps::new();
    let core_logic = initialize_core_logic_map();

    // Create a window with the dimensions of the primary monitor
    let mut window = Window::new(
        "Parallax Exporter",
        SCALED_WINDOW_WIDTH,
        SCALED_WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    // Initialize window and scaled buffer
    let mut window_buffer = vec![0; 256 * 224];
    let mut scaled_buffer = vec![0; SCALED_WINDOW_WIDTH * SCALED_WINDOW_HEIGHT];
    let camera = state::structs::Camera::new(0.0, 0.0);


    let state = State {
        camera,
        sprites,
        window_buffer: &mut window_buffer,
        window_width: SCALED_WINDOW_WIDTH,
        window_height: SCALED_WINDOW_HEIGHT,
        window: &mut window,
        scaled_buffer: &mut scaled_buffer,
        art_width: 256,
        art_height: 224,
        lighthouse_sprite_frame_index: 0,
        ground_sprite_frame_index: 0,
        mountains_sprite_frame_index: 0,
        last_ground_sprite_frame_index_change: std::time::Instant::now(),
        last_light_house_sprite_frame_index_change: std::time::Instant::now(),
    };

    start_event_loop(state, core_logic, experimental_mode);
}