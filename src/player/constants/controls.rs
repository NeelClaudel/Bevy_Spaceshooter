//! Control constants - tweak these for feel and responsiveness.

// --- Movement ---
pub const DECELERATION_FACTOR: f32 = 0.98;
pub const REVERSE_SPEED_FRACTION: f32 = 0.3;
/// Cursor radius (world units) inside which aim is suppressed so the ship
/// doesn't spin when the cursor is on top of it.
pub const AIM_DEADZONE_RADIUS: f32 = 8.0;

// --- Camera ---
pub const CAMERA_FOLLOW_LERP: f32 = 0.1;
pub const CAMERA_ZOOM_MIN: f32 = 0.5;
pub const CAMERA_ZOOM_MAX: f32 = 3.0;
pub const CAMERA_ZOOM_STEP: f32 = 1.1;
pub const CAMERA_ZOOM_LERP: f32 = 0.15;
pub const CAMERA_ZOOM_DEFAULT: f32 = 1.0;

// --- Game Speed ---
pub const GAME_SPEED_STEPS: [f32; 4] = [0.25, 0.5, 1.0, 2.0];
pub const DEFAULT_GAME_SPEED: f32 = 1.0;

// --- Targeting ---
pub const MIN_CLICK_TARGET_RADIUS: f32 = 20.0;

// --- Firing ---
/// Cooldown multiplier applied to player turrets that fire under pure auto-fire
/// (no manual button held). Manual-fire bypasses this penalty.
pub const AUTO_FIRE_COOLDOWN_PENALTY: f32 = 2.0;
