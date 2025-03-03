use rand::{Rng, rng};
use std::{num::NonZeroU8, str::FromStr};

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl FromStr for DiceType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d4" => Ok(DiceType::D4),
            "d6" => Ok(DiceType::D6),
            "d8" => Ok(DiceType::D8),
            "d10" => Ok(DiceType::D10),
            "d12" => Ok(DiceType::D12),
            "d20" => Ok(DiceType::D20),
            "d100" => Ok(DiceType::D100),
            _ => Err("Invalid dice type"),
        }
    }
}

fn random_number(die: DiceType) -> i64 {
    let mut rng = rng();
    rng.random_range(1..=die.into())
}

pub struct Roll {
    pub number: NonZeroU8,
    pub dice: DiceType,
}

impl Roll {
    pub fn roll(roll: &Roll) -> Vec<i64> {
        (0..roll.number.into())
            .map(|_| random_number(roll.dice))
            .collect::<Vec<i64>>()
    }

    pub fn roll_one(dice: DiceType) -> i64 {
        random_number(dice)
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
