use std::collections::HashMap;
use std::fmt;
use std;
use tcod::colors;
use geo::*;
use spells;

use spawning_pool::EntityId;
use spawning_pool::storage::{Storage, VectorStorage, HashMapStorage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visual {
    pub always_display: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    pub spawn_time: i32,
    pub duration: i32,
    pub expire_time: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMemory {
    pub previous_position: Option<Point>,
    pub path_goal: Option<Point>,
    pub path: Option<Vec<Point>>
}

impl AiMemory {
    pub fn new() -> AiMemory {
        AiMemory {
            previous_position: None,
            path_goal: None,
            path: None
        }
    }
    pub fn remember_path_to(&mut self, current_position: Point, target: Point) -> Option<Point> {
        if let Some(prev) = self.previous_position {
            if prev != current_position {
                self.forget();
                return None;
            }
        }
        let goal = self.path_goal?;
        if goal != target {
            return None;
        }
        match self.path {
            Some(ref mut path) => {
                let next = path.pop();
                self.previous_position = next;
                next
            },
            None => None
        }
    }
    pub fn forget(&mut self) {
        self.path_goal = None;
        self.path = None;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AI {
    Player,
    Basic,
    SpellCaster
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Faction {
    Player,
    Enemy
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Effect {
    Slow,
    Confuse
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Effect::Slow => write!(f, "slow"),
            Effect::Confuse => write!(f, "confuse")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub faction: Faction,
    pub max_health: i32,
    pub health: i32,
    pub strength: i32,
    pub defense: i32,
    pub points: i32,
    pub effects: HashMap<Effect, i32>
}

impl Stats {
    pub fn new(faction: Faction, max_health: i32, strength: i32, defense: i32) -> Stats {
        Stats {
            faction,
            max_health,
            health: max_health,
            strength,
            defense,
            points: 0,
            effects: Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellBook {
    pub spells: Vec<spells::Spells>
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
    pub dimensions: Point,
    explored: Vec<bool>,
    visible: Vec<bool>,
}

impl MapMemory {
    pub fn new(width: i32, height: i32) -> Self {
        MapMemory {
            dimensions: (width, height).into(),
            explored: vec![false; (width*height) as usize],
            visible: vec![false; (width*height) as usize]
        }
    }

    pub fn reset(&mut self) {
        self.explored = vec![false; (self.dimensions.x * self.dimensions.y) as usize];
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
        self.explored[(x + (y  * self.dimensions.x)) as usize]
    }

    pub fn explore(&mut self, x: i32, y: i32) {
        self.explored[(x + (y  * self.dimensions.x)) as usize] = true;
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

create_spawning_pool!(
    (Visual, visual, VectorStorage),
    (Physics, physics, VectorStorage),
    (Controller, controller, HashMapStorage),
    (AiMemory, ai_memory, HashMapStorage),
    (Stats, stats, VectorStorage),
    (Door, door, HashMapStorage),
    (Information, information, VectorStorage),
    (MapMemory, map_memory, HashMapStorage),
    (Flags, flags, HashMapStorage),
    (Item, item, HashMapStorage),
    (Inventory, inventory, HashMapStorage),
    (Equipment, equipment, HashMapStorage),
    (SpellBook, spell_book, HashMapStorage),
    (Duration, duration, HashMapStorage)
);
