use spawning_pool::{EntityId};
use game::*;
use actions::*;

mod behaviour;

pub fn perform_basic_ai(actor: EntityId, state: &mut GameState) -> Option<Vec<Action>> {
    use components::*;
    match behaviour::melee_attack_entity(actor, state.player, state) {
        Some(actions) => Some(actions),
        None => {
            if !behaviour::can_see_entity(actor, state.player, state) {
                match behaviour::recall_player_position(actor, state) {
                    Some(position) => {
                        match behaviour::walk_to_position(actor, position, state) {
                            Some(actions) => Some(actions),
                            None => behaviour::wait_and_forget(actor, state)
                        }
                    },
                    None => behaviour::wait_and_forget(actor, state)
                }
            } else {
                let player_position = get_entity_position(state.player, state)?;
                if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
                    mem.player_position = Some(player_position);
                }
                match behaviour::walk_to_position(actor, player_position, state) {
                    Some(actions) => Some(actions),
                    None => behaviour::wait_and_forget(actor, state)
                }
            }
        }
    }
}

pub fn perform_spell_ai(actor: EntityId, state: &mut GameState) -> Option<Vec<Action>> {
    use components::*;
    let actor_position = get_entity_position(actor, state)?;
    if !behaviour::can_see_entity(actor, state.player, state) {
        match behaviour::recall_player_position(actor, state) {
            Some(position) => {
                match behaviour::walk_to_position(actor, position, state) {
                    Some(actions) => Some(actions),
                    None => behaviour::wait_and_forget(actor, state)
                }
            },
            None => behaviour::wait_and_forget(actor, state)
        }
    } else {
        let player_position = get_entity_position(state.player, state)?;
        if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
            mem.player_position = Some(player_position);
        }
        if actor_position.distance(player_position) < 3.0 {
            match behaviour::walk_to_away_from(actor, player_position, state) {
                Some(actions) => Some(actions),
                None => {
                    match behaviour::cast_spell_at(actor, state.player, state) {
                        Some(actions) => Some(actions),
                        None => behaviour::wait_and_forget(actor, state)
                    }
                }
            }
        } else {
            match behaviour::cast_spell_at(actor, state.player, state) {
                Some(actions) => Some(actions),
                None => {
                    match behaviour::walk_to_position(actor, player_position, state) {
                        Some(actions) => Some(actions),
                        None => behaviour::wait_and_forget(actor, state)
                    }
                }
            }
        }
    }
}
