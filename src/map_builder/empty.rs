use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect;

impl MapArchitect for EmptyArchitect {
    fn builder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };
        builder.fill(TileType::Floor);
        builder.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        builder.amulet_start = builder.find_most_distant();
        for _ in 0..50 {
            builder
                .monster_spawns
                .push(Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)))
        }
        builder
    }
}
