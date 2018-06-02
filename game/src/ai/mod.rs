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
    use components::*;
    println!("\nActor: {}", actor);
    let actual_player_position = get_entity_position(state.player, state)?;
    println!("Actual position: {:?}", actual_player_position);
    let is_visible = state.spawning_pool.get::<MapMemory>(actor)?.is_visible(actual_player_position.x, actual_player_position.y);
    println!("Is visible: {:?}", is_visible);
    let actor_position = get_entity_position(actor, state)?;
    println!("Actor position: {:?}", actor_position);
    match behaviour::get_player_position(actor, state) {
        Some(p) => {
            println!("behaviour pos: {:?}", p);
        },
        None => {
            println!("behaviour pos: None");
        }
    };
    let player_position = behaviour::get_player_position(actor, state)?;
    println!("Player position: {:?}", player_position);
    if !is_visible {
        println!("walking towards");
        return match behaviour::walk_to_position(actor, player_position, state) {
            Some(actions) => Some(actions),
            None => behaviour::wait_and_forget(actor, state)
        };
    }
    if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
        mem.path_goal = Some(player_position);
    }

    if actor_position.distance(player_position) > 3.0 {
        match behaviour::cast_spell_at(actor, state.player, state) {
            Some(actions) => Some(actions),
            None => {
                println!("Trying to walk to: {:?}", player_position);
                match behaviour::walk_to_position(actor, player_position, state) {
                    Some(actions) => Some(actions),
                    None => behaviour::wait_and_forget(actor, state)
                }
            }
        }
    } else {
        match behaviour::walk_to_away_from(actor, player_position, state) {
            Some(actions) => Some(actions),
            None => {
                match behaviour::cast_spell_at(actor, state.player, state) {
                    Some(actions) => Some(actions),
                    None => behaviour::wait_and_forget(actor, state)
                }
            }
        }
    }
}
