pub struct Simulation {
    accumulator: f32,
    pub tick_rate: f32,
}

impl Simulation {
    pub fn new(tick_rate: f32) -> Simulation {
        Simulation {
            accumulator: 0.0,
            tick_rate,
        }
    }
    pub fn tick(&mut self, amount: f32) -> bool {
        self.accumulator += amount;
        if self.accumulator >= 1.0 / self.tick_rate {
            self.accumulator = 0.0;
            return true;
        }
        return false;
    }
}
