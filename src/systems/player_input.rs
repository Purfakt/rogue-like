use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(InBackpack)]
#[read_component(Weapon)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    let Some(key) = *key else { return };
    let delta = match key {
        VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::A => Point::new(-1, 0),
        VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::D => Point::new(1, 0),
        VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::W => Point::new(0, -1),
        VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::S => Point::new(0, 1),
        VirtualKeyCode::G => {
            let (player, player_pos) = players.iter(ecs).map(|(entity, pos)| (*entity, *pos)).next().unwrap();
            collect_item(ecs, commands, player, player_pos)
        }
        VirtualKeyCode::Key1 => use_item(0, ecs, commands),
        VirtualKeyCode::Key2 => use_item(1, ecs, commands),
        VirtualKeyCode::Key3 => use_item(2, ecs, commands),
        VirtualKeyCode::Key4 => use_item(3, ecs, commands),
        VirtualKeyCode::Key5 => use_item(4, ecs, commands),
        VirtualKeyCode::Key6 => use_item(5, ecs, commands),
        VirtualKeyCode::Key7 => use_item(6, ecs, commands),
        VirtualKeyCode::Key8 => use_item(7, ecs, commands),
        VirtualKeyCode::Key9 => use_item(8, ecs, commands),
        _ => Point::zero(),
    };

    let (player_entity, destination) = players
        .iter(ecs)
        .map(|(entity, pos)| (*entity, *pos + delta))
        .next()
        .unwrap();

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    if delta.x != 0 || delta.y != 0 {
        let mut hit_something = false;
        enemies
            .iter(ecs)
            .filter(|(_entity, pos)| **pos == destination)
            .for_each(|(entity, _pos)| {
                hit_something = true;
                commands.push((
                    (),
                    WantsToAttack {
                        attacker: player_entity,
                        victim: *entity,
                    },
                ));
            });
        if !hit_something {
            commands.push((
                (),
                WantsToMove {
                    entity: player_entity,
                    destination,
                },
            ));
        }
    }

    *turn_state = TurnState::PlayerTurn;
}

fn collect_item(ecs: &mut SubWorld, commands: &mut CommandBuffer, player: Entity, player_pos: Point) -> Point {
    let mut items = <(Entity, &Item, &Point)>::query();
    items
        .iter(ecs)
        .filter(|(_entity, _item, &item_pos)| item_pos == player_pos)
        .for_each(|(entity, _item, _item_pos)| {
            commands.remove_component::<Point>(*entity);
            commands.add_component(*entity, InBackpack { owner: player });

            if let Ok(e) = ecs.entry_ref(*entity) {
                if e.get_component::<Weapon>().is_ok() {
                    <(Entity, &InBackpack, &Weapon)>::query()
                        .iter(ecs)
                        .filter(|(_entity, in_backpack, _weapon)| in_backpack.owner == player)
                        .for_each(|(e, _in_backpack, _weapon)| {
                            commands.remove(*e);
                        })
                }
            }
        });
    Point::zero()
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, _player)| *entity)
        .next()
        .unwrap();
    let item_entity = <(Entity, &Item, &InBackpack)>::query()
        .iter(ecs)
        .filter(|(_entity, _item, in_backpack)| in_backpack.owner == player_entity)
        .enumerate()
        .filter(|(item_count, (_entity, _item, _in_backpack))| *item_count == n)
        .map(|(_item_count, (item_entity, _item, _in_backpack))| *item_entity)
        .next();
    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }
    Point::zero()
}
