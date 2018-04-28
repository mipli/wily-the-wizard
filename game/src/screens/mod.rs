use std::collections::{LinkedList};

use tcod;
use tcod::input::{self, Mouse, Event, Key};
use tcod::console::*;
use tcod::colors;

use render;

use point::*;
use consts::*;
use actions::*;
use game::*;

pub mod game_screen;
pub mod main_menu;
mod inventory_screen;
mod game_over;
mod utils;
mod target;

pub use self::inventory_screen::{InventoryScreen, InventoryAction};
pub use self::game_over::GameOverScreen;
pub use self::target::{TargetScreen, SingleTargetScreen};

pub struct Input {
    key: Key,
    mouse: Mouse
}

#[derive(PartialEq, Eq)]
pub enum ScreenResult {
    Stop,
    PassThrough
}

pub struct ModularWindow {
    screen: Offscreen,
    pos: ModularWindowPosition,
    alpha: f32
}

#[derive(PartialEq, Eq)]
pub enum ModularWindowPosition {
    Center,
    Position{point: Point}
}

pub struct ScreenManager {
    pub screens: LinkedList<Box<Screen>>
}

impl ScreenManager {
    pub fn new() -> Self {
        ScreenManager {
            screens: Default::default()
        }
    }

    pub fn add_screens(&mut self, state: &mut GameState) {
        let mut screens = vec![];
        for screen in &mut self.screens {
            screens.append(&mut screen.new_screens(state));
        }
        while !screens.is_empty() {
            self.add_screen(screens.pop().unwrap());
        }
    }

    pub fn render(&mut self, delta: f64, state: &mut GameState, fov: &tcod::map::Map, tcod: &mut render::Tcod) {
        let mut windows = vec![];
        for screen in &mut self.screens {
            let (res, window) = screen.render(delta, state, fov, tcod);
            if let Some(window) = window {
                windows.push(window);
            }
            if res == ScreenResult::Stop {
                break;
            }
        }
        tcod.root.set_default_background(colors::BLACK);
        tcod.root.clear();
        windows.reverse();
        for window in &windows {
            let (x, y) = match window.pos {
                ModularWindowPosition::Position{point} => {
                    (point.x, point.y)
                },
                ModularWindowPosition::Center => {
                    let x = SCREEN_WIDTH/2 - window.screen.width()/2;
                    let y = SCREEN_HEIGHT/2 - window.screen.height()/2;
                    (x, y)
                }
            };
            blit(&window.screen, (0, 0), (window.screen.width(), window.screen.height()), &mut tcod.root, (x, y), 1.0, window.alpha);
        }
        tcod.root.flush();
    }

    pub fn tick(&mut self, state: &mut GameState, tcod: &mut render::Tcod, actions: &mut Vec<Action>) {
        for screen in &mut self.screens {
            let res = screen.tick(state, tcod, actions);
            if res == ScreenResult::Stop {
                break;
            }
        }
    }

    pub fn handle_input(&mut self, state: &mut GameState) {
        let mut input = Input {
            key: Default::default(),
            mouse: Default::default()
        };
        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => input.mouse = m,
            Some((_, Event::Key(k))) => input.key = k,
            _ => input.key = Default::default()
        }
        for screen in &mut self.screens {
            let res = screen.handle_input(&input, state);
            if res == ScreenResult::Stop {
                break;
            }
        }
    }

    pub fn add_screen(&mut self, screen: Box<Screen>) {
        self.screens.push_front(screen);
    }

    pub fn clear_screens(&mut self, state: &mut GameState) {
        let screens = self.screens.drain_filter(|s| s.status(state)).collect();
        self.screens = screens;
    }
}

pub trait Screen {
    fn status(&self, state: &mut GameState) -> bool;
    fn new_screens(&mut self, state: &mut GameState) -> Vec<Box<Screen>>;
    fn render(&mut self, delta: f64, state: &mut GameState, fov: &tcod::map::Map, tcod: &mut render::Tcod) -> (ScreenResult, Option<ModularWindow>);
    fn tick(&mut self, state: &mut GameState, tcod: &mut render::Tcod, actions: &mut Vec<Action>) -> ScreenResult;
    fn handle_input(&mut self, input: &Input, state: &mut GameState) -> ScreenResult;
}


pub fn create_new_game() -> Game {
    let state = GameState::new();

    let fov = tcod::map::Map::new(state.map.dimensions.x, state.map.dimensions.y);

    Game {
        state,
        fov,
        tick_time: 0,
        current_action: None,
        action_queue: vec![],
        reaction_queue: vec![],
        rejection_queue: vec![]
    }
}