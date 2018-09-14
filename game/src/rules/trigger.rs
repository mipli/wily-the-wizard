use crate::rules::definitions::*;
use crate::actions::*;
use crate::game::*;
use crate::components;
use crate::spells;

pub fn trigger(action: &mut Action, state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> ActionStatus {
    match action.command {
        Command::WalkDirection{dir} => {
            if let Some(actor) = action.actor {
                let solid = match state.spawning_pool.get::<components::Flags>(actor) {
                    Some(flags) => flags.solid,
                    None => false
                };

                if !solid {
                    return ActionStatus::Accept;
                }

                let pos = match state.spawning_pool.get::<components::Physics>(actor) {
                    Some(physics) => physics.coord,
                    None => panic!("Walk command initiated on entity without physics component")
                };

                let new_pos = pos + dir;
                if let Some(cell) = state.spatial_table.get(new_pos) {
                    for entity in &cell.entities {
                        if let Some(trigger) = state.spawning_pool.get::<components::Trigger>(*entity) {
                            if trigger.kind != components::TriggerKind::Step {
                                continue;
                            }
                            if let Some(ref on_trigger) = trigger.on_trigger {
                                match on_trigger {
                                    components::OnTriggerCallback::Spell(spell) => {
                                        let spl = spells::Spell::create(*spell);
                                        let target = match spl.target {
                                            spells::SpellTargetType::Spot => {
                                                if let Some(pos) = get_entity_position(actor, &state) {
                                                    Some(ActionTarget::Position(pos))
                                                } else {
                                                    None
                                                }
                                            },
                                            _ => {
                                                Some(ActionTarget::Entity(actor))
                                            }
                                        };
                                        reaction_actions.push(Action{
                                            actor: None,
                                            target,
                                            command: Command::CastSpell{spell: spl}
                                        });
                                        reaction_actions.push(Action{
                                            actor: Some(*entity),
                                            target: None,
                                            command: Command::KillEntity
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ActionStatus::Accept
        },
        _ => ActionStatus::Accept
    }
}
