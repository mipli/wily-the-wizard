use std::cmp::{max};
use tcod::console::*;
use tcod::colors;

pub fn get_menu<T: AsRef<str>>(
    options: &[T]
) -> Offscreen {
    assert!(options.len() <= 26, "A menu cannot have more than 26 options");

    let mut width = 15;

    for option in options {
        let text = option.as_ref();
        width = max(width, text.len() as i32);
    }
    let height = max(options.len() as i32, 1);
    let mut window = Offscreen::new(width, height);

    window.set_default_foreground(colors::WHITE);
    for (offset, text) in options.iter().enumerate() {
        window.print_rect_ex(
            0,
            offset as i32,
            width,
            1,
            BackgroundFlag::None,
            TextAlignment::Left,
            text.as_ref()
        );
    }

    window
}
