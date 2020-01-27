/// Represents a Nazara application
pub struct Application {
    should_close: bool,
}

impl Application {
    pub fn new() -> Application {
        Application {
            should_close: false,
        }
    }

    pub fn execute(&mut self) -> bool {
        !self.should_close
    }

    pub fn run(mut self) {
        while self.execute() {}
    }
}
