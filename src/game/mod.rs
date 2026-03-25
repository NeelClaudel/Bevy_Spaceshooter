use bevy::prelude::*;

use crate::constants::FIXED_TIME_STEP;
use crate::player::GamePaused;

#[derive(Resource)]
pub struct GameTimeDelta(pub f32);

#[derive(Resource)]
pub struct GameSpeed(pub f32);

pub static DESPAWN_STAGE: &str = "despawn_stage";

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum DespawnSet {
    Parallel,
    CommandFlush,
}

#[derive(Default)]
pub struct BaseGamePlugin;

impl Plugin for BaseGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(Update, control_game_speed);
        app.add_systems(Update, crate::materials::set_ship_shader_team_color);
    }
}

fn startup(mut commands: Commands) {
    commands.insert_resource(GameTimeDelta(FIXED_TIME_STEP));
    commands.insert_resource(GameSpeed(
        crate::player::constants::controls::DEFAULT_GAME_SPEED,
    ));
}

fn control_game_speed(
    mut dt: ResMut<GameTimeDelta>,
    speed: Res<GameSpeed>,
    paused: Res<GamePaused>,
) {
    if paused.0 {
        dt.0 = 0.0;
    } else {
        dt.0 = FIXED_TIME_STEP * speed.0;
    }
}
