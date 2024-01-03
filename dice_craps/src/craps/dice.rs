use crate::craps::die::Die;

pub struct Dice {
    dice: Vec<Die>
}

impl Dice {
    pub fn new(n: u32) -> Self {
        let mut dice: Vec<Die> = Vec::new();
        (0..n).for_each(|_| dice.push(Die::new()));
        Self { dice }
    }

    pub fn roll(&mut self) -> Vec<u8> {
        let mut results: Vec<u8> = Vec::new();
        for dice in self.dice.iter_mut() {
            results.push(dice.roll());
        }
        results
    }

    pub fn total(&self) -> u8 {
        self.dice.iter().map(|d| d.value).sum()
    }
}
