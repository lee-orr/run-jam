use crate::{
    assets::GameAssets,
    game_state::GameState,
    gravity::{self, DelayedActivity},
    gravity_spawner::Prediction,
    pickup::{self, ActivePickup, Pickup, PickupType, PICKUPS},
    player,
    space_material::SpaceMaterial,
};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_turborand::{DelegatedRng, GlobalRng};
use iyes_loopless::state::NextState;
use noisy_bevy::simplex_noise_2d;

#[derive(Resource, Debug)]
pub struct LevelBoundary {
    pub(crate) min: Vec2,
    pub(crate) max: Vec2,
}

#[derive(Component)]
pub struct Backdrop;

#[derive(Component)]
pub struct LevelEntity;

pub enum LevelEvent {
    LevelStarted,
    PickupCollected(PickupType),
}

#[derive(Resource)]
pub struct GoalStatus {
    pub current: GoalType,
    pub completed: Vec<GoalType>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GoalType {
    Chips,
    Fruit,
    Gas,
    Post,
    ToiletPaper,
}

impl GoalType {
    pub fn get_asset_string(&self) -> &'static str {
        match self {
            GoalType::Chips => "chips.png",
            GoalType::Fruit => "fruit.png",
            GoalType::Gas => "gas.png",
            GoalType::Post => "post.png",
            GoalType::ToiletPaper => "toilet-paper.png",
        }
    }
}

const GOAL_TYPES: [GoalType; 5] = [
    GoalType::Chips,
    GoalType::Fruit,
    GoalType::Gas,
    GoalType::Post,
    GoalType::ToiletPaper,
];

pub(crate) fn check_boundary(
    players: Query<&Transform, With<player::Player>>,
    boundary: Res<LevelBoundary>,
    mut commands: Commands,
) {
    for player in players.iter() {
        if player.translation.xy().cmplt(boundary.min).any()
            || player.translation.xy().cmpgt(boundary.max).any()
        {
            commands.insert_resource(NextState(GameState::GameOver));
            return;
        }
    }
}

pub(crate) fn update_backdrop(
    boundary: Res<LevelBoundary>,
    backdrop: Query<Entity, With<Backdrop>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<SpaceMaterial>>,
) {
    if !boundary.is_changed() {
        return;
    }

    let material = materials.add(SpaceMaterial {
        map_boundary: Vec4::new(
            boundary.min.x,
            boundary.min.y,
            boundary.max.x,
            boundary.max.y,
        ),
        ..default()
    });

    for backdrop in backdrop.iter() {
        commands.entity(backdrop).insert(material.clone());
    }
}

pub fn start_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut events: EventWriter<LevelEvent>,
) {
    commands.insert_resource(LevelBoundary {
        min: Vec2::new(-600., -400.),
        max: Vec2::new(600., 400.),
    });
    events.send(LevelEvent::LevelStarted);
    commands.insert_resource(Prediction::None);
    commands.insert_resource(GoalStatus {
        current: GoalType::Chips,
        completed: vec![],
    });
    commands.insert_resource(ActivePickup(None));

    commands
        .spawn((SpatialBundle::default(), LevelEntity))
        .with_children(|p| {
            p.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::ONE * 50.),
                        ..Default::default()
                    },
                    texture: assets.player.clone(),
                    transform: Transform::from_translation(Vec3::new(-500., 0., 0.)),
                    ..default()
                },
                gravity::GravitationalBody(1., 25.),
                player::Player,
                gravity::GravitationTransform::velocity(Vec2::X * 100.),
            ));
        });
}

pub fn clear_level(
    mut commands: Commands,
    levels: Query<Entity, With<LevelEntity>>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for level in levels.iter() {
        commands.entity(level).despawn_recursive();
    }

    for mut camera in cameras.iter_mut() {
        camera.translation = Vec3::ZERO;
    }
}

const GOAL_GAP: f32 = 100.;
const PLANET_GAP: f32 = 50.;

const MAX_POSITION_TESTS: usize = 10;

fn get_spawn_position<T: Fn(Vec2) -> bool>(
    rng_offset: f32,
    time: f32,
    bound_diff: Vec2,
    bound_min: Vec2,
    gap: f32,
    check_valid: T,
) -> Vec2 {
    let bound_diff = bound_diff - gap * 2.;
    let mut offset = rng_offset;
    let mut position = Vec2::new(
        simplex_noise_2d(Vec2::new(time - offset * 15., (time + 5.) * offset * 5.)),
        simplex_noise_2d(Vec2::new(offset + time, time * 3. - offset * 45.)),
    );
    let mut tests = 0;

    loop {
        if check_valid(position) || tests >= MAX_POSITION_TESTS {
            break;
        }
        tests += 1;
        position = Vec2::new(
            simplex_noise_2d(Vec2::new(time - offset * 15., (time + 5.) * offset * 5.)),
            simplex_noise_2d(Vec2::new(offset + time, time * 3. - offset * 45.)),
        );
        offset = (position.x * position.y + position.y / 2.) * 1000.;
    }

    position.abs() * bound_diff + bound_min + gap
}

pub fn spawn_goal(
    mut events: EventReader<LevelEvent>,
    mut commands: Commands,
    bounds: Res<LevelBoundary>,
    existing_bodies: Query<(&GlobalTransform, &gravity::GravitationalBody)>,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut rng: ResMut<GlobalRng>,
    mut goal_status: ResMut<GoalStatus>,
) {
    if events.is_empty() {
        return;
    }

    let time = time.elapsed_seconds();
    let offset = 670.;
    let bound_diff = bounds.max - bounds.min;

    for event in events.iter() {
        if !matches!(
            event,
            LevelEvent::PickupCollected(PickupType::Goal) | LevelEvent::LevelStarted
        ) {
            continue;
        }
        let position = get_spawn_position(offset, time, bound_diff, bounds.min, GOAL_GAP, |p| {
            for (t, b) in existing_bodies.iter() {
                if p.distance(t.translation().xy()) < b.1 + GOAL_GAP {
                    return false;
                }
            }
            true
        });

        let goal_type = rng.sample(&GOAL_TYPES).unwrap();
        goal_status.current = *goal_type;
        let asset = match goal_type {
            GoalType::Chips => &assets.chips,
            GoalType::Fruit => &assets.fruit,
            GoalType::Gas => &assets.gas,
            GoalType::Post => &assets.post,
            GoalType::ToiletPaper => &assets.toilet_paper,
        }
        .clone();

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::ONE * 50.),
                    ..Default::default()
                },
                texture: asset,
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
                ..default()
            },
            pickup::Pickup(30., pickup::PickupType::Goal),
            LevelEntity,
        ));
        break;
    }
}

pub fn spawn_planet(
    mut events: EventReader<LevelEvent>,
    mut commands: Commands,
    bounds: Res<LevelBoundary>,
    existing_bodies: Query<(&GlobalTransform, &gravity::GravitationalBody)>,
    time: Res<Time>,
    assets: Res<GameAssets>,
) {
    if events.is_empty() {
        return;
    }

    let time = time.elapsed_seconds();
    let offset = 58.;
    let bound_diff = bounds.max - bounds.min;

    for event in events.iter() {
        if !matches!(
            event,
            LevelEvent::PickupCollected(PickupType::Goal) | LevelEvent::LevelStarted
        ) {
            continue;
        }
        let position = get_spawn_position(offset, time, bound_diff, bounds.min, 0., |p| {
            for (t, b) in existing_bodies.iter() {
                if p.distance(t.translation().xy()) < b.1 + PLANET_GAP {
                    return false;
                }
            }
            true
        });

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::ONE * 50.),
                    ..Default::default()
                },
                texture: assets.large_planet.clone(),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
                ..default()
            },
            gravity::GravitationalBody(10000., 30.),
            gravity::GravitationTransform::Static,
            LevelEntity,
            DelayedActivity(3.),
        ));
        break;
    }
}

const PICKUP_PROBABILITY: f32 = 0.1;

pub fn spawn_pickup(
    mut commands: Commands,
    event: EventReader<LevelEvent>,
    assets: Res<GameAssets>,
    time: Res<Time>,
    bounds: Res<LevelBoundary>,
    existing_bodies: Query<(&GlobalTransform, &gravity::GravitationalBody)>,
    mut rng: ResMut<GlobalRng>,
) {
    if event.is_empty() {
        return;
    }
    let time = time.elapsed_seconds();
    let offset = 162.;
    let bound_diff = bounds.max - bounds.min;
    let position = get_spawn_position(offset, time, bound_diff, bounds.min, 0., |p| {
        for (t, b) in existing_bodies.iter() {
            if p.distance(t.translation().xy()) < b.1 + PLANET_GAP {
                return false;
            }
        }
        true
    });

    let probability = rng.f32_normalized().abs();
    if probability.abs() > PICKUP_PROBABILITY {
        return;
    }
    let pickup = *rng.sample(&PICKUPS).unwrap();

    let (pickup, image) = match pickup {
        PickupType::PlanetKiller => (
            Pickup(30., PickupType::PlanetKiller),
            assets.planet_killer_pickup.clone(),
        ),
        PickupType::Goal => (Pickup(30., PickupType::Goal), assets.goal.clone()),
    };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE * pickup.0 * 2.),
                ..Default::default()
            },
            texture: image,
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
            ..default()
        },
        pickup,
        crate::level::LevelEntity,
    ));
}
