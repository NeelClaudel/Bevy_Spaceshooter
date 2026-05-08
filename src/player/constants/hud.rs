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

// --- Weapon panel (top-right, below target info) ---
pub const WEAPON_PANEL_TOP: f32 = 64.0;
pub const WEAPON_PANEL_WIDTH: f32 = 260.0;
pub const WEAPON_ROW_GAP: f32 = 4.0;
pub const WEAPON_TOGGLE_WIDTH: f32 = 38.0;
pub const WEAPON_TOGGLE_HEIGHT: f32 = 20.0;
pub const WEAPON_NAME_WIDTH: f32 = 78.0;
pub const WEAPON_COOLDOWN_BAR_WIDTH: f32 = 70.0;
pub const WEAPON_COOLDOWN_BAR_HEIGHT: f32 = 8.0;
pub const WEAPON_AMMO_WIDTH: f32 = 36.0;
pub const WEAPON_TOGGLE_ON_COLOR: [f32; 4] = [0.2, 0.65, 0.25, 0.9];
pub const WEAPON_TOGGLE_OFF_COLOR: [f32; 4] = [0.55, 0.2, 0.2, 0.9];
pub const WEAPON_TOGGLE_NOPOWER_COLOR: [f32; 4] = [0.35, 0.35, 0.35, 0.7];
pub const WEAPON_COOLDOWN_FILL_COLOR: [f32; 4] = [0.3, 0.7, 1.0, 0.95];
pub const WEAPON_COOLDOWN_BG_COLOR: [f32; 4] = [0.12, 0.12, 0.18, 0.9];
