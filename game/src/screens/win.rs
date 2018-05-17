use tcod::console::*;
use tcod::colors;
use screens::*;

pub type WinScreenCallback = FnMut(&mut Box<Screen>);

pub struct WinScreen { 
    exit: bool,
    callback: Option<Box<WinScreenCallback>>,
    creator: Option<ScreenPointer>
}

impl WinScreen {
    pub fn new(callback: Box<WinScreenCallback>) -> Self {
        WinScreen {
            exit: false,
            callback: Some(callback),
            creator: None
        }
    }

    pub fn leave(&mut self) {
        if let Some(ref mut callback) = self.callback {
            if let Some(ref creator) = self.creator {
                let mut c = creator.borrow_mut();
                (callback)(&mut c);
            }
        }
    }
}

impl Screen for WinScreen {
    fn should_discard(&self, _state: &mut GameState) -> bool {
        self.exit
    }

    fn set_creator(&mut self, creator: ScreenPointer) {
        self.creator = Some(creator);
    }

    fn new_screens(&mut self, _state: &mut GameState) -> Vec<ScreenPointer> {
        vec![]
    }

    fn render(&mut self, _delta: f64, _sate: &mut GameState, _fov: &tcod::map::Map, _tcod: &mut render::Tcod) -> (ScreenResult, Option<ModularWindow>) {
        let mut root = Offscreen::new(19, 5);
        root.set_default_foreground(colors::WHITE);
        root.print_rect_ex(
            3,
            2,
            19,
            5,
            BackgroundFlag::None,
            TextAlignment::Left,
            "You have won!"
        );
        (ScreenResult::PassThrough, Some(ModularWindow{screen: root, alpha: 1.0, pos: ModularWindowPosition::Center}))
    }

    fn tick(&mut self, _state: &mut GameState, _tcod: &mut render::Tcod, _actions: &mut Vec<Action>) -> ScreenResult {
        if self.exit {
            self.leave();
        }
        ScreenResult::PassThrough
    }

    fn handle_input(&mut self, input: &Input, _state: &mut GameState) -> ScreenResult {
        match input.key {
            Key { pressed: true, .. } => {
                self.exit = true;
            },
            _ => {}
        };
        ScreenResult::Stop
    }
}
