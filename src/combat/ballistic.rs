//! Ballistic projectiles: simple linear-motion bullets with swept collision.
//!
//! A bullet entity carries damage/team/hit-effect data and moves in a straight
//! line each tick. On collision with an enemy hitbox we spawn a transient
//! `Effect` entity so the standard attack pipeline (evasion → shields →
//! damage → hit explosion) handles the actual damage, then despawn the bullet.

use bevy::prelude::*;

use crate::fx::HitEffect;
use crate::game::GameTimeDelta;

use super::{
    attack::{Attack, AttackResult},
    damage::Damage,
    effects::{Effect, EffectLocation, Effectiveness, Instigator, SourceTransform},
    projectile::CircularHitBox,
    CombatSystems, Target, Team,
};

/// A linearly-moving projectile. `velocity` is in world units per second.
#[derive(Component, Copy, Clone)]
pub struct BallisticProjectile {
    pub velocity: Vec2,
}

pub struct BallisticPlugin;

impl Plugin for BallisticPlugin {
    fn build(&self, app: &mut App) {
        // Movement runs before combat systems each tick so the swept collision
        // sees the just-advanced position. The Effect entities we spawn are
        // consumed by the regular pipeline that runs in CombatSystems.
        app.add_systems(
            FixedUpdate,
            (move_ballistics, ballistics_check_collisions)
                .chain()
                .before(CombatSystems),
        );
    }
}

fn move_ballistics(
    dt: Res<GameTimeDelta>,
    mut query: Query<(&BallisticProjectile, &mut Transform)>,
) {
    for (ballistic, mut transform) in query.iter_mut() {
        transform.translation.x += ballistic.velocity.x * dt.0;
        transform.translation.y += ballistic.velocity.y * dt.0;
    }
}

fn ballistics_check_collisions(
    mut commands: Commands,
    dt: Res<GameTimeDelta>,
    bullets: Query<(
        Entity,
        &BallisticProjectile,
        &Transform,
        &Damage,
        &Attack,
        &Team,
        &HitEffect,
        &Instigator,
    )>,
    targets: Query<(Entity, &GlobalTransform, &CircularHitBox, &Team)>,
) {
    for (bullet_entity, ballistic, transform, damage, attack, team, hit_fx, instigator) in
        bullets.iter()
    {
        let curr = transform.translation.truncate();
        let prev = curr - ballistic.velocity * dt.0;
        let segment = curr - prev;
        let seg_len = segment.length();
        if seg_len < 0.001 {
            continue;
        }
        let dir = segment / seg_len;

        // Find the closest enemy whose hitbox intersects the swept segment.
        let mut closest: Option<(Entity, f32, Vec2)> = None;
        for (target_entity, target_gtf, hitbox, target_team) in targets.iter() {
            if target_team.0 == team.0 {
                continue;
            }
            let center = target_gtf.translation().truncate();
            let to_center = center - prev;
            let t = to_center.dot(dir).clamp(0.0, seg_len);
            let closest_on_seg = prev + dir * t;
            let dist_sq = (center - closest_on_seg).length_squared();
            if dist_sq > hitbox.radius * hitbox.radius {
                continue;
            }
            if closest.map_or(true, |(_, prev_t, _)| t < prev_t) {
                closest = Some((target_entity, t, closest_on_seg));
            }
        }

        let Some((hit_entity, _, hit_point)) = closest else {
            continue;
        };

        let z = transform.translation.z;
        let hit_world = hit_point.extend(z);
        let hit_transform = Transform::from_translation(hit_world);
        let hit_global = GlobalTransform::from(hit_transform);

        commands.spawn((
            Damage(damage.0),
            Attack {
                accuracy: attack.accuracy,
                result: AttackResult::Hit,
            },
            Target(Some(hit_entity)),
            *instigator,
            SourceTransform(hit_global),
            hit_transform,
            hit_global,
            Effect,
            Effectiveness::default(),
            EffectLocation(hit_world),
            HitEffect {
                effect: hit_fx.effect,
            },
        ));

        commands.entity(bullet_entity).despawn();
    }
}
