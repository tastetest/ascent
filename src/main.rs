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
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(player::setup_player)
        .add_startup_system(ldtk::ldtk_setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<ldtk::MyBundle>("Tiles")
        .add_system(player::player_physics)
        .add_system(bevy::window::close_on_esc)
        // .add_system(texture_atlas::setup)
        .run()
}
