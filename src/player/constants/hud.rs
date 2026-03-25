//! HUD visual constants - tweak colors, sizes, and positions.

// --- Bar colors [R, G, B, A] ---
pub const HEALTH_BAR_COLOR: [f32; 4] = [0.8, 0.2, 0.2, 1.0];
pub const SHIELD_BAR_COLOR: [f32; 4] = [0.2, 0.4, 0.9, 1.0];
pub const ENERGY_SHIELD_COLOR: [f32; 4] = [0.3, 0.6, 1.0, 1.0];
pub const ENERGY_WEAPON_COLOR: [f32; 4] = [1.0, 0.5, 0.2, 1.0];
pub const ENERGY_ENGINE_COLOR: [f32; 4] = [0.3, 0.9, 0.3, 1.0];
pub const BAR_BACKGROUND_COLOR: [f32; 4] = [0.15, 0.15, 0.15, 0.8];
pub const TARGET_INDICATOR_COLOR: [f32; 4] = [1.0, 0.8, 0.0, 1.0];

// --- Bar dimensions ---
pub const HEALTH_BAR_WIDTH: f32 = 250.0;
pub const HEALTH_BAR_HEIGHT: f32 = 20.0;
pub const SHIELD_BAR_WIDTH: f32 = 250.0;
pub const SHIELD_BAR_HEIGHT: f32 = 16.0;
pub const ENERGY_BAR_WIDTH: f32 = 200.0;
pub const ENERGY_BAR_HEIGHT: f32 = 14.0;
pub const ENERGY_CELL_WIDTH: f32 = 18.0;
pub const ENERGY_CELL_GAP: f32 = 2.0;

// --- Layout ---
pub const HUD_PADDING: f32 = 12.0;
pub const HUD_MARGIN: f32 = 16.0;
pub const HUD_PANEL_BG_COLOR: [f32; 4] = [0.05, 0.05, 0.1, 0.85];
pub const HUD_PANEL_BORDER_COLOR: [f32; 4] = [0.3, 0.3, 0.4, 0.6];
pub const HUD_FONT_SIZE: f32 = 14.0;
pub const HUD_LABEL_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
