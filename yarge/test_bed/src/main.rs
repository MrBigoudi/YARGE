use core_layer::{Entry, Game, info};
use error::ErrorType;

struct TestBedGame;
impl Game for TestBedGame {
    fn on_start(&mut self) -> Result<(), ErrorType> {
        info!("Test bed starts");
        Ok(())
    }

    fn on_shutdown(&mut self) -> Result<(), ErrorType> {
        info!("Test bed ends");
        Ok(())
    }
}

fn main() {
    let config_file = None;
    Entry::run(&mut TestBedGame, config_file).unwrap();
}
