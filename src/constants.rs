pub const ASPECT_RATIO: f32 = 1.0; // 16.0 / 10.0;
pub const RESOLUTION_Y: usize = 200;
pub const RESOLUTION_X: usize = (RESOLUTION_Y as f32 * ASPECT_RATIO) as usize;

pub const RESOLUTION_X_FLOAT: f32 = RESOLUTION_X as f32;
pub const RESOLUTION_Y_FLOAT: f32 = RESOLUTION_Y as f32;
pub const TOTAL_PIXELS: usize = RESOLUTION_X * RESOLUTION_Y;
