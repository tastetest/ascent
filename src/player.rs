use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player(f32);

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;

    let entity_spawn = Vec3::ZERO;

    commands.spawn().insert_bundle(Camera2dBundle::default());

    let sprite_size = 50.0;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("character.png"),
            transform: Transform::from_translation(entity_spawn).with_scale(Vec3::splat(5.0)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::ball(5.0))
        .insert(Player(100.0));
}

pub fn player_physics(
    mut query: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
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
