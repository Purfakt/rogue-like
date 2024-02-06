mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_system: Schedule,
    player_system: Schedule,
    monster_system: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        reset_game(&mut ecs, &mut resources);
        Self {
            ecs,
            resources,
            input_system: build_input_scheduler(),
            player_system: build_player_scheduler(),
            monster_system: build_monster_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(5, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.ecs = World::default();
            reset_game(&mut self.ecs, &mut self.resources);
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            reset_game(&mut self.ecs, &mut self.resources);
        }
    }

    fn next_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .next()
            .unwrap();

        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &InBackpack)>::query()
            .iter(&self.ecs)
            .filter(|(_, backpack)| backpack.owner == player_entity)
            .for_each(|(&entity, _)| {
                entities_to_keep.insert(entity);
                println!("Keeping entity: {:?}", entity);
            });

        // Remove everything that isn't marked to keep
        let mut command_buffer = CommandBuffer::new(&self.ecs);
        Entity::query().iter(&self.ecs).for_each(|entity| {
            if !entities_to_keep.contains(entity) {
                command_buffer.remove(*entity);
                println!("Removing entity: {:?}", entity);
            }
        });

        command_buffer.flush(&mut self.ecs, &mut self.resources);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        // Spawn stairs or amulet
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });
        if map_level == 2 {
            spawn_amulet(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_level(&mut self.ecs, &mut rng, map_level as usize, &map_builder.monster_spawns);
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
}

fn reset_game(ecs: &mut World, resources: &mut Resources) {
    let mut rng = RandomNumberGenerator::new();
    let mut map_builder = MapBuilder::new(&mut rng);
    spawn_player(ecs, map_builder.player_start);
    let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
    map_builder.map.tiles[exit_idx] = TileType::Exit;
    spawn_level(ecs, &mut rng, 0, &map_builder.monster_spawns);
    resources.insert(map_builder.map);
    resources.insert(Camera::new(map_builder.player_start));
    resources.insert(TurnState::AwaitingInput);
    resources.insert(map_builder.theme);
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => self.input_system.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_system.execute(&mut self.ecs, &mut self.resources),
            TurnState::EnemyTurn => self.monster_system.execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.next_level(),
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;
    main_loop(context, State::new())
}
