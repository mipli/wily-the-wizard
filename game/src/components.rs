use std::collections::HashMap;
use std;
use tcod::colors;
use point::*;
use spells;

use spawning_pool::EntityId;
use spawning_pool::storage::{Storage, VectorStorage, HashMapStorage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visual {
    pub glyph: char,
    pub color: colors::Color
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Physics {
    pub coord: Point
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Controller {
    pub ai: AI
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AI {
    Player,
    Basic
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Faction {
    Player,
    Enemy
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub faction: Faction,
    pub max_health: i32,
    pub health: i32,
    pub strength: i32,
    pub defense: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Door { 
    pub opened: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Information {
    pub name: String
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapMemory {
    pub map: Vec<bool>,
    pub visible: Vec<bool>,
    pub dimensions: Point
}

impl MapMemory {
    pub fn new(width: i32, height: i32) -> Self {
        MapMemory {
            dimensions: (width, height).into(),
            map: vec![false; (width*height) as usize],
            visible: vec![false; (width*height) as usize]
        }
    }

    pub fn reset(&mut self) {
        self.map = vec![false; (self.dimensions.x * self.dimensions.y) as usize];
        self.visible = vec![false; (self.dimensions.x * self.dimensions.y) as usize];
    }

    pub fn clear_visible(&mut self) {
        self.visible = vec![false; (self.dimensions.x * self.dimensions.y) as usize];
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        self.visible[(x + (y  * self.dimensions.x)) as usize]
    }

    pub fn set_visible(&mut self, x: i32, y: i32, v: bool) {
        self.visible[(x + (y  * self.dimensions.x)) as usize] = v;
    }

    pub fn is_explored(&self, x: i32, y: i32) -> bool {
        self.map[(x + (y  * self.dimensions.x)) as usize]
    }

    pub fn explore(&mut self, x: i32, y: i32) {
        self.map[(x + (y  * self.dimensions.x)) as usize] = true;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags {
    pub block_sight: bool,
    pub solid: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsBonus {
    pub strength: i32,
    pub defense: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnUseCallback {
    Spell(spells::Spells)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemKind {
    Scroll,
    Potion,
    Equipment
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub equip: Option<EquipmentSlot>,
    pub statistics_bonus: Option<StatisticsBonus>,
    pub on_use: Option<OnUseCallback>,
    pub kind: ItemKind
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<EntityId>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    LeftHand,
    RightHand,
    Head
}

impl std::fmt::Display for EquipmentSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EquipmentSlot::LeftHand => write!(f, "left hand"),
            EquipmentSlot::RightHand => write!(f, "right hand"),
            EquipmentSlot::Head => write!(f, "head"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub items: HashMap<EquipmentSlot, EntityId>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffects {
    pub confused: Option<i32>
}

create_spawning_pool!(
    (Visual, visual, VectorStorage),
    (Physics, physics, VectorStorage),
    (Controller, controller, HashMapStorage),
    (Stats, stats, HashMapStorage),
    (Door, door, HashMapStorage),
    (Information, information, HashMapStorage),
    (MapMemory, map_memory, HashMapStorage),
    (Flags, flags, HashMapStorage),
    (Item, item, HashMapStorage),
    (Inventory, inventory, HashMapStorage),
    (Equipment, equipment, HashMapStorage),
    (StatusEffects, status_effects, HashMapStorage)
);