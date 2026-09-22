use bevy::{math::DVec2, prelude::*};
use bevy_rapier2d::prelude::*;

const GRAVITATIONAL_CONSTANT: f64 = 0.0000000000667430;
const SOLAR_MASS: f32 = 1.989e30;
const ASTRONOMICAL_UNIT: f32 = 1.496e11;
const DISPLAY_DISTANCE: f32 = 400.0;
const TIME_SCALE: f64 = 100.0;


fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_physics(mut commands: Commands) {

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(-0.5 * DISPLAY_DISTANCE, 0.5 * DISPLAY_DISTANCE, 0.0))
        .insert(GravityScale(0.0))
        .insert(AdditionalMassProperties::Mass(SOLAR_MASS))
        .insert(Velocity::linear(Vec2::new(-87.0, 32.0)))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        });

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.5 * DISPLAY_DISTANCE, 0.0, 0.0))
        .insert(GravityScale(0.0))
        .insert(AdditionalMassProperties::Mass(0.1 * SOLAR_MASS))
        .insert(Velocity::linear(Vec2::new(81.0, 32.0)))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        });

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(-0.75 * DISPLAY_DISTANCE, -0.1 * DISPLAY_DISTANCE, 0.0))
        .insert(GravityScale(0.0))
        .insert(AdditionalMassProperties::Mass(2.0 * SOLAR_MASS))
        .insert(Velocity::linear(Vec2::new(87.0, -40.0)))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        });
}

fn apply_forces(
    bodies: Query<(Entity, &Transform, &AdditionalMassProperties)>,
    mut forces: Query<&mut ExternalForce>,
) {
    let snapshot: Vec<(Entity, DVec2, f64)> = bodies
        .iter()
        .filter_map(|(entity, transform, mass)| match mass {
            AdditionalMassProperties::Mass(mass) => Some((
                entity,
                transform
                    .translation
                    .truncate()
                    .as_dvec2()
                    * (ASTRONOMICAL_UNIT as f64 / DISPLAY_DISTANCE as f64),
                *mass as f64,
            )),
            _ => None,
        })
        .collect();

    for (entity, position, mass) in &snapshot {
        let mut net_force = DVec2::ZERO;

        for (other_entity, other_position, other_mass) in &snapshot {
            if entity == other_entity {
                continue;
            }

            let offset = *other_position - *position;
            let distance_squared = offset.length_squared();

            if distance_squared == 0.0 {
                continue;
            }

            let force_magnitude = GRAVITATIONAL_CONSTANT * *mass * *other_mass
                / distance_squared
                * TIME_SCALE
                * TIME_SCALE;
            net_force += offset.normalize() * force_magnitude;
        }

        if let Ok(mut force) = forces.get_mut(*entity) {
            force.force = Vec2::new(net_force.x as f32, net_force.y as f32);
        }
    }
}

fn freeze_on_collision(
    mut collision_events: MessageReader<CollisionEvent>,
    mut bodies: Query<&mut RigidBody>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity_a, entity_b, _) = event {
            if let Ok(mut body) = bodies.get_mut(*entity_a) {
                *body = RigidBody::Fixed;
            }

            if let Ok(mut body) = bodies.get_mut(*entity_b) {
                *body = RigidBody::Fixed;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, (apply_forces, freeze_on_collision))
        .run();
}