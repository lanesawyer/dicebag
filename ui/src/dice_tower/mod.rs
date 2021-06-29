use rand::{thread_rng, Rng};
use std::num::NonZeroU8;

pub mod tower;

#[derive(Clone, Copy)]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

pub struct Roll {
    pub number: NonZeroU8,
    pub dice: DiceType,
}

impl Roll {
    fn roll(roll: &Roll) -> Vec<i64> {
        let mut rng = thread_rng();
        (0..roll.number.into())
            .map(|_| rng.gen_range(1..=roll.dice.into()))
            .collect::<Vec<i64>>()
    }
}

impl From<DiceType> for i64 {
    fn from(die: DiceType) -> Self {
        match die {
            DiceType::D4 => 4,
            DiceType::D6 => 6,
            DiceType::D8 => 8,
            DiceType::D10 => 10,
            DiceType::D12 => 12,
            DiceType::D20 => 20,
            DiceType::D100 => 100,
        }
    }
}
