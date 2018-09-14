use tcod::line::{Line};
use geo::*;
use spawning_pool::{EntityId};
use crate::utils;
use crate::actions::*;
use crate::game::*;
use crate::components;
use crate::spells::*;

use crate::rules::definitions::*;

pub fn lightning_strike(action: &mut Action, _state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> ActionStatus {
    if let  Command::LightningStrike{damage} = action.command {
        reaction_actions.push(Action{
            target: action.target,
            actor: action.actor,
            command: Command::TakeDamage{damage}
        });
    }
    ActionStatus::Accept
}

pub fn validate_spell(action: &mut Action, state: &GameState, _rejected_actions: &mut Vec<Action>, _reaction_actions: &mut Vec<Action>) -> ActionStatus {
    use components::*;
    if let Command::CastSpell{ref spell} = action.command {
        if let Some(actor) = action.actor {
            if state.spawning_pool.get::<Physics>(actor).is_none() {
               return  ActionStatus::Reject;
            }
        }
        if let Some(ActionTarget::Entity(target)) = action.target {
            if state.spawning_pool.get::<Physics>(target).is_none() {
               return  ActionStatus::Reject;
            }
        }
        if let Some(actor) = action.actor {
            if let Some(ActionTarget::Entity(target)) = action.target {
                let actor_position = utils::get_position(actor, &state.spawning_pool).unwrap();
                let target_position = utils::get_position(target, &state.spawning_pool).unwrap();
                let distance = actor_position.distance(target_position);
                if distance > spell.range as f32 {
                    return  ActionStatus::Reject;
                }
            }
        }
    }
    ActionStatus::Accept
}

pub fn cast_spell(action: &mut Action, state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> ActionStatus {
    match action.command {
        Command::CastSpell{ref spell} => {
            if cast(spell, action.actor, action.target, state, reaction_actions) {
                ActionStatus::Accept
            } else {
                ActionStatus::Reject
            }
        },
        _ => ActionStatus::Accept
    }
}

enum SpellTarget {
    Entity(EntityId),
    Entities(Vec<EntityId>),
    Position(Point)
}

fn cast(spell: &Spell, caster: Option<EntityId>, target: Option<ActionTarget>, state: &GameState, reaction_actions: &mut Vec<Action>) -> bool {
    let spell_target = match spell.target {
        SpellTargetType::Ray => {
            if let Some(ActionTarget::Position(target)) = target {
                let targets = get_ray_targets(caster.unwrap(), target, state);
                if targets.is_empty() {
                    None
                } else {
                    Some(SpellTarget::Entities(targets))
                }
            } else {
                None
            }
        },
        SpellTargetType::Projectile => {
            if let Some(ActionTarget::Entity(target)) = target {
                let target = get_projectile_target(caster.unwrap(), target, state);
                if target.is_some() {
                    Some(SpellTarget::Entity(target.unwrap()))
                } else {
                    None
                }
            } else {
                None
            }
        },
        SpellTargetType::Entity => {
            if let Some(ActionTarget::Entity(target)) = target {
                Some(SpellTarget::Entity(target))
            } else {
                None
            }
        },
        SpellTargetType::Closest => {
            let target = get_closest_target(caster.unwrap(), state);
            if target.is_some() {
                Some(SpellTarget::Entity(target.unwrap()))
            } else {
                None
            }
        },
        SpellTargetType::Spot => {
            if let Some(ActionTarget::Position(target)) = target {
                Some(SpellTarget::Position(target))
            } else {
                None
            }
        }
    };
    if spell_target.is_none() {
        return false;
    }
    match spell.kind {
        Spells::RayOfFrost => {
            let targets = match spell_target {
                Some(SpellTarget::Entities(ids)) => ids,
                _ => return false
            };
            for target in &targets {
                reaction_actions.push(Action{
                    actor: caster,
                    target: Some(ActionTarget::Entity(*target)),
                    command: Command::TakeDamage{damage: spell.power}
                });
                reaction_actions.push(Action{
                    actor: caster,
                    target: Some(ActionTarget::Entity(*target)),
                    command: Command::Slow
                });
            }
        },
        Spells::Stun => {
            let target = match spell_target {
                Some(SpellTarget::Entity(id)) => id,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: Some(ActionTarget::Entity(target)),
                command: Command::Stun
            });
        },
        Spells::Fog => {
            let target = match spell_target {
                Some(SpellTarget::Position(pos)) => pos,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: None,
                command: Command::SpawnFog{pos: target}
            });
        },
        Spells::MagicMissile => {
            let target = match spell_target {
                Some(SpellTarget::Entity(id)) => id,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: Some(ActionTarget::Entity(target)),
                command: Command::TakeDamage{damage: spell.power}
            });
        },
        Spells::LightningStrike => {
            let target = match spell_target {
                Some(SpellTarget::Entity(id)) => id,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: Some(ActionTarget::Entity(target)),
                command: Command::LightningStrike{damage: spell.power}
            });
        },
        Spells::Confusion => {
            let target = match spell_target {
                Some(SpellTarget::Entity(id)) => id,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: Some(ActionTarget::Entity(target)),
                command: Command::Confuse
            });
        },
        Spells::Heal => {
            let target = match spell_target {
                Some(SpellTarget::Entity(id)) => id,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: Some(ActionTarget::Entity(target)),
                command: Command::Heal{amount: spell.power}
            });
        },
        Spells::Experience => {
            let target = match spell_target {
                Some(SpellTarget::Entity(id)) => id,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: Some(ActionTarget::Entity(target)),
                command: Command::GainPoint
            });
        }
    }
    return true;
}

fn get_closest_target(caster: EntityId, state: &GameState) -> Option<EntityId> {
    let pos = match state.spawning_pool.get::<components::Physics>(caster) {
        Some(physics) => Some(physics.coord),
        None => None
    };
    if let Some(pos) = pos {
        let circles: Vec<(Point, EntityId)> = state.spatial_table.in_circle(pos, 5);
        let mut entities: Vec<(EntityId, i32)> = circles.iter().map(|&(_, id)| {
            let physics = state.spawning_pool.get::<components::Physics>(id).unwrap();
            let delta = pos.distance(physics.coord) as i32;
            (id, delta)
        }).filter(|&(ref id, _)| {
            let stats = state.spawning_pool.get::<components::Stats>(*id);
            stats.is_some()
        }).collect();
        if entities.len() > 1 {
            entities.sort_by(|a, b| {
                a.1.cmp(&b.1)
            });
           return Some(entities[1].0);
        }
    }
    None
}

fn get_projectile_target(caster: EntityId, target: EntityId, state: &GameState) -> Option<EntityId> {
    use components::*;
    let start = get_entity_position(caster, state)?;
    let end = get_entity_position(target, state)?;
    let line = Line::new((start.x, start.y), (end.x, end.y));
    for (x, y) in line {
        match state.spatial_table.get((x, y)) {
            Some(cell) if cell.solid && !cell.entities.is_empty() => {
                for entity in &cell.entities {
                    if state.spawning_pool.get::<Stats>(*entity).is_some() {
                        return Some(*entity);
                    }
                }
            },
            _ => {}
        }
    }
    None
}

fn get_ray_targets(caster: EntityId, end: Point, state: &GameState) -> Vec<EntityId> {
    use components::*;
    let mut entities = vec![];
    if let Some(start) = get_entity_position(caster, state) {
        let line = Line::new((start.x, start.y), (end.x, end.y));
        for (x, y) in line {
            match state.spatial_table.get((x, y)) {
                Some(cell) if cell.solid && !cell.entities.is_empty() => {
                    for entity in &cell.entities {
                        if state.spawning_pool.get::<Stats>(*entity).is_some() {
                            entities.push(*entity);
                        }
                    }
                },
                _ => {}
            }
        }
    }
    entities
}
