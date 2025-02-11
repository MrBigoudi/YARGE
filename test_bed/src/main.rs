use yarge::{Entry, Game, error::ErrorType, log_info};

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
}

fn main() {
    let config_file = None;
    Entry::run(&mut TestBedGame, config_file).unwrap();
}
