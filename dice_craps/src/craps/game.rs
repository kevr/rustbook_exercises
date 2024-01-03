
use crate::craps::dice::Dice;

const NOPASS: [u8; 2] = [2, 3];
const PASS: [u8; 2] = [7, 11];
const PUSH: [u8; 1] = [12];

const ENDING_ROLLS: [u8; 5] = [
    NOPASS[0],
    NOPASS[1],
    PASS[0],
    PASS[1],
    PUSH[0],
];

pub struct Game {
    dice: Dice,
    end_of_turn: bool
}

impl Game {
    // Constructor
    pub fn new() -> Self {
        Self {
            dice: Dice::new(2),
            end_of_turn: false
        }
    }

    fn roll(&mut self) -> u8 {
        self.dice.roll();
        self.dice.total()
    }

    pub fn push(&self) -> bool {
        let total = self.dice.total();
        PUSH.contains(&total)
    }

    pub fn pass(&self) -> bool {
        let total = self.dice.total();
        PASS.contains(&total)
    }

    pub fn nopass(&self) -> bool {
        let total = self.dice.total();
        NOPASS.contains(&total)
    }

    // A come-out roll
    //
    // Sets `self.end_of_turn` if the come-out roll hits a sentinel value
    // Otherwise, `self.end_of_turn` is set to false
    //
    pub fn come_out(&mut self) -> u8 {
        let total = self.roll();
        self.end_of_turn = ENDING_ROLLS.contains(&total);
        total
    }

    pub fn point(&mut self, p: u8) -> u8 {
        let total = self.roll();
        self.end_of_turn = total == p || total == 7;
        total
    }

    pub fn end_of_turn(&self) -> bool {
        self.end_of_turn
    }
}
