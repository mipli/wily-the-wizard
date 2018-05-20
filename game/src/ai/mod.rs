use spawning_pool::{EntityId};
use game::*;
use actions::*;

mod behaviour;

pub fn perform_basic_ai(actor: EntityId, state: &mut GameState) -> Option<Vec<Action>> {
    match behaviour::melee_attack_entity(actor, state.player, state) {
        Some(actions) => Some(actions),
        None => {
            let player_position = behaviour::get_player_position(actor, state)?;
            match behaviour::walk_to_position(actor, player_position, state) {
                Some(actions) => Some(actions),
                None => behaviour::wait_and_forget(actor, state)
            }
        }
    }
}

pub fn perform_spell_ai(actor: EntityId, state: &mut GameState) -> Option<Vec<Action>> {
    match behaviour::cast_spell_at(actor, state.player, state) {
        Some(actions) => Some(actions),
        None => perform_basic_ai(actor, state)
    }
}
