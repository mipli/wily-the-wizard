extern crate rand;

use rand::Rng;

use spawning_pool::EntityId;

use utils;
use map;

use point::*;
use actions::*;
use game::*;
use components;

pub fn confusion(entity: EntityId, game_state: &mut GameState) -> Option<Action> {
    let action = get_confusion_action(entity, game_state);

    if action.is_some() {
        reduce_timer(entity, game_state);
    }

    action
}

fn get_confusion_action(entity: EntityId, game_state: &mut GameState) -> Option<Action> {
    let status_effects = game_state.spawning_pool.get::<components::StatusEffects>(entity)?;
    let _ = status_effects.confused?;
    let entity_position = get_entity_position(entity, game_state)?;
    let mut neighbours = utils::get_neigbours(entity_position.x, entity_position.y);
    rand::thread_rng().shuffle(&mut neighbours);
    for n in neighbours {
        if map::can_walk(n, &game_state.spatial_table, &game_state.map) {
            let (x, y) = entity_position.direction_to(n);
            return Some(Action {
                    actor: Some(entity),
                    target: None,
                    command: Command::WalkDirection{dir: Point::new(x, y)}
                });
        }
    }
    None
}

fn reduce_timer(entity: EntityId, game_state: &mut GameState) {
    if let Some(status_effects) = game_state.spawning_pool.get_mut::<components::StatusEffects>(entity) {
        if let Some(confused) = status_effects.confused {
            let remain = confused - 1;
            status_effects.confused =  if remain > 0 {
                Some(remain)
            } else {
                None
            };
        }
    }
}
