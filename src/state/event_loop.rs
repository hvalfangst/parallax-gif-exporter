use crate::graphics::render_graphics::render_pixel_buffer;
use crate::graphics::update_graphics::update_pixel_buffer;
use crate::state::core_logic::{execute_core_logic, CoreLogic};
use crate::state::structs::State;

use minifb::Key;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::process::exit;
use std::rc::Rc;
use std::time::Instant;
use crate::graphics::gif::{initialize_gif_encoder, process_frame};
use crate::state::constants::graphics::MAX_GIF_FRAMES;

pub fn start_event_loop(mut game_state: State, core_logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>>, experimental: bool, headless: bool) {
    let mut x_offset = 0;
    let mut frame_count = 0;
    let mut last_update = Instant::now();
    let width = game_state.window_width as u16;
    let height = game_state.window_height as u16;

    let mut rng = if experimental {
        Some(rand::thread_rng())
    } else {
        None
    };

    let path = if experimental {
        let date = chrono::Local::now().format("%Y_%m_%d").to_string();
        format!("mutations/{}.gif", date)
    } else {
        String::from("output.gif")
    };
    let mut image = File::create(&path).unwrap();
    let mut encoder = initialize_gif_encoder(&mut image, width, height);

    let encoding_mode = if experimental {
        "experimental"
    } else {
        "standard"
    };

    let window_mode = if headless {
        "headless"
    } else {
        "windowed"
    };

    println!("Starting GIF encoding in {} {} mode with dimensions: {}x{}", encoding_mode, window_mode, width, height);

    loop {
        if !headless {
            if let Some(window) = &game_state.window {
                if !window.is_open() || window.is_key_down(Key::Escape) {
                    break;
                }
            }
        }

        execute_core_logic(&mut game_state, &core_logic_map);
        update_pixel_buffer(&mut game_state);
        render_pixel_buffer(&mut game_state, headless);

        if last_update.elapsed() >= std::time::Duration::from_nanos(0) {
            if frame_count < MAX_GIF_FRAMES {
                process_frame(&mut game_state, &mut encoder, width, height, &mut x_offset, &mut frame_count, &mut rng, experimental);
                last_update = Instant::now();
            } else {
                println!("Finished capturing {} frames to file '{}'", MAX_GIF_FRAMES, &path);
                exit(0);
            }
        }
    }
}