#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Spells {
    LightningStrike,
    Confusion,
    MagicMissile,
    Heal
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellTarget {
    Entity,
    Closest
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub kind: Spells,
    pub power: i32,
    pub target: SpellTarget,
    pub range: i32
}

impl Spell {
    pub fn create(spl: Spells) -> Spell {
        match spl {
            Spells::MagicMissile => {
                Spell {
                    name: "Magic Missile".to_string(),
                    kind: Spells::MagicMissile,
                    power: 2,
                    range: 5,
                    target: SpellTarget::Entity
                }
            },
            Spells::LightningStrike => {
                Spell {
                    name: "Lightning Strike".to_string(),
                    kind: Spells::LightningStrike,
                    power: 10,
                    range: 4,
                    target: SpellTarget::Entity
                }
            },
            Spells::Confusion => {
                Spell {
                    name: "Confusion".to_string(),
                    kind: Spells::Confusion,
                    power: 0,
                    range: 5,
                    target: SpellTarget::Closest
                }
            },
            Spells::Heal => {
                Spell {
                    name: "Heal".to_string(),
                    kind: Spells::Heal,
                    power: 5,
                    range: 3,
                    target: SpellTarget::Entity
                }
            }
        }
    }
}

/*
pub fn cast(spell: &Spell, caster: Option<EntityId>, target: Option<EntityId>, state: &mut GameState, reaction_actions: &mut Vec<Action>) {
    let target = match spell.target {
        SpellTarget::Entity => target,
        SpellTarget::Closest => get_closest_target(caster.unwrap(), state),

    };
    if let Some(target) = target {
        let actor_name = match caster {
            Some(caster) => utils::get_entity_name(caster, &state.spawning_pool),
            None => "Unknown".to_string()
        };
        let target_name = utils::get_entity_name(target, &state.spawning_pool);
        state.messages.log(MessageLevel::Spell, format!("{} casts {} on {}!", actor_name, spell.name, target_name));
    }
    match spell.kind {
        Spells::MagicMissile => {
            reaction_actions.push(Action{
                actor: caster,
                target,
                command: Command::TakeDamage{damage: spell.power}
            });
        },
        Spells::LightningStrike => {
            reaction_actions.push(Action{
                actor: caster,
                target,
                command: Command::LightningStrike{damage: spell.power}
            });
        },
        Spells::Confusion => {
            reaction_actions.push(Action{
                actor: caster,
                target,
                command: Command::Confuse
            });
        },
        Spells::Heal => {
            reaction_actions.push(Action{
                actor: caster,
                target,
                command: Command::Heal{amount: spell.power}
            });
        }
    }
}

fn get_closest_target(caster: EntityId, state: &mut GameState) -> Option<EntityId> {
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
*/
