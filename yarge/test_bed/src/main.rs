use core_layer::{Entry, Game};
use error::ErrorType;

struct TestBedGame;
impl Game for TestBedGame {
    fn on_start(&mut self) -> Result<(), ErrorType> {
        println!("Test bed starts");
        Ok(())
    }
}

fn main() {
    let config_file = None;
    Entry::run(&mut TestBedGame, config_file).unwrap();
}
