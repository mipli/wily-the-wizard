use std::cmp::max;

use rules::definitions::*;
use actions::*;
use game::*;
use components;

pub fn attack_entity(action: &mut Action, game_state: &GameState, rejected_actions: &mut Vec<Action>, _reaction_actions: &mut Vec<Action>) -> (ActionStatus, RuleStatus) {
    match action.command {
        Command::AttackEntity{bonus_strength, bonus_defense} => {
            if let Some(actor) = action.actor {
                let target_id = match action.target {
                    Some(id) => id,
                    _ => unreachable!()
                };

                let strength = match game_state.spawning_pool.get::<components::Stats>(actor) {
                    Some(stats) => stats.strength,
                    None => 0 
                };
                let target_defense = bonus_defense + match game_state.spawning_pool.get::<components::Stats>(target_id) {
                    Some(stats) => stats.defense,
                    None => 0 
                };
                let attack_strength = bonus_strength + strength;

                let damage = max(1, attack_strength - target_defense);

                rejected_actions.push(Action{
                    target: Some(target_id),
                    actor: action.actor,
                    command: Command::TakeDamage{damage}
                });
            }
            (ActionStatus::Reject, RuleStatus::Stop)
        },
        _ => (ActionStatus::Accept, RuleStatus::Continue)
    }
}
