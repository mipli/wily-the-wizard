use tcod::console::*;
use components;
use tcod::colors;
use tcod::input::{KeyCode};
use screens::*;
use screens::utils::{get_menu};
use save;

pub struct MainMenuScreen { 
    exit: bool,
    create_game: bool,
    load: bool,
    cont: bool,
    alive: bool,
    running: bool,
    menu: Offscreen
}

impl MainMenuScreen {
    pub fn new() -> Self {

        MainMenuScreen {
            exit: false,
            alive: true,
            load: false,
            create_game: false,
            cont: false,
            running: false,
            menu: get_menu(&[""])
        }
    }
}

impl Screen for MainMenuScreen {
    fn should_discard(&self, _state: &mut GameState) -> bool {
        self.exit
    }

    fn new_screens(&mut self, _state: &mut GameState) -> Vec<ScreenPointer> {
        if self.create_game {
            self.create_game = false;
            self.running = true;

            vec![Rc::new(RefCell::new(Box::new(game_screen::GameScreen::new())))]
        } else if self.alive && self.cont {
            self.cont = false;
            self.running = true;
            vec![Rc::new(RefCell::new(Box::new(game_screen::GameScreen::new())))]
        } else if self.load {
            self.cont = false;
            self.running = true;
            self.load = false;
            vec![Rc::new(RefCell::new(Box::new(game_screen::GameScreen::new())))]
        } else {
            vec![]
        }
    }

    fn render(&mut self, _delta: f64, _sate: &mut GameState, _fov: &tcod::map::Map, _tcod: &mut render::Tcod) -> (ScreenResult, Option<ModularWindow>) {
        let mut root = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
        root.set_default_foreground(colors::WHITE);
        root.print_rect_ex(
            SCREEN_WIDTH / 2 - 8,
            10,
            20,
            5,
            BackgroundFlag::None,
            TextAlignment::Left,
            "Wily the Wizard"
        );
        let width = self.menu.width();
        let height = self.menu.height();

        let x = (SCREEN_WIDTH / 2) - width / 2;

        blit(&self.menu, (0, 0), (width, height), &mut root, (x, 12), 1.0, 1.0);
        (ScreenResult::Stop, Some(ModularWindow{screen: root, alpha: 1.0, pos: ModularWindowPosition::Position{point: (0, 0).into()}}))
    }

    fn tick(&mut self, state: &mut GameState, _tcod: &mut render::Tcod, actions: &mut Vec<Action>) -> ScreenResult {
        self.alive = if let Some(stats) = state.spawning_pool.get::<components::Stats>(state.player) {
            stats.health > 0
        } else {
            false
        };
        if self.alive && self.running {
            self.menu = get_menu(&["(c) Continue", "(n) New Game", "(l) Load Game", "(q) Save and Quit"]);
        } else {
            self.menu = get_menu(&["(n) New Game", "(l) Load Game", "(q) Quit"]);
        }
        if self.exit && self.running {
            save_game(state);
        }
        if self.load {
            actions.push(Action{
                actor: None,
                target: None,
                command: Command::LoadGame
            });
        }
        if self.create_game {
            actions.push(Action{
                actor: None,
                target: None,
                command: Command::CreateGame
            });
        }
        ScreenResult::Stop
    }

    fn handle_input(&mut self, input: &Input, _state: &mut GameState) -> ScreenResult {
        match input.key {
            Key { code: KeyCode::Escape, .. } | Key { printable: 'q', .. } => {
                self.exit = true;
            },
            Key { printable: 'n', .. } => {
                self.create_game = true;
            },
            Key { printable: 'l', .. } => {
                self.load = true;
            },
            Key { printable: 'c', .. } => {
                self.cont = self.running;
            },
            _ => {}
        };
        ScreenResult::Stop
    }
}

fn save_game(state: &GameState) {
    let _ = save::save_game(&state);
}

pub fn load_game() -> GameState {
    if let Ok(state) = save::load_game() {
        state
    } else {
        panic!("Could not load game");
    }
}
