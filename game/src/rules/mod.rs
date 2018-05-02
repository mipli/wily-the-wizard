use actions::*;
use game::*;

mod items;
mod spells;
mod definitions;
mod collision;
mod attack;

pub use self::definitions::*;

pub fn apply_rules(action: &mut Action, game_state: &GameState, rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> ActionStatus {
    let rules = [
        spells::validate_spell,
        items::use_item,
        items::apply_equipment_bonus,
        collision::collision,
        spells::cast_spell,
        spells::lightning_strike,
        attack::attack,
        attack::take_damage,
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
