//! Visual thruster constants. Tweak these to adjust flame size/feel.

// --- Nozzle offsets (ship-local, pre-scale) ---
// Ship mesh is 64×64. Negative Y is rear, positive Y is front.
pub const THRUSTER_FORWARD_OFFSET: [f32; 2] = [0.0, -22.0];
pub const THRUSTER_REVERSE_OFFSET: [f32; 2] = [0.0, 22.0];
/// Right-side nozzle. Fires when A is held (strafe left).
pub const THRUSTER_STRAFE_LEFT_OFFSET: [f32; 2] = [20.0, 0.0];
/// Left-side nozzle. Fires when D is held (strafe right).
pub const THRUSTER_STRAFE_RIGHT_OFFSET: [f32; 2] = [-20.0, 0.0];

// --- Flame dimensions (full-intensity values, ship-local pre-scale) ---
pub const THRUSTER_MAIN_LENGTH: f32 = 32.0;
pub const THRUSTER_MAIN_WIDTH: f32 = 9.0;
pub const THRUSTER_RCS_LENGTH: f32 = 14.0;
pub const THRUSTER_RCS_WIDTH: f32 = 7.0;

// --- Flame colors (HDR > 1.0 components blow out into bloom) ---
pub const THRUSTER_FLAME_COLOR: [f32; 4] = [4.0, 1.6, 0.4, 1.0]; // orange main thrust
pub const THRUSTER_RCS_COLOR: [f32; 4] = [1.5, 2.5, 4.0, 1.0]; // cool blue RCS
/// Inner core color for the warm main thruster: white-hot leaning yellow.
pub const THRUSTER_MAIN_CORE_COLOR: [f32; 4] = [5.0, 4.5, 3.0, 1.0];
/// Inner core color for the cool RCS thrusters: white-hot leaning blue.
pub const THRUSTER_RCS_CORE_COLOR: [f32; 4] = [3.5, 4.5, 5.0, 1.0];

// --- Layered flame core (child of outer flame; local scale multiplies parent's) ---
pub const THRUSTER_CORE_LENGTH_FACTOR: f32 = 0.7;
pub const THRUSTER_CORE_WIDTH_FACTOR: f32 = 0.45;

// --- Animation feel ---
/// Larger = snappier intensity follow. Roughly: time-to-90% ≈ 2.3 / rate.
pub const THRUSTER_INTENSITY_LERP_RATE: f32 = 14.0;
/// Below this intensity, the flame is treated as off (sprite hidden).
pub const THRUSTER_OFF_THRESHOLD: f32 = 0.05;

// --- Particle exhaust ---
/// Below this intensity, no particles are emitted.
pub const THRUSTER_EMIT_THRESHOLD: f32 = 0.4;
/// Seconds between particle emissions at full intensity. Scales with intensity.
pub const THRUSTER_PARTICLE_INTERVAL: f32 = 0.025;
/// How many particles to spawn per emission.
pub const THRUSTER_PARTICLES_PER_EMIT: u32 = 2;
/// Seconds a particle lives before despawn.
pub const THRUSTER_PARTICLE_LIFETIME: f32 = 0.4;
/// Initial size (world units) of a freshly-spawned particle.
pub const THRUSTER_PARTICLE_INITIAL_SIZE: f32 = 5.0;
/// Outward speed (world units/sec) along the exhaust direction.
pub const THRUSTER_PARTICLE_SPEED: f32 = 70.0;
/// Perpendicular speed range (±) — gives the plume a small cone shape.
pub const THRUSTER_PARTICLE_SPREAD: f32 = 25.0;
/// Probability of using the hotter (white-yellow) palette vs the ember (orange).
pub const THRUSTER_PARTICLE_HOT_PROBABILITY: f64 = 0.4;
/// Hot palette: white-yellow HDR.
pub const THRUSTER_PARTICLE_HOT_COLOR: [f32; 4] = [4.5, 3.5, 1.5, 1.0];
/// Ember palette: deep orange HDR — matches the outer flame.
pub const THRUSTER_PARTICLE_EMBER_COLOR: [f32; 4] = [3.5, 1.0, 0.2, 1.0];
