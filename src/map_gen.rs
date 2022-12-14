use bevy::{
	prelude::*,
};

pub(crate) mod room_gen;
pub(crate) mod delaunay;
pub(crate) mod mst;
pub(crate) mod pathfinding;
pub(crate) mod random_objs;
use room_gen::room_generator;
use delaunay::triangulate;
use mst::prims;
use pathfinding::hallway;
use random_objs::place_objects;

use crate::{
	GameState,
};

pub struct RoomGenPlugin;

impl Plugin for RoomGenPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(map_generator.label("map_gen"))
        .add_system_set(SystemSet::on_update(GameState::Overworld))
		.add_system_set(SystemSet::on_enter(GameState::Overworld))
		.add_system_set(SystemSet::on_exit(GameState::Overworld))
        ;
    }
}


#[derive(Component)]
pub struct Room {
    pub size: Vec2,
	pub id: i32,
    pub center: Vec3,
}

impl Room {
	fn new(size: Vec2, id: i32, center: Vec3) -> Room {
		Room {
			size,
			id,
            center,
		}
	}
}

#[derive(Component)]
pub struct Edge (pub Vec2, pub Vec2);

#[derive(Component)]
pub struct BlockPath(pub Vec2);

#[derive(Component)]
pub struct StandPath(pub Vec2);

fn map_generator(
    mut commands: Commands,
) {
    let (centers, sizes) = room_generator(&mut commands);

    let final_polygon = triangulate(&centers);     // DELAUNAY
	
    let final_polygon = prims(&final_polygon);

    for edge in final_polygon.iter() {
       commands.spawn()
           .insert(Edge(edge.0, edge.1));
    }

	hallway(&centers, &sizes, &final_polygon, &mut commands);

	place_objects(&centers,&sizes,&mut commands);
}
