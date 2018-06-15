use tcod::console::*;
use tcod::colors;
use tcod::input::{KeyCode};
use screens::*;

pub struct GameOverScreen { 
    exit: bool,
}

impl GameOverScreen {
    pub fn new() -> Self {
        GameOverScreen {
            exit: false,
        }
    }
}

impl Screen for GameOverScreen {
    fn should_discard(&self, _state: &mut GameState) -> bool {
        self.exit
    }

    fn new_screens(&mut self, _state: &mut GameState) -> Vec<ScreenPointer> {
        vec![]
    }

    fn render(&mut self, _delta: f64, _sate: &mut GameState, _fov: &tcod::map::Map, _tcod: &mut render::Tcod) -> (ScreenResult, Option<ModularWindow>) {
        let mut root = Offscreen::new(14, 5);
        root.set_default_foreground(colors::WHITE);
        root.print_rect_ex(
            2,
            2,
            14,
            5,
            BackgroundFlag::None,
            TextAlignment::Left,
            "Game Over!"
        );
        (ScreenResult::PassThrough, Some(ModularWindow{screen: root, alpha: 1.0, pos: ModularWindowPosition::Center}))
    }

    fn tick(&mut self, _state: &mut GameState, _tcod: &mut render::Tcod, _actions: &mut Vec<Action>) -> ScreenResult {
        ScreenResult::PassThrough
    }

    fn handle_input(&mut self, input: &Input, _state: &mut GameState) -> ScreenResult {
        match input.key {
            Key { code: KeyCode::Escape, .. } | Key { code: KeyCode::Text, printable: 'q', .. } => {
                self.exit = true;
            },
            _ => {}
        };
        ScreenResult::Stop
    }
}
