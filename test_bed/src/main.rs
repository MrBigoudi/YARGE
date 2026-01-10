#[allow(unused)]
use yarge::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use yarge::{
    Entry, Event, Game, event_builder,
    keyboard::{KeyboardKey, Special},
    mouse::MouseButton,
};

use std::collections::VecDeque;

struct TestBedGame {}

impl TestBedGame {
    pub fn new() -> Self {
        Self {}
    }
}

impl Game for TestBedGame {
    fn on_update(&mut self, _delta_time: f64) -> Result<VecDeque<Event>, ErrorType> {
        Ok(VecDeque::new())
    }

    fn on_start(&mut self) -> Result<VecDeque<Event>, ErrorType> {
        log_info!("Test bed starts");
        Ok(VecDeque::new())
    }

    fn on_shutdown(&mut self) -> Result<VecDeque<Event>, ErrorType> {
        log_info!("Test bed ends");
        Ok(VecDeque::new())
    }

    fn on_keyboard_key_pressed(
        &mut self,
        keyboard_key: KeyboardKey,
    ) -> Result<VecDeque<Event>, ErrorType> {
        let mut events = VecDeque::new();
        log_info!("Keyboard's {:?} key pressed", keyboard_key);
        if keyboard_key == KeyboardKey::Special(Special::Escape) {
            events.push_back(event_builder::QuitAppEventBuilder::build()?);
        }
        Ok(events)
    }

    fn on_keyboard_key_released(
        &mut self,
        keyboard_key: KeyboardKey,
    ) -> Result<VecDeque<Event>, ErrorType> {
        log_info!("Keyboard's {:?} key released", keyboard_key);
        Ok(VecDeque::new())
    }

    fn on_mouse_button_pressed(
        &mut self,
        mouse_button: MouseButton,
    ) -> Result<VecDeque<Event>, ErrorType> {
        log_info!("Mouse's {:?} button pressed", mouse_button);
        Ok(VecDeque::new())
    }

    fn on_mouse_button_released(
        &mut self,
        mouse_button: MouseButton,
    ) -> Result<VecDeque<Event>, ErrorType> {
        log_info!("Mouse's {:?} button released", mouse_button);
        Ok(VecDeque::new())
    }
}

fn main() {
    let config_file = None;
    let mut game = TestBedGame::new();
    if let Err(err) = Entry::run(&mut game, config_file) {
        eprintln!("Failed to run the test bed: {:?}", err);
    }
}
