#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Spells {
    LightningStrike,
    Confusion,
    MagicMissile,
    Heal,
    Fog
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellTargetType {
    Entity,
    Closest,
    Spot
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub kind: Spells,
    pub power: i32,
    pub target: SpellTargetType,
    pub range: i32
}

impl Spell {
    pub fn create(spl: Spells) -> Spell {
        match spl {
            Spells::Fog => {
                Spell {
                    name: "Fog".to_string(),
                    kind: Spells::Fog,
                    power: 0,
                    range: 10,
                    target: SpellTargetType::Spot
                }
            },
            Spells::MagicMissile => {
                Spell {
                    name: "Magic Missile".to_string(),
                    kind: Spells::MagicMissile,
                    power: 2,
                    range: 5,
                    target: SpellTargetType::Entity
                }
            },
            Spells::LightningStrike => {
                Spell {
                    name: "Lightning Strike".to_string(),
                    kind: Spells::LightningStrike,
                    power: 10,
                    range: 4,
                    target: SpellTargetType::Entity
                }
            },
            Spells::Confusion => {
                Spell {
                    name: "Confusion".to_string(),
                    kind: Spells::Confusion,
                    power: 0,
                    range: 5,
                    target: SpellTargetType::Closest
                }
            },
            Spells::Heal => {
                Spell {
                    name: "Heal".to_string(),
                    kind: Spells::Heal,
                    power: 5,
                    range: 3,
                    target: SpellTargetType::Entity
                }
            }
        }
    }
}
