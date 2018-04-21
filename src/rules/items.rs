use utils;
use actions::*;
use game::*;
use components;

use rules::definitions::*;

pub fn use_item(action: &mut Action, game_state: &GameState, _rejected_actions: &mut Vec<Action>, _reaction_actions: &mut Vec<Action>) -> (ActionStatus, RuleStatus) {
    match action.command {
        Command::UseItem{item_id} => {
            let item_comp = game_state.spawning_pool.get::<components::Item>(item_id);

            if item_comp.is_none() {
                return (ActionStatus::Reject, RuleStatus::Stop);
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
