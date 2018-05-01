use actions::*;
use game::*;
use utils;

mod items;
mod definitions;
mod collision;

pub use self::definitions::*;

pub fn apply_rules(action: &mut Action, game_state: &GameState, rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> ActionStatus {
    let rules = [
        items::use_item,
        items::apply_equipment_bonus,
        collision::collision,
        validate_spell
    ];
    if action.command == Command::Abort {
        return ActionStatus::Reject;
    }
    let mut action_status = ActionStatus::Accept;  
    for rule in &rules {
        let (s, rule_status) = rule(action, game_state, rejected_actions, reaction_actions);
        action_status = s;
        if rule_status == RuleStatus::Stop || action_status == ActionStatus::Reject {
            break;
        }
    }
    action_status
}

fn validate_spell(action: &mut Action, state: &GameState, _rejected_actions: &mut Vec<Action>, _reaction_actions: &mut Vec<Action>) -> (ActionStatus, RuleStatus) {
    use components::*;
    if let Command::CastSpell{ref spell} = action.command {
        if let Some(actor) = action.actor {
            if state.spawning_pool.get::<Physics>(actor).is_none() {
               return  (ActionStatus::Reject, RuleStatus::Stop);
            }
        }
        if let Some(target) = action.target {
            if state.spawning_pool.get::<Physics>(target).is_none() {
               return  (ActionStatus::Reject, RuleStatus::Stop);
            }
        }
        if let Some(actor) = action.actor {
            if let Some(target) = action.target {
                let actor_position = utils::get_position(actor, &state.spawning_pool).unwrap();
                let target_position = utils::get_position(target, &state.spawning_pool).unwrap();
                let distance = actor_position.distance(target_position);
                if distance > spell.range as f32 {
                    return  (ActionStatus::Reject, RuleStatus::Stop);
                }
            }
        }
    }
    (ActionStatus::Accept, RuleStatus::Continue)
}
