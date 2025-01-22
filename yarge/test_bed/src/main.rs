use core_layer::{App, Game};
use error::ErrorType;

struct TestBedGame;
impl Game for TestBedGame {
    fn on_start(&mut self) -> Result<(), ErrorType> {
        println!("Test bed starts");
        Ok(())
    }
}

fn main() {
    App::run(&mut TestBedGame).unwrap();
}
