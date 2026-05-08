//! Weapon stats - tweak these to balance each weapon type.

use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

// --- Pulse Laser ---
pub const PULSE_LASER_POWER_COST: u32 = 1;
pub const PULSE_LASER_COOLDOWN: f32 = 0.8;
pub const PULSE_LASER_DAMAGE: f32 = 20.0;
pub const PULSE_LASER_RANGE: f32 = 350.0;
pub const PULSE_LASER_CONE: f32 = PI;
pub const PULSE_LASER_ACCURACY: f32 = 3.0;

// --- Heavy Laser ---
pub const HEAVY_LASER_POWER_COST: u32 = 2;
pub const HEAVY_LASER_COOLDOWN: f32 = 2.0;
pub const HEAVY_LASER_DAMAGE: f32 = 60.0;
pub const HEAVY_LASER_RANGE: f32 = 300.0;
pub const HEAVY_LASER_CONE: f32 = FRAC_PI_4;
pub const HEAVY_LASER_ACCURACY: f32 = 5.0;

// --- Missile Launcher ---
pub const MISSILE_POWER_COST: u32 = 1;
pub const MISSILE_COOLDOWN: f32 = 2.5;
pub const MISSILE_DAMAGE: f32 = 40.0;
pub const MISSILE_RANGE: f32 = 700.0;
pub const MISSILE_CONE: f32 = TAU;
pub const MISSILE_ACCURACY: f32 = 10.0;
pub const MISSILE_MAX_AMMO: u32 = 12;
pub const MISSILE_START_AMMO: u32 = 8;

// --- Flak Cannon ---
pub const FLAK_POWER_COST: u32 = 2;
pub const FLAK_COOLDOWN: f32 = 2.5;
pub const FLAK_DAMAGE_PER_PROJECTILE: f32 = 5.0;
pub const FLAK_PROJECTILE_COUNT: u32 = 5;
pub const FLAK_RANGE: f32 = 400.0;
pub const FLAK_CONE: f32 = FRAC_PI_2;
pub const FLAK_ACCURACY: f32 = 1.5;

// --- Gatling (ballistic, high RoF, low per-shot damage) ---
pub const GATLING_POWER_COST: u32 = 2;
pub const GATLING_COOLDOWN: f32 = 0.08; // ~12 rounds/sec
pub const GATLING_DAMAGE: f32 = 4.0;
pub const GATLING_RANGE: f32 = 600.0;
pub const GATLING_CONE: f32 = TAU; // 360°, gating done by player_primary_fire
pub const GATLING_ACCURACY: f32 = 2.0;
pub const GATLING_BULLET_SPEED: f32 = 800.0;
pub const GATLING_SPREAD: f32 = 0.05; // ±~3° random aim jitter

// --- Turret position offsets on player ship (in local space) ---
pub const TURRET_FRONT_TOP_OFFSET: [f32; 2] = [0.0, 12.0];
pub const TURRET_FRONT_BOTTOM_OFFSET: [f32; 2] = [0.0, -12.0];
pub const TURRET_SIDE_LEFT_OFFSET: [f32; 2] = [-14.0, 0.0];
pub const TURRET_SIDE_RIGHT_OFFSET: [f32; 2] = [14.0, 0.0];
pub const TURRET_CENTER_OFFSET: [f32; 2] = [0.0, 0.0];
