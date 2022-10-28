use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};

use std::collections::{HashMap, HashSet};

use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Clone, Debug)]
pub struct Walls;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    walls: Walls,
}

#[derive(Default, Component)]
struct Tiles;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
}

pub fn ldtk_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level1_handle = asset_server.load("./level1.ldtk");

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: level1_handle,
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

// impl From<EntityInstance> for ColliderBundle {
//     fn from(entity_instance: EntityInstance) -> ColliderBundle {
//         match entity_instance.identifier.as_ref() {
//             "Tiles" => ColliderBundle {
//                 collider: Collider::cuboid(3.0, 3.0),
//             },
//             "Walls" => ColliderBundle {
//                 collider: Collider::cuboid(2.0, 2.0),
//             },
//             _ => ColliderBundle::default(),
//         }
//     }
// }

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        if int_grid_cell.value == 2 {
            println!("this line has been reached");
            ColliderBundle {
                collider: Collider::cuboid(9.0, 9.0),
            }
        } else {
            ColliderBundle::default()
        }
    }
}
