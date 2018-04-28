use spawning_pool::{EntityId};
use geo::*;
use spells;

#[derive(Clone, Debug)]
pub struct Action {
    pub actor: Option<EntityId>,
    pub target: Option<EntityId>,
    pub command: Command
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
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
    DropItem{item_id: EntityId},
    Heal{amount: i32},
    PreKillEntity,
    KillEntity,
    PickUpItem{item_id: EntityId},
    LightningStrike{damage: i32},
    Confuse,
    Wait,
    Abort
}

