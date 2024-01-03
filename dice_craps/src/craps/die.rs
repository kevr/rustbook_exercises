
pub struct Die {
    pub value: u8
}

impl Die {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn roll(&mut self) -> u8 {
        self.value = (rand::random::<u8>() % 6) + 1;
        self.value
    }
}
