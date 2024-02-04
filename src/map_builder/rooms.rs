use super::MapArchitect;
use crate::prelude::*;

pub struct RoomsArchitect;

impl MapArchitect for RoomsArchitect {
    fn builder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        builder.fill(TileType::Wall);
        builder.build_random_rooms(rng);
        builder.build_corridors(rng);
        builder.player_start = builder.rooms[0].center();
        builder.amulet_start = builder.find_most_distant();
        for room in builder.rooms.iter().skip(1) {
            builder.monster_spawns.push(room.center());
        }

        builder
    }
}
