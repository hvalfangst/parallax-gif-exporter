use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use gif::{Encoder, Frame, Repeat};
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::state::structs::State;

pub fn initialize_gif_encoder(image: &mut File, width: u16, height: u16) -> Encoder<&mut File> {
    let color_map = &[];
    let mut encoder = Encoder::new(image, width, height, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    encoder
}

pub fn process_frame(
    game_state: &mut State,
    encoder: &mut Encoder<&mut File>,
    width: u16,
    height: u16,
    x_offset: &mut usize,
    frame_count: &mut usize,
    rng: &mut Option<ThreadRng>,
    experimental: bool,
) {
    let (color_map, mut color_to_index_map) = generate_color_map(&game_state.scaled_buffer, rng, experimental);
    *x_offset += 20;
    *frame_count += 1;
    game_state.camera.x = *x_offset as f32;

    let buffer = map_pixels_to_indices(&game_state.scaled_buffer, &mut color_to_index_map);
    if buffer.is_empty() {
        println!("Warning: Buffer is empty, skipping frame {}", *frame_count);
        return;
    }
    write_frame_to_gif(encoder, width, height, &color_map, &buffer, *frame_count);
}

fn generate_color_map(buffer: &[u32], rng: &mut Option<ThreadRng>, experimental: bool) -> (Vec<u8>, HashMap<u32, u8>) {
    let unique_colors: HashSet<u32> = buffer.iter().cloned().collect();
    let mut color_map = Vec::new();
    let mut color_to_index_map = HashMap::new();

    for (index, &color) in unique_colors.iter().enumerate() {
        let red = ((color >> 16) & 0xFF) as u8;
        let green = ((color >> 8) & 0xFF) as u8;
        let blue = (color & 0xFF) as u8;

        if experimental {
            // Experimental: Modify the color values slightly
            let red_shift = rng.as_mut().unwrap().gen_range(-5..=5);
            let green_shift = rng.as_mut().unwrap().gen_range(-5..=5);
            let blue_shift = rng.as_mut().unwrap().gen_range(-5..=5);
            let modified_red = (red as i16 + red_shift).clamp(0, 255) as u8;
            let modified_green = (green as i16 + green_shift).clamp(0, 255) as u8;
            let modified_blue = (blue as i16 + blue_shift).clamp(0, 255) as u8;

            // Randomly decide which colors to push (at least one must be pushed)
            let mut push_red = rng.as_mut().unwrap().gen_bool(0.5);
            let mut push_green = rng.as_mut().unwrap().gen_bool(0.5);
            let mut push_blue = rng.as_mut().unwrap().gen_bool(0.5);

            // If none were selected, randomly choose one
            if !push_red && !push_green && !push_blue {
                match rng.as_mut().unwrap().gen_range(0..3) {
                    0 => push_red = true,
                    1 => push_green = true,
                    _ => push_blue = true,
                }
            }

            if push_red { color_map.push(modified_red); }
            if push_green { color_map.push(modified_green); }
            if push_blue { color_map.push(modified_blue); }
        } else {
            // Use original colors for non-experimental mode
            color_map.push(red);
            color_map.push(green);
            color_map.push(blue);
        }

        color_to_index_map.insert(color, index as u8);
    }

    (color_map, color_to_index_map)
}

fn map_pixels_to_indices(buffer: &[u32], color_to_index_map: &mut HashMap<u32, u8>) -> Vec<u8> {
    let mut logged_pixels = HashSet::new();
    let mut next_index = color_to_index_map.len() as u8;
    let mut color_to_index = |pixel: u32| {
        logged_pixels.insert(pixel);
        *color_to_index_map.entry(pixel).or_insert_with(|| {
            let index = next_index;
            next_index += 1;
            index
        })
    };

    buffer.iter().map(|&pixel| color_to_index(pixel)).collect()
}

fn write_frame_to_gif(
    encoder: &mut Encoder<&mut File>,
    width: u16,
    height: u16,
    color_map: &[u8],
    buffer: &[u8],
    frame_count: usize,
) {
    let mut frame = Frame::default();
    frame.width = width;
    frame.height = height;
    frame.palette = Some(color_map.to_vec());
    frame.buffer = Cow::Borrowed(buffer);
    frame.delay = 10;

    encoder.write_frame(&frame).expect("Failed to write frame to GIF");
    println!("Frame {} written to GIF file.", frame_count);
}