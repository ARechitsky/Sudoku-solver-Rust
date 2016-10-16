use time::precise_time_ns;
use std::default::Default;

#[derive(PartialEq)]
enum State {
    NotStarted,
    Started,
    Ended
}

impl Default for State {
    fn default() -> Self {
        State::NotStarted
    }
}

#[derive(Default)]
pub struct Timer {
    start: u64,
    end: u64,
    state: State
}

impl Timer {
    pub fn start(&mut self) {
        if self.state != State::NotStarted { panic!() }
        self.start = precise_time_ns();
        self.state = State::Started
    }

    pub fn stop(&mut self) {
        if self.state != State::Started { panic!() }
        self.end = precise_time_ns();
        self.state = State::Ended
    }

    pub fn get_time(&self) -> f64 {
        if self.state != State::Ended { panic!() }
            ((self.end - self.start) as f64) / 1e9
    }
}
