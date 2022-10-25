use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};

use std::collections::HashSet;

use bevy_rapier2d::prelude::*;

#[derive(Default, Component)]
struct Walls;

#[derive(Default, Component)]
struct Tiles;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
}

pub fn ldtk_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("./level1.ldtk"),
        ..Default::default()
    });
}

// setting up the LDTK bundle
#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: Walls,
    b: Tiles,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        match entity_instance.identifier.as_ref() {
            "Tiles" => ColliderBundle {
                collider: Collider::cuboid(3.0, 3.0),
            },
        }
    }
}
pub fn collisions(mut commands: Commands) {
    // TODO
    commands
        .spawn()
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(Tiles);
}
