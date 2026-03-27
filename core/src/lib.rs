mod campaign;
mod db;
mod dice;
mod encounter;
mod entity;
mod player;

pub use campaign::Campaign;
pub use db::Persistable;
pub use dice::{DiceType, Roll};
pub use encounter::{CombatantRef, Encounter, Participant};
pub use entity::{Entity, EntityKind};
pub use player::Player;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
