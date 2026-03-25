//! Energy system constants - tweak these to balance power management.

// --- Reactor ---
pub const REACTOR_MAX_POWER: u32 = 10;

// --- Default allocation ---
pub const DEFAULT_SHIELD_POWER: u32 = 4;
pub const DEFAULT_WEAPON_POWER: u32 = 3;
pub const DEFAULT_ENGINE_POWER: u32 = 3;

// --- Max levels per system (upgradeable cap) ---
pub const SHIELD_MAX_LEVEL: u32 = 4;
pub const WEAPON_MAX_LEVEL: u32 = 5;
pub const ENGINE_MAX_LEVEL: u32 = 3;

// --- Shield effects per cell ---
pub const SHIELD_HP_PER_CELL: f32 = 50.0;
pub const SHIELD_REGEN_PER_CELL: f32 = 10.0;

// --- Engine effects per cell ---
pub const ENGINE_SPEED_BONUS_PER_CELL: f32 = 0.25;
pub const ENGINE_BASE_SPEED_FRACTION: f32 = 0.3;
pub const ENGINE_EVASION_PER_CELL: f32 = 0.05;
