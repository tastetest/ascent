use bevy::prelude::*;
use bevy::render::texture::*;
use rapier2d::prelude::*;

mod player;
mod texture_atlas;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(player::setup_player)
        .add_system(player::player_physics)
        .add_system(bevy::window::close_on_esc)
        //        .add_system(texture_atlas::setup)
        .add_startup_system(setup_camera)
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
