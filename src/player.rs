use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component)]
pub struct Player(f32); 


pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity_spawn = Vec3::new(1.0, 1.0, 2.0);


    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("character.png"),
            transform: Transform::from_translation(entity_spawn).with_scale(Vec3::splat(6.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(Camera2dBundle {
                transform: Transform::from_scale(Vec3::splat(0.08)),
                ..default()
            }); // setting up the scale of the camera
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(Velocity::zero())
        .insert(Player(90.0) 
       );
}

pub fn player_physics(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rb_vels) in &mut player_info {
        let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
        let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * player.0;
    }
}

