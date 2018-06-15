use rules::definitions::*;
use map::*;
use actions::*;
use game::*;
use components;

pub fn collision(action: &mut Action, game_state: &GameState, rejected_actions: &mut Vec<Action>, _reaction_actions: &mut Vec<Action>) -> ActionStatus {
    match action.command {
        Command::WalkDirection{dir} => {
            if let Some(actor) = action.actor {
                let solid = match game_state.spawning_pool.get::<components::Flags>(actor) {
                    Some(flags) => flags.solid,
                    None => false
                };

                if !solid {
                    return ActionStatus::Accept;
                }

                let pos = match game_state.spawning_pool.get::<components::Physics>(actor) {
                    Some(physics) => physics.coord,
                    None => panic!("Walk command initiated on entity without physics component")
                };

                let new_pos = pos + dir;
                let cell = game_state.map.get_cell(new_pos.x, new_pos.y);
                if cell.tile_type == TileType::Wall {
                    return ActionStatus::Reject;
                }

                if let Some(cell) = game_state.spatial_table.get(new_pos) {
                    if cell.solid {
                        for entity in &cell.entities {
                            let is_closed_door = match game_state.spawning_pool.get::<components::Door>(*entity) {
                                Some(door) => !door.opened,
                                None => false
                            };
                            if is_closed_door {
                                rejected_actions.push(Action {
                                    actor: action.actor,
                                    target: None,
                                    command: Command::OpenDoor{entity: *entity}
                                });
                                continue;
                            }
                            let actor_faction = match game_state.spawning_pool.get::<components::Information>(action.actor.unwrap()) {
                                Some(info) => Some(info.faction),
                                None => None
                            };
                            if actor_faction.is_none() {
                                continue;
                            }
                            let entity_faction = match game_state.spawning_pool.get::<components::Information>(*entity) {
                                Some(info) => Some(info.faction),
                                None => None
                            };
                            if entity_faction.is_none() {
                                continue;
                            }
                            if entity_faction != actor_faction {
                                rejected_actions.push(Action {
                                    actor: action.actor,
                                    target: Some(ActionTarget::Entity(*entity)),
                                    command: Command::AttackEntity{bonus_strength: 0, bonus_defense: 0}
                                });
                            }
                        }
                        return ActionStatus::Reject;
                    }
                }
            }
            ActionStatus::Accept
        },
        _ => ActionStatus::Accept
    }
}
