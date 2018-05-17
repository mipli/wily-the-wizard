use tcod::input::{KeyCode};
use std::rc::Rc;
use std::cell::RefCell;
use spells;

use utils;
use components;

use screens::*;

enum InputCommand {
    SelfHeal,
    PickUpItem,
    ShowInventoryUse,
    ShowInventoryDrop,
    Quit,
    GameCommand{command: Command},
    ToggleOmnipotence,
    TileInteraction,
    Look
}

pub struct GameScreen {
    exit: bool,
    alive: bool,
    game_over: bool,
    omnipotent: bool,
    stats: Option<components::Stats>,
    map_memory: Option<components::MapMemory>,
    screens: Vec<ScreenPointer>,
    input_command: Option<InputCommand>
}

impl GameScreen {
    pub fn new() -> Self {
        GameScreen {
            exit: false,
            alive: true,
            game_over: false,
            omnipotent: false,
            screens: vec![],
            stats: None,
            map_memory: None,
            input_command: None,
        }
    }

    pub fn add_win_screen(&mut self) {
        println!("Adding win screen");
        self.screens.push(Rc::new(RefCell::new(Box::new(WinScreen::new(Box::new(|s| { 
            s.close();
        }))))));
    }
}

impl Screen for GameScreen {
    fn close(&mut self) {
        self.exit = true;
    }

    fn should_discard(&self, _state: &mut GameState) -> bool {
        self.exit
    }

    fn new_screens(&mut self, _state: &mut GameState) -> Vec<ScreenPointer> {
        self.screens.drain(..).collect()
    }

    fn render(&mut self, delta: f64, state: &mut GameState, _fov: &tcod::map::Map, tcod: &mut render::Tcod) -> (ScreenResult, Option<ModularWindow>) {
        let stats: Option<components::Stats> = if let Some(stats) = state.spawning_pool.force_get::<components::Stats>(state.player) {
            Some(stats.clone())
        } else {
            self.stats.clone()
        };
        let memory: Option<components::MapMemory> = if let Some(memory) = state.spawning_pool.force_get::<components::MapMemory>(state.player) {
            Some(memory.clone())
        } else {
            self.map_memory.clone()
        };
        if let Some(stats) = stats {
            if let Some(memory) = memory {
                let screen = render(tcod, &stats, &memory, state, self.omnipotent, delta);
                self.stats = Some(stats);
                self.map_memory = Some(memory);
                return (ScreenResult::Stop, Some(ModularWindow{screen, alpha: 1.0, pos: ModularWindowPosition::Position{point: (0, 0).into()}}));
            }
        }
        (ScreenResult::Stop, None)
    }

    fn tick(&mut self, state: &mut GameState, _tcod: &mut render::Tcod, actions: &mut Vec<Action>) -> ScreenResult {
        match self.input_command {
            Some(InputCommand::GameCommand{ref command}) => {
                actions.push(Action{
                    actor: Some(state.player),
                    target: None,
                    command: command.clone()
                });
            },
            Some(InputCommand::SelfHeal) => {
                actions.push(Action{
                    actor: Some(state.player),
                    target: Some(ActionTarget::Entity(state.player)),
                    command: Command::CastSpell{
                        spell: spells::Spell::create(spells::Spells::Heal)
                    }
                });
            }
            Some(InputCommand::Quit) => {
                self.exit = true;
            },
            Some(InputCommand::ToggleOmnipotence) => {
                self.omnipotent = !self.omnipotent;
            },
            Some(InputCommand::Look) => {
                if let Some(physics) = state.spawning_pool.get::<components::Physics>(state.player) {
                    self.screens.push(Rc::new(RefCell::new(Box::new(TargetScreen::new(physics.coord, state)))));
                }
            },
            Some(InputCommand::ShowInventoryUse) => {
                self.screens.push(Rc::new(RefCell::new(Box::new(InventoryScreen::new(InventoryAction::UseItem)))));
            },
            Some(InputCommand::ShowInventoryDrop) => {
                self.screens.push(Rc::new(RefCell::new(Box::new(InventoryScreen::new(InventoryAction::DropItem)))));
            },
            Some(InputCommand::PickUpItem{..}) => {
                let position = match state.spawning_pool.get::<components::Physics>(state.player) {
                    Some(physics) => physics.coord,
                    None => panic!("Non physical entity trying to pick something up")
                };
                if let Some(item) = get_item_at(position, state) {
                    actions.push(Action{
                        actor: Some(state.player),
                        target: None,
                        command: Command::PickUpItem{item_id: item}
                    });
                }
            },
            Some(InputCommand::TileInteraction) => {
                tile_interaction(state, actions);
                if actions.iter().any(|a| a.command == Command::Win) {
                    self.add_win_screen();
                    actions.clear();
                }
            }
            _ => {}
        };
        self.input_command = None;

        if let Some(current) = state.scheduler.current {
            if current == state.player {
                if let Some(stats) = state.spawning_pool.get::<components::Stats>(state.player) {
                    self.alive = stats.health > 0;
                } else {
                    self.alive = false;
                }
                if !self.alive && !self.game_over {
                    self.screens.push(Rc::new(RefCell::new(Box::new(GameOverScreen::new()))));
                    self.game_over = true;
                }
            }
        }

        ScreenResult::Stop
    }

    fn handle_input(&mut self, input: &Input, _state: &mut GameState) -> ScreenResult {
        self.input_command = match input.key {
            Key { code: KeyCode::Escape, .. } => Some(InputCommand::Quit),  // exit game
            Key { code: KeyCode::Up, .. } | Key { printable: 'k', .. } => {
                Some(InputCommand::GameCommand{command: Command::WalkDirection{ dir: (0, -1).into() }})
            },
            Key { printable: 'u', .. } => {
                Some(InputCommand::GameCommand{command: Command::WalkDirection{ dir: (1, -1).into() }})
            },
            Key { code: KeyCode::Right, .. } | Key { printable: 'l', .. } => {
                Some(InputCommand::GameCommand{command: Command::WalkDirection{ dir: (1, 0).into() }})
            },
            Key { printable: 'n', .. } => {
                Some(InputCommand::GameCommand{command: Command::WalkDirection{ dir: (1, 1).into() }})
            },
            Key { code: KeyCode::Down, .. } | Key { printable: 'j', .. } => {
                Some(InputCommand::GameCommand{command: Command::WalkDirection{ dir: (0, 1).into() }})
            },
            Key { printable: 'b', .. } => {
                Some(InputCommand::GameCommand{command: Command::WalkDirection{ dir: (-1, 1).into() }})
            },
            Key { code: KeyCode::Left, .. } | Key { printable: 'h', .. } => {
                Some(InputCommand::GameCommand{command: Command::WalkDirection{ dir: (-1, 0).into() }})
            },
            Key { printable: 'y', .. } => {
                Some(InputCommand::GameCommand{command: Command::WalkDirection{ dir: (-1, -1).into() }})
            },
            Key { printable: ' ', .. } => {
                Some(InputCommand::TileInteraction)
            },
            Key { printable: 'i', .. } => {
                Some(InputCommand::ShowInventoryUse)
            },
            Key { printable: 'd', .. } => {
                Some(InputCommand::ShowInventoryDrop)
            },
            Key { printable: 'x', .. } => {
                Some(InputCommand::Look)
            },
            Key { printable: ',', .. } => {
                Some(InputCommand::PickUpItem)
            },
            Key { printable: '.', .. } => {
                Some(InputCommand::GameCommand{command: Command::Wait})
            },
            Key { printable: '1', .. } => {
                Some(InputCommand::GameCommand{command: Command::CastSpell{
                    spell: spells::Spell::create(spells::Spells::Fog)
                }})
            },
            Key { printable: '0', .. } => {
                Some(InputCommand::ToggleOmnipotence)
            },
            Key { printable: '9', .. } => {
                Some(InputCommand::SelfHeal)
            },
            _ => None
        };
        if self.game_over {
            self.exit = true;
        }
        ScreenResult::Stop
    }
}

fn tile_interaction(state: &GameState, actions: &mut Vec<Action>) {
    if let Some(pos) = utils::get_position(state.player, &state.spawning_pool) {
        if let Some(spatial_cell) = state.spatial_table.get(pos) {
            for entity in &spatial_cell.entities {
                let glyph = utils::get_glyph(*entity, &state.spawning_pool);
                if let Some(glyph) = glyph {
                    if glyph == '<' {
                        trigger_stair_interaction(state, actions);
                    } else if glyph == 'X' {
                        trigger_portal_interaction(state, actions);
                    }
                }
            }
        }
    }
}

fn trigger_stair_interaction(state: &GameState, actions: &mut Vec<Action>) {
    actions.push(Action{
        actor: Some(state.player),
        target: None,
        command: Command::DescendStairs
    });
}

fn trigger_portal_interaction(state: &GameState, actions: &mut Vec<Action>) {
    actions.push(Action{
        actor: Some(state.player),
        target: None,
        command: Command::Win
    });
}

