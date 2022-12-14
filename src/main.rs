use bevy::prelude::*;
use bevy::render::texture::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

mod ldtk;
mod player;
// mod texture_atlas;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Ascent".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(ldtk::ldtk_setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<ldtk::MyBundle>("Level_1")
        .add_startup_system(gravity_setup)
        .add_startup_system(player::setup_player)
        .register_ldtk_int_cell::<ldtk::ColliderBundle>(2)
        .add_system(player::player_physics)
        .add_system(bevy::window::close_on_esc)
        // .add_system(texture_atlas::setup)
        .run()
}

pub fn gravity_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
