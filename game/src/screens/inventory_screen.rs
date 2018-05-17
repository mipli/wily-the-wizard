use std::collections::HashMap;
use tcod::console::*;
use tcod::colors;
use tcod::input::{KeyCode};
use screens::*;
use screens::utils::{get_menu};

use spawning_pool::{EntityId};
use components;

pub struct InventoryScreen { 
    exit: bool,
    target: bool,
    options: HashMap<char, EntityId>,
    selected: Option<EntityId>,
    action: InventoryAction,
    screens: Vec<ScreenPointer>
}

pub enum InventoryAction {
    UseItem,
    DropItem
}

impl InventoryScreen {
    pub fn new(action: InventoryAction) -> Self {
        InventoryScreen {
            action,
            target: false,
            exit: false,
            options: Default::default(),
            selected: None,
            screens: vec![]
        }
    }
}

impl Screen for InventoryScreen {
    fn should_discard(&self, _state: &mut GameState) -> bool {
        self.exit
    }

    fn new_screens(&mut self, _state: &mut GameState) -> Vec<ScreenPointer> {
        self.screens.drain(..).collect()
    }

    fn render(&mut self, _delta: f64, state: &mut GameState, _fov: &tcod::map::Map, _tcod: &mut render::Tcod) -> (ScreenResult, Option<ModularWindow>) {
        let equipped: HashMap<EntityId, components::EquipmentSlot> = match state.spawning_pool.get::<components::Equipment>(state.player) {
            Some(equipment) => {
                let mut tmp: HashMap<EntityId, components::EquipmentSlot> = Default::default();
                for (slot, item_id) in &equipment.items {
                    tmp.insert(*item_id, *slot);
                }
                tmp
            },
            None => Default::default()
        };
        let mut items = vec![];
        if let Some(inventory) = state.spawning_pool.get::<components::Inventory>(state.player) {
            let mut index = 0;
            if !inventory.items.is_empty() {
                for &id in &inventory.items {
                    if let Some(information) = state.spawning_pool.get::<components::Information>(id) {
                        let name = match equipped.get(&id) {
                            Some(slot) => format!("{} (equipped in {})", information.name, slot),
                            None => format!("{}", information.name)
                        };
                        let chr = (b'a' + index as u8) as char;
                        items.push(format!("({}) {}", chr, name));
                        self.options.insert(chr, id);
                        index += 1;
                    }
                }
            }
        }
        let menu = get_menu(&items);
        let width = menu.width();
        let height = menu.height();

        let mut root = Offscreen::new(width + 2, height + 3);
        root.set_default_foreground(colors::WHITE);
        root.print_rect_ex(
            (width + 2)/2 - 4,
            0,
            width,
            1,
            BackgroundFlag::None,
            TextAlignment::Left,
            "Inventory"
        );

        blit(&menu, (0, 0), (width, height), &mut root, (1, 2), 1.0, 1.0);
        (ScreenResult::PassThrough, Some(ModularWindow{screen: root, alpha: 0.7, pos: ModularWindowPosition::Center}))
    }

    fn tick(&mut self, state: &mut GameState, _tcod: &mut render::Tcod, actions: &mut Vec<Action>) -> ScreenResult {
        if let Some(selected) = self.selected {
            match self.action {
                InventoryAction::UseItem => {
                    self.add_use_action(selected, actions, state);
                },
                InventoryAction::DropItem => {
                    self.target = true;
                    actions.push(Action{
                        actor: Some(state.player),
                        target: None,
                        command: Command::DropItem{item_id: selected}
                    });
                }
            };
            self.selected = None;
            self.exit = true;
        }
        ScreenResult::Stop
    }

    fn handle_input(&mut self, input: &Input, _state: &mut GameState) -> ScreenResult {
        if let Key { code: KeyCode::Escape, .. } = input.key {
            self.exit = true;
        }
        if input.key.printable.is_alphabetic() {
            if let Some(item) = self.options.get(&input.key.printable.to_ascii_lowercase()) {
                self.selected = Some(*item);
            }
        }

        ScreenResult::Stop
    }
}

impl InventoryScreen {
    fn add_use_action(&mut self, item_id: EntityId, actions: &mut Vec<Action>, state: &GameState) {
        if !self.equip_action(state.scheduler.get_current(), item_id, actions, &state.spawning_pool) {
            actions.push(Action{
                actor: Some(state.scheduler.get_current()),
                target: None,
                command: Command::UseItem{item_id}
            });
        }
    }

    fn equip_action(&self, entity: EntityId, item_id: EntityId, actions: &mut Vec<Action>, spawning_pool: &components::SpawningPool) -> bool {
        let can_equip = spawning_pool.get::<components::Equipment>(entity).is_some();
        if !can_equip {
            return false;
        }
        if let Some(item) = spawning_pool.get::<components::Item>(item_id) {
            if let Some(slot) = item.equip {
                let equipped = self.currently_equipped(entity, slot, spawning_pool);
                match equipped {
                    Some(eqid) => {
                        actions.push(Action{
                            actor: Some(entity),
                            target: None,
                            command: Command::UnequipItem{item_id}
                        });

                        if eqid != item_id {
                            actions.push(Action{
                                actor: Some(entity),
                                target: None,
                                command: Command::EquipItem{item_id}
                            });
                        }
                    },
                    None => {
                        actions.push(Action{
                            actor: Some(entity),
                            target: None,
                            command: Command::EquipItem{item_id}
                        });
                    }
                }
                return true;
            }
        }
        false
    }

    fn currently_equipped(&self, entity: EntityId, slot: components::EquipmentSlot, spawning_pool: &components::SpawningPool) -> Option<EntityId> {
        let equipment = spawning_pool.get::<components::Equipment>(entity)?;
        let item = equipment.items.get(&slot)?;
        Some(*item)
    }
}
