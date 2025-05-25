pub mod graphics {
    use std::time::Duration;

    pub const FRAME_DURATION: Duration = Duration::from_nanos(16666667); // 16.6666667 ms = 60 FPS
    pub const BACKGROUND_CHANGE_INTERVAL: Duration = Duration::from_secs(1);
    pub const SCALED_WINDOW_WIDTH: usize = 960;
    pub const SCALED_WINDOW_HEIGHT: usize = 540;
    pub const TILE_WIDTH: usize = 16;
    pub const TILE_HEIGHT: usize = 16;
    pub const MAX_GIF_FRAMES: usize = 78; // Total number of frames to capture
}





