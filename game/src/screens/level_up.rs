use tcod::console::*;
use tcod::colors;
use tcod::input::{KeyCode};
use screens::*;
use screens::utils::{get_menu};

use spawning_pool::{EntityId};
use actions::{LevelUpChoice, Action, Command};

pub struct LevelUpScreen { 
    exit: bool,
    selected: Option<EntityId>,
    screens: Vec<ScreenPointer>,
    choice: Option<LevelUpChoice>
}

impl LevelUpScreen {
    pub fn new() -> Self {
        LevelUpScreen {
            exit: false,
            selected: None,
            choice: None,
            screens: vec![]
        }
    }
}

impl Screen for LevelUpScreen {
    fn should_discard(&self, _state: &mut GameState) -> bool {
        self.exit
    }

    fn new_screens(&mut self, _state: &mut GameState) -> Vec<ScreenPointer> {
        self.screens.drain(..).collect()
    }

    fn render(&mut self, _delta: f64, _state: &mut GameState, _fov: &tcod::map::Map, _tcod: &mut render::Tcod) -> (ScreenResult, Option<ModularWindow>) {
        let menu = get_menu(&["(s) Strength", "(d) Defense"]);
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
            "Level Up!"
        );

        blit(&menu, (0, 0), (width, height), &mut root, (1, 2), 1.0, 1.0);
        (ScreenResult::PassThrough, Some(ModularWindow{screen: root, alpha: 0.7, pos: ModularWindowPosition::Center}))
    }

    fn tick(&mut self, state: &mut GameState, _tcod: &mut render::Tcod, actions: &mut Vec<Action>) -> ScreenResult {
        match self.choice {
            Some(LevelUpChoice::Strength) => {
                actions.push(Action{
                    actor: Some(state.player),
                    target: None,
                    command: Command::LevelUp(LevelUpChoice::Strength)
                });
                self.exit = true;
            },
            Some(LevelUpChoice::Defense) => {
                actions.push(Action{
                    actor: Some(state.player),
                    target: None,
                    command: Command::LevelUp(LevelUpChoice::Defense)
                });
                self.exit = true;
            },
            None => {}
        };
        ScreenResult::Stop
    }

    fn handle_input(&mut self, input: &Input, _state: &mut GameState) -> ScreenResult {
        match input.key {
            Key { printable: 's', .. } => {
                self.choice = Some(LevelUpChoice::Strength);
            },
            Key { printable: 'd', .. } => {
                self.choice = Some(LevelUpChoice::Defense);
            },
            _ => {}
        }

        ScreenResult::Stop
    }
}
