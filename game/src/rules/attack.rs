use std::cmp::max;

use crate::rules::definitions::*;
use crate::game::*;
use crate::actions::*;
use crate::components;

pub fn attack(action: &mut Action, state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> ActionStatus {
    if let Command::AttackEntity{bonus_strength, bonus_defense} = action.command {
        if let Some(actor) = action.actor {
            let target_id = match action.target {
                Some(ActionTarget::Entity(id)) => id,
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

            reaction_actions.push(Action::new(
                action.actor,
                Some(ActionTarget::Entity(target_id)),
                Command::TakeDamage{damage}
            ));
        }
    }
    ActionStatus::Accept
}

pub fn take_damage(action: &mut Action, state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> ActionStatus {
    if let Command::TakeDamage{damage} = action.command {
        if let Some(ActionTarget::Entity(target)) = action.target {
            if let Some(stats) = state.spawning_pool.get::<components::Stats>(target) {
                let health = stats.health - damage;
                if health <= 0 {
                    reaction_actions.push(Action::new(
                        Some(target),
                        None,
                        Command::KillEntity
                    ));
                }
            } else {
                return ActionStatus::Reject;
            }
        }
    }
    ActionStatus::Accept
}
