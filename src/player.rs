use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player(f32);

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
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Player(3.0));
}

pub fn player_physics(
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length() * time.delta_seconds();
        }

        rb_vels.linvel = move_delta * player.0;
    }
}
