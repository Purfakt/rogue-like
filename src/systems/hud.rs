use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(InBackpack)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).next().unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // Display message
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");

    // Display Health
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {} ", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    // Display Inventory
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, _)| *entity)
        .next()
        .unwrap();

    let mut item_query = <(&Item, &Name, &InBackpack)>::query();
    let mut y = 3;
    item_query
        .iter(ecs)
        .filter(|(_, _, backpack)| backpack.owner == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });

    if y > 3 {
        draw_batch.print(Point::new(3, 2), "Items carried");
    }

    // Display level
    let (_, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, player)| (*entity, player.map_level))
        .next()
        .unwrap();

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    // Submit the batch
    draw_batch.submit(10000).expect("Batch error");
}
