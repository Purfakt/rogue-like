use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect;

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            *t = if rng.range(0, 100) > 55 {
                TileType::Floor
            } else {
                TileType::Wall
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut count = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[map_idx(x + ix, y + iy)] == TileType::Wall {
                    count += 1;
                }
            }
        }
        count
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                new_tiles[idx] = if neighbors > 4 || neighbors == 0 {
                    TileType::Wall
                } else {
                    TileType::Floor
                };
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, distance1), (_, distance2)| distance1.partial_cmp(distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        map.index_to_point2d(closest_point)
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn builder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::DungeonTheme::new(),
        };

        self.random_noise_map(rng, &mut builder.map);
        for _ in 0..10 {
            self.iteration(&mut builder.map);
        }

        let start = self.find_start(&builder.map);
        builder.monster_spawns = builder.spawn_monsters(&start, rng);
        builder.player_start = start;
        builder.amulet_start = builder.find_most_distant();

        builder
    }
}
