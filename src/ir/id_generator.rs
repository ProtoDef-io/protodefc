pub struct IdGenerator {
    id: u64,
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator {
            id: 0,
        }
    }
    pub fn get(&mut self) -> u64 {
        self.id += 1;
        self.id
    }
}
