use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
",
    12,
    11,
);

pub fn apply_prefab(builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &[builder.map.point2d_to_index(builder.player_start)],
        &builder.map,
        1024.0,
    );

    let mut attempts = 0;

    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.1),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );
        let mut can_place = false;

        dimensions.for_each(|pt| {
            let idx = builder.map.point2d_to_index(pt);
            let distance = dijkstra_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && builder.amulet_start != pt {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            builder.monster_spawns.retain(|pt| !points.contains(pt));
        }

        attempts += 1;
    }

    if let Some(placement) = placement {
        println!("Fortress placed at {:?}", placement);
        let string_vec: Vec<char> = FORTRESS.0.chars().filter(|a| *a != '\r' && *a != '\n').collect();
        let mut i = 0;
        for ty in placement.y..placement.y + FORTRESS.2 {
            for tx in placement.x..placement.x + FORTRESS.1 {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];
                match c {
                    '-' => {
                        builder.map.tiles[idx] = TileType::Floor;
                    }
                    '#' => {
                        builder.map.tiles[idx] = TileType::Wall;
                    }
                    'M' => {
                        builder.map.tiles[idx] = TileType::Floor;
                        builder.monster_spawns.push(Point::new(tx, ty));
                    }
                    _ => {
                        println!("Unknown glyph {}", c);
                    }
                }
                i += 1;
            }
        }
    }
}
