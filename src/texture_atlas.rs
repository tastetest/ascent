use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn create_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO
    let file = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(""),
        ..default()
    });

    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {}
            }
        }
    }
}
