extern crate rand;

use rand::Rng;

use utils;
use map;
use components;

use point::*;
use rules::definitions::*;
use actions::*;
use game::*;

pub fn confused(action: &mut Action, game_state: &GameState, _rejected_actions: &mut Vec<Action>, _reaction_actions: &mut Vec<Action>) -> (ActionStatus, RuleStatus) {
    match action.command {
        Command::WalkDirection{..} => {
            if let Some(actor) = action.actor {
                if let Some(status_effects) = game_state.spawning_pool.get::<components::StatusEffects>(actor) {
                    if status_effects.confused.is_some() {
                        if let Some(entity_position) = get_entity_position(actor, game_state) {
                            let mut neighbours = utils::get_neigbours(entity_position.x, entity_position.y);
                            rand::thread_rng().shuffle(&mut neighbours);
                            for n in neighbours {
                                if map::can_walk(n, &game_state.spatial_table, &game_state.map) {
                                    let (x, y) = entity_position.direction_to(n);
                                    action.command = Command::WalkDirection{dir: Point::new(x, y)};
                                }
                            }
                        }
                    }
                }
            }
            (ActionStatus::Accept, RuleStatus::Continue)
        },
        _ => (ActionStatus::Accept, RuleStatus::Continue)
    }
}
