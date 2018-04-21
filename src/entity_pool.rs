use std::collections::HashMap;
use std;

use spawning_pool::EntityId;
use spawning_pool::storage::{Storage, VectorStorage};

use tcod::colors;
use point::*;

#[derive(Debug)]
pub struct Visual {
    pub glyph: char,
    pub color: colors::Color
}

#[derive(Debug, Clone)]
pub struct Physics {
    pub coord: Point
}

#[derive(Debug)]
pub struct Controller {
    pub kind: AI
}

#[derive(Debug, Copy, Clone)]
pub enum AI {
    Player,
    Basic
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Faction {
    Player,
    Enemy
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub faction: Faction,
    pub max_health: i32,
    pub health: i32,
    pub strength: i32,
    pub defense: i32
}

#[derive(Debug, Clone)]
pub struct Door { 
    pub opened: bool
}

#[derive(Debug, Clone)]
pub struct Information {
    pub name: String
}


#[derive(Debug)]
pub struct MapMemory {
    pub map: Vec<bool>,
    pub dimensions: Point
}

impl MapMemory {
    pub fn new(width: i32, height: i32) -> Self {
        MapMemory {
            dimensions: (width, height).into(),
            map: vec![false; (width*height) as usize]
        }
    }

    pub fn is_explored(&self, x: i32, y: i32) -> bool {
        self.map[(x + (y  * self.dimensions.x)) as usize]
    }

    pub fn explore(&mut self, x: i32, y: i32) {
        self.map[(x + (y  * self.dimensions.x)) as usize] = true;
    }
}

#[derive(Debug, Clone)]
pub struct Flags {
    pub block_sight: bool,
    pub solid: bool
}

#[derive(Debug, Clone)]
pub struct StatisticsBonus {
    pub strength: i32,
    pub defense: i32
}

#[derive(Debug, Clone)]
pub enum OnUseCallback {
    SelfHeal,
    LightningStrike
}

#[derive(Debug, Clone)]
pub struct Item {
    pub equip: Option<EquipmentSlot>,
    pub statistics_bonus: Option<StatisticsBonus>,
    pub on_use: Option<OnUseCallback>
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: Vec<EntityId>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
pub struct Equipment {
    pub items: HashMap<EquipmentSlot, EntityId>
}
