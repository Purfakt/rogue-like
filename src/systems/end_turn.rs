use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_hp_and_pos = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();
    let current_state = turn_state.clone();

    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    player_hp_and_pos.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        } else if amulet_pos == pos {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}
