use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(InBackpack)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        let is_player = ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok();
        let base_damage = ecs
            .entry_ref(*attacker)
            .map(|a| match a.get_component::<Damage>() {
                Ok(damage) => damage.0,
                Err(_) => 0,
            })
            .unwrap_or(0);
        let weapon_damage: i32 = <(&InBackpack, &Damage)>::query()
            .iter(ecs)
            .filter(|(item, _)| item.owner == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();
        let damage = base_damage + weapon_damage;

        if let Ok(health) = ecs.entry_mut(*victim).unwrap().get_component_mut::<Health>() {
            health.current -= damage;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    });
}
