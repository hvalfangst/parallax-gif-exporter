use std::time::Instant;
use minifb::Window;
use crate::graphics::sprites::SpriteMaps;

pub struct Camera {
    pub x: f32,
    pub y: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32) -> Self {
        Camera {
            x,
            y
        }
    }
}

pub struct State<'a> {
    pub camera: Camera, // Camera object
    pub sprites: SpriteMaps, // Sprite maps
    pub window_buffer: &'a mut Vec<u32>, // Window buffer
    pub window_width: usize, // Width of the window
    pub window_height: usize, // Height of the window
    pub window: &'a mut Window, // Window object
    pub scaled_buffer: &'a mut Vec<u32>, // Scaled buffer
    pub art_width: usize, // Width of the game world
    pub art_height: usize, // Height of the game world
    pub lighthouse_sprite_frame_index: usize, // Index for the lighthouse sprite animation frame
    pub ground_sprite_frame_index: usize, // Index for the ground sprite animation frame
    pub mountains_sprite_frame_index: usize, // Index for the mountains sprite animation frame
    pub last_ground_sprite_frame_index_change: Instant, // Timestamp of the last ground sprite frame change
    pub last_light_house_sprite_frame_index_change: Instant, // Timestamp of the last lighthouse sprite frame change
}