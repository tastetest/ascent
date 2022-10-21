use bevy::prelude::*;

use bevy_ecs_tilemap::prelude::*;
mod helpers;

const QUADRANT_SIDE_LENGTH: u32 = 80;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("iso_color.png");
}
