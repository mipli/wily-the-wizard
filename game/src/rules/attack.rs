use std::cmp::max;

use rules::definitions::*;
use game::*;
use actions::*;
use components;

pub fn attack(action: &mut Action, state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> (ActionStatus, RuleStatus) {
    if let Command::AttackEntity{bonus_strength, bonus_defense} = action.command {
        if let Some(actor) = action.actor {
            let target_id = match action.target {
                Some(id) => id,
                _ => unreachable!()
            };

            let strength = match state.spawning_pool.get::<components::Stats>(actor) {
                Some(stats) => stats.strength,
                None => 0 
            };
            let target_defense = bonus_defense + match state.spawning_pool.get::<components::Stats>(target_id) {
                Some(stats) => stats.defense,
                None => 0 
            };
            let attack_strength = bonus_strength + strength;

            let damage = max(1, attack_strength - target_defense);

            reaction_actions.push(Action{
                target: Some(target_id),
                actor: action.actor,
                command: Command::TakeDamage{damage}
            });
        }
    }
    (ActionStatus::Accept, RuleStatus::Continue)
}

pub fn take_damage(action: &mut Action, state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> (ActionStatus, RuleStatus) {
    if let Command::TakeDamage{damage} = action.command {
        if let Some(target) = action.target {
            if let Some(mut stats) = state.spawning_pool.get::<components::Stats>(target) {
                let health = stats.health - damage;
                if health <= 0 {
                    reaction_actions.push(Action{
                        actor: Some(target),
                        target: None,
                        command: Command::KillEntity
                    });
                }
            } else {
                return (ActionStatus::Reject, RuleStatus::Stop);
            }
        }
    }
    (ActionStatus::Accept, RuleStatus::Continue)
}
