use spawning_pool::{EntityId};
use point::*;
use game::*;
use actions::*;
use path;

pub fn perform_basic_ai(actor: EntityId, game_state: &GameState) -> Option<Vec<Action>> {
    if let Some(entity_position) = get_entity_position(actor, game_state) {
        if let Some(player_position) = get_entity_position(game_state.player, game_state) {
            if entity_position.tile_distance(player_position) > 5 {
                return Some(vec![Action {
                    actor: Some(actor),
                    target: None,
                    command: Command::Wait
                }]);
            }

            match path::path(entity_position, player_position, &game_state.spatial_table, &game_state.map) {
                Some(mut path) => {
                    let next = path.pop().unwrap();
                    let dir = Point::new(next.x - entity_position.x, next.y - entity_position.y);
                    return Some(vec![Action {
                            actor: Some(actor),
                            target: None,
                            command: Command::WalkDirection{dir}
                        }]);
                },
                None => {
                    return Some(vec![Action {
                        actor: Some(actor),
                        target: None,
                        command: Command::Wait
                    }]);
                }
            }

        }
    }
    Some(vec![Action {
        actor: Some(actor),
        target: None,
        command: Command::Wait
    }])
}
