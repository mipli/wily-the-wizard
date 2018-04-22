extern crate rand;

use rand::Rng;

use spawning_pool::EntityId;

use utils;
use map;

use utils::*;
use messages::*;
use point::*;
use actions::*;
use game::*;
use components;

pub fn confusion(entity: EntityId, state: &mut GameState) -> Option<Action> {
    let action = get_confusion_action(entity, state);

    if action.is_some() {
        reduce_timer(entity, state);
    }

    action
}

fn get_confusion_action(entity: EntityId, state: &mut GameState) -> Option<Action> {
    let status_effects = state.spawning_pool.get::<components::StatusEffects>(entity)?;
    let _ = status_effects.confused?;
    let entity_position = get_entity_position(entity, state)?;
    let mut neighbours = utils::get_neigbours(entity_position.x, entity_position.y);
    rand::thread_rng().shuffle(&mut neighbours);
    for n in neighbours {
        if map::can_walk(n, &state.spatial_table, &state.map) {
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

fn reduce_timer(entity: EntityId, state: &mut GameState) {
    let name = utils::get_entity_name(entity, &state.spawning_pool);
    if let Some(status_effects) = state.spawning_pool.get_mut::<components::StatusEffects>(entity) {
        if let Some(confused) = status_effects.confused {
            let remain = confused - 1;
            status_effects.confused =  if remain > 0 {
                Some(remain)
            } else {
                state.messages.log(MessageLevel::Info, format!("The {} is longer confused", name));
                None
            };
        }
    }
}
