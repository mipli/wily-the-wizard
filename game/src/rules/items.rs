use utils;
use actions::*;
use game::*;
use components;
use spells;

use rules::definitions::*;

pub fn use_item(action: &mut Action, game_state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> (ActionStatus, RuleStatus) {
    match action.command {
        Command::UseItem{item_id} => {
            let item_comp = game_state.spawning_pool.get::<components::Item>(item_id);

            if item_comp.is_none() {
                return (ActionStatus::Reject, RuleStatus::Stop);
            }

            if let Some(item) = game_state.spawning_pool.get::<components::Item>(item_id) {
                if let Some(on_use) = get_callback(action, &game_state.spawning_pool) {
                    match on_use {
                        components::OnUseCallback::Spell(spell) => {
                            reaction_actions.push(Action{
                                actor: action.actor,
                                target: None,
                                command: Command::DestroyItem{item_id}
                            });
                            let mut act = Action {
                                actor: action.actor,
                                target: None,
                                command: Command::CastSpell{spell: spells::Spell::create(spell)}
                            };
                            if item.kind == components::ItemKind::Potion {
                                act.target = action.actor;
                            }
                            reaction_actions.push(act);
                        }
                    }
                }
            }

            (ActionStatus::Accept, RuleStatus::Continue)
        },
        _ => (ActionStatus::Accept, RuleStatus::Continue)
    }
}

pub fn apply_equipment_bonus(action: &mut Action, game_state: &GameState, _rejected_actions: &mut Vec<Action>, _reaction_actions: &mut Vec<Action>) -> (ActionStatus, RuleStatus) {
    match action.command {
        Command::AttackEntity{..} => {
            if let Some(actor) = action.actor {
                if let Some(target) = action.target {
                    action.command = Command::AttackEntity {
                        bonus_strength: utils::get_strength_bonus(actor, &game_state.spawning_pool),
                        bonus_defense: utils::get_defense_bonus(target, &game_state.spawning_pool)
                    };
                    return (ActionStatus::Accept, RuleStatus::Continue);
                }
            }
            (ActionStatus::Accept, RuleStatus::Continue)
        },
        _ => (ActionStatus::Accept, RuleStatus::Continue)
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
