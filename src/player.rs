use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity_spawn = Vec3::new(1.0, 1.0, 2.0);

    commands.spawn().insert_bundle(Camera2dBundle::default());

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("character.png"),
            transform: Transform::from_translation(entity_spawn).with_scale(Vec3::splat(5.0)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Player);
}
pub fn player_physics(
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut velocities: Query<&mut Velocity>,
) {
    if keys.pressed(KeyCode::W) {
        for mut transform in query.iter_mut() {
            transform.translation.y += 200.2 * time.delta_seconds();
        }
    }
    if keys.pressed(KeyCode::A) {
        for mut transform in query.iter_mut() {
            transform.translation.x += -200.2 * time.delta_seconds();
        }
    }
    if keys.pressed(KeyCode::S) {
        for mut transform in query.iter_mut() {
            transform.translation.y += -200.2 * time.delta_seconds();
        }
    }
    if keys.pressed(KeyCode::D) {
        for mut transform in query.iter_mut() {
            transform.translation.x += 200.2 * time.delta_seconds();
        }
    }
}
