use spawning_pool::{EntityId};
use crate::geo::*;
use crate::spells;

#[derive(Clone, Copy, Debug)]
pub enum ActionTarget {
    Entity(EntityId),
    Position(Point)
}

#[derive(Clone, Debug)]
pub struct Action {
    pub actor: Option<EntityId>,
    pub target: Option<ActionTarget>,
    pub command: Command,
    pub set_time: Option<i32>
}

impl Action {
    pub fn new(actor: Option<EntityId>, target: Option<ActionTarget>, command: Command) -> Self {
        Action {
            actor,
            target,
            command,
            set_time: None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LevelUpChoice {
    Strength,
    Defense
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Win,
    CreateGame,
    LoadGame,
    DescendStairs,
    WalkDirection{dir: Point},
    AttackEntity{bonus_strength: i32, bonus_defense: i32},
    OpenDoor{entity: EntityId},
    TakeDamage{damage: i32},
    UseItem{item_id: EntityId},
    EquipItem{item_id: EntityId},
    UnequipItem{item_id: EntityId},
    DestroyItem{item_id: EntityId},
    CastSpell{spell: spells::Spell},
    WriteRune{spell: spells::Spells},
    DropItem{item_id: EntityId},
    Heal{amount: i32},
    SpawnFog{pos: Point},
    KillEntity,
    PickUpItem{item_id: EntityId},
    LightningStrike{damage: i32},
    Confuse,
    Slow,
    Stun,
    GainPoint,
    LevelUp(LevelUpChoice),
    Wait,
    Abort
}

