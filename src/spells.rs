use spawning_pool::{EntityId};
use game::*;
use actions::{Action, Command};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Spells {
    LightningStrike,
    Confusion
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellTarget {
    Entity
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub kind: Spells,
    pub power: i32,
    pub target: SpellTarget
}

impl Spell {
    pub fn create(spl: Spells) -> Spell {
        match spl {
            Spells::LightningStrike => {
                Spell {
                    name: "LightningStrike".to_string(),
                    kind: Spells::LightningStrike,
                    power: 10,
                    target: SpellTarget::Entity
                }
            },
            Spells::Confusion => {
                Spell {
                    name: "Confusion".to_string(),
                    kind: Spells::Confusion,
                    power: 0,
                    target: SpellTarget::Entity
                }
            }
        }
    }
}

pub fn cast(spell: &Spell, caster: Option<EntityId>, target: Option<EntityId>, _state: &mut GameState, reaction_actions: &mut Vec<Action>) {
    match spell.kind {
        Spells::LightningStrike => {
            reaction_actions.push(Action{
                actor: caster,
                target,
                command: Command::LightningStrike{damage: spell.power}
            });
        },
        Spells::Confusion => {
            println!("What now?");
            reaction_actions.push(Action{
                actor: caster,
                target,
                command: Command::Confuse
            });
        }
    }
}



/*
pub fn cast(spell: Spell, caster: EntityId, target: Option<EntityId>, state: &mut GameState, reaction_actions: &mut Vec<Action>) -> bool {
    match spell.target {
        SpellTarget::Entity => cast_on_entity(spell, caster, target, state, reaction_actions)
    }
}

fn cast_on_entity(spell: Spell, caster: EntityId, target: Option<EntityId>, state: &mut GameState, reaction_actions: &mut Vec<Action>) -> bool {
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
            reaction_actions.push(Action{
                actor: Some(caster),
                target: Some(entities[1].0),
                command: Command::LightningStrike{damage: spell.damage}
            });
            return true;
        }
    }
    false
}
*/
