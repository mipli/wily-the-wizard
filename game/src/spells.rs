#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Spells {
    LightningStrike,
    Confusion,
    MagicMissile,
    RayOfFrost,
    Experience,
    Heal,
    Fog,
    Stun
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellTargetType {
    Entity,
    Closest,
    Spot,
    Projectile,
    Ray
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellTargeting {
    Select,
    Closest,
    Caster
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub kind: Spells,
    pub power: i32,
    pub range: i32,
    pub target: SpellTargetType,
    pub targeting: SpellTargeting
}

impl Spell {
    pub fn create(spl: Spells) -> Spell {
        match spl {
            Spells::Stun => {
                Spell {
                    name: "Stun".to_string(),
                    kind: Spells::Stun,
                    power: 1,
                    range: 5,
                    target: SpellTargetType::Entity,
                    targeting: SpellTargeting::Select
                }
            },
            Spells::Experience => {
                Spell {
                    name: "Experience".to_string(),
                    kind: Spells::Experience,
                    power: 1,
                    range: 10,
                    target: SpellTargetType::Entity,
                    targeting: SpellTargeting::Caster
                }
            },
            Spells::RayOfFrost => {
                Spell {
                    name: "Ray of Frost".to_string(),
                    kind: Spells::RayOfFrost,
                    power: 1,
                    range: 10,
                    target: SpellTargetType::Ray,
                    targeting: SpellTargeting::Select
                }
            },
            Spells::Fog => {
                Spell {
                    name: "Fog".to_string(),
                    kind: Spells::Fog,
                    power: 0,
                    range: 5,
                    target: SpellTargetType::Spot,
                    targeting: SpellTargeting::Select
                }
            },
            Spells::MagicMissile => {
                Spell {
                    name: "Magic Missile".to_string(),
                    kind: Spells::MagicMissile,
                    power: 5,
                    range: 10,
                    target: SpellTargetType::Projectile,
                    targeting: SpellTargeting::Select
                }
            },
            Spells::LightningStrike => {
                Spell {
                    name: "Lightning Strike".to_string(),
                    kind: Spells::LightningStrike,
                    power: 10,
                    range: 4,
                    target: SpellTargetType::Entity,
                    targeting: SpellTargeting::Select
                }
            },
            Spells::Confusion => {
                Spell {
                    name: "Confusion".to_string(),
                    kind: Spells::Confusion,
                    power: 0,
                    range: 5,
                    target: SpellTargetType::Closest,
                    targeting: SpellTargeting::Closest
                }
            },
            Spells::Heal => {
                Spell {
                    name: "Heal".to_string(),
                    kind: Spells::Heal,
                    power: 5,
                    range: 3,
                    target: SpellTargetType::Entity,
                    targeting: SpellTargeting::Caster
                }
            }
        }
    }
}
