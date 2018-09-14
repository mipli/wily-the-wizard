use tcod::colors::Color;
use tcod::colors;
use crate::consts::*;

pub type Messages = Vec<(String, Color)>;

pub enum MessageLevel {
    Info,
    Spell,
    Important
}

pub trait MessageLog {
    fn add<T: Into<String>>(&mut self, message: T, color: Color);
    fn log<T: Into<String>>(&mut self, level: MessageLevel, message: T);
}

impl MessageLog for Messages {
    fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        if self.len() == LOG_MEMORY as usize {
            self.remove(0);
        }
        self.push((message.into(), color));
    }

    fn log<T: Into<String>>(&mut self, level: MessageLevel, message: T) {
        match level {
            MessageLevel::Info => self.add(message, colors::WHITE),
            MessageLevel::Important => self.add(message, colors::RED),
            MessageLevel::Spell => self.add(message, colors::SKY),
        }
    }
}
