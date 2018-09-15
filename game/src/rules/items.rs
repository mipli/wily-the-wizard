use crate::utils;
use crate::actions::*;
use crate::game::*;
use crate::components;
use crate::spells;

use crate::rules::definitions::*;

pub fn use_item(action: &mut Action, game_state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> ActionStatus {
    match action.command {
        Command::UseItem{item_id} => {
            let item_comp = game_state.spawning_pool.get::<components::Item>(item_id);

            if item_comp.is_none() {
                return ActionStatus::Reject;
            }

            if let Some(item) = game_state.spawning_pool.get::<components::Item>(item_id) {
                if let Some(on_use) = get_callback(action, &game_state.spawning_pool) {
                    match on_use {
                        components::OnUseCallback::Spell(spell) => {
                            reaction_actions.push(Action::new(
                                action.actor,
                                None,
                                Command::DestroyItem{item_id}
                            ));
                            let mut act = Action::new(
                                action.actor,
                                None,
                                Command::CastSpell{spell: spells::Spell::create(spell)}
                            );
                            act.set_time = Some(50);
                            if item.kind == components::ItemKind::Potion {
                                act.target = Some(ActionTarget::Entity(action.actor.unwrap()));
                            }
                            reaction_actions.push(act);
                        }
                    }
                }
            }
            ActionStatus::Accept
        },
        _ => ActionStatus::Accept
    }
}

pub fn apply_equipment_bonus(action: &mut Action, game_state: &GameState, _rejected_actions: &mut Vec<Action>, _reaction_actions: &mut Vec<Action>) -> ActionStatus {
    match action.command {
        Command::AttackEntity{..} => {
            if let Some(actor) = action.actor {
                if let Some(ActionTarget::Entity(target)) = action.target {
                    action.command = Command::AttackEntity {
                        bonus_strength: utils::get_strength_bonus(actor, &game_state.spawning_pool),
                        bonus_defense: utils::get_defense_bonus(target, &game_state.spawning_pool)
                    };
                    return ActionStatus::Accept;
                }
            }
            ActionStatus::Accept
        },
        _ => ActionStatus::Accept
    }
}

fn get_callback(action: &Action, spawning_pool: &components::SpawningPool) -> Option<components::OnUseCallback> {
    let item_id = match action.command {
        Command::UseItem{item_id} => Some(item_id),
        _ => None
    };
    let item = match item_id {
        Some(item_id) => {
            match spawning_pool.get::<components::Item>(item_id) {
                Some(i) => Some(i),
                None => None
            }
        },
        None => None
    };
    if let Some(item) = item {
        return item.on_use.clone();
    }
    None
}
