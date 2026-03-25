//! Control constants - tweak these for feel and responsiveness.

// --- Movement ---
pub const DECELERATION_FACTOR: f32 = 0.98;
pub const REVERSE_SPEED_FRACTION: f32 = 0.3;

// --- Camera ---
pub const CAMERA_FOLLOW_LERP: f32 = 0.1;

// --- Game Speed ---
pub const GAME_SPEED_STEPS: [f32; 4] = [0.25, 0.5, 1.0, 2.0];
pub const DEFAULT_GAME_SPEED: f32 = 1.0;

// --- Targeting ---
pub const MIN_CLICK_TARGET_RADIUS: f32 = 20.0;
pub const AUTO_TARGET_RANGE: f32 = 600.0;
pub const AUTO_TARGET_RETARGET_INTERVAL: f32 = 0.5;
