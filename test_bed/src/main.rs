use yarge::{Entry, Game, error::ErrorType, keyboard::KeyboardKey, log_info, mouse::MouseButton};

struct TestBedGame;
impl Game for TestBedGame {
    fn on_start(&mut self) -> Result<(), ErrorType> {
        log_info!("Test bed starts");
        Ok(())
    }

    fn on_shutdown(&mut self) -> Result<(), ErrorType> {
        log_info!("Test bed ends");
        Ok(())
    }

    fn on_keyboard_key_pressed(&mut self, keyboard_key: KeyboardKey) -> Result<(), ErrorType> {
        log_info!("Keyboard's {:?} key pressed", keyboard_key);
        Ok(())
    }

    fn on_keyboard_key_released(&mut self, keyboard_key: KeyboardKey) -> Result<(), ErrorType> {
        log_info!("Keyboard's {:?} key released", keyboard_key);
        Ok(())
    }

    fn on_mouse_button_pressed(&mut self, mouse_button: MouseButton) -> Result<(), ErrorType> {
        log_info!("Mouse's {:?} button pressed", mouse_button);
        Ok(())
    }

    fn on_mouse_button_released(&mut self, mouse_button: MouseButton) -> Result<(), ErrorType> {
        log_info!("Mouse's {:?} button released", mouse_button);
        Ok(())
    }
}

fn main() {
    let config_file = None;
    if let Err(err) = Entry::run(&mut TestBedGame, config_file) {
        eprintln!("Failed to run the test bed: {:?}", err);
    }
}
