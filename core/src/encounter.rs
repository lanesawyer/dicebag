use crate::{Entity, Player};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CombatantRef {
    Player(i32), // -> Player.id
    Entity(i32), // -> Entity.id
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Participant {
    id: i32,
    source: CombatantRef,
    max_hp: i32,
    current_hp: i32,
    initiative: Option<i32>,
}

impl Participant {
    pub fn from_player(id: i32, player: &Player, max_hp: i32) -> Self {
        Participant {
            id,
            source: CombatantRef::Player(player.id()),
            max_hp,
            current_hp: max_hp,
            initiative: None,
        }
    }

    pub fn from_entity(id: i32, entity: &Entity, max_hp_override: Option<i32>) -> Self {
        let max_hp = max_hp_override.unwrap_or(entity.max_hp());
        Participant {
            id,
            source: CombatantRef::Entity(entity.id()),
            max_hp,
            current_hp: max_hp,
            initiative: None,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn source(&self) -> &CombatantRef {
        &self.source
    }

    pub fn max_hp(&self) -> i32 {
        self.max_hp
    }

    pub fn current_hp(&self) -> i32 {
        self.current_hp
    }

    pub fn initiative(&self) -> Option<i32> {
        self.initiative
    }

    pub fn set_initiative(&mut self, initiative: i32) {
        self.initiative = Some(initiative);
    }

    pub fn clear_initiative(&mut self) {
        self.initiative = None;
    }

    /// Apply damage (positive) or healing (negative). HP is clamped to [0, max_hp].
    pub fn apply_hp_change(&mut self, delta: i32) {
        self.current_hp = (self.current_hp - delta).clamp(0, self.max_hp);
    }

    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Encounter {
    id: i32,
    name: String,
    campaign_id: i32,
    participants: Vec<Participant>,
}

impl Encounter {
    pub fn new(id: i32, name: String, campaign_id: i32) -> Self {
        Encounter {
            id,
            name,
            campaign_id,
            participants: vec![],
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn campaign_id(&self) -> i32 {
        self.campaign_id
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }

    pub fn participants(&self) -> &[Participant] {
        &self.participants
    }

    pub fn participant_mut(&mut self, id: i32) -> Option<&mut Participant> {
        self.participants.iter_mut().find(|p| p.id == id)
    }

    /// Returns participants sorted by initiative (descending). Participants without
    /// initiative are placed at the end in insertion order.
    pub fn initiative_order(&self) -> Vec<&Participant> {
        let mut with_init: Vec<&Participant> = self
            .participants
            .iter()
            .filter(|p| p.initiative.is_some())
            .collect();
        let without_init: Vec<&Participant> = self
            .participants
            .iter()
            .filter(|p| p.initiative.is_none())
            .collect();

        with_init.sort_by(|a, b| b.initiative.unwrap().cmp(&a.initiative.unwrap()));
        with_init.extend(without_init);
        with_init
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Entity, EntityKind, Player};

    fn make_encounter() -> Encounter {
        Encounter::new(1, "Test Encounter".to_string(), 1)
    }

    #[test]
    fn from_player_sets_source() {
        let player = Player::new(42, "Aragorn".to_string());
        let p = Participant::from_player(1, &player, 40);
        assert_eq!(p.source(), &CombatantRef::Player(42));
        assert_eq!(p.max_hp(), 40);
        assert_eq!(p.current_hp(), 40);
    }

    #[test]
    fn from_entity_uses_default_hp() {
        let entity = Entity::new(7, "Goblin".to_string(), EntityKind::Enemy, 10);
        let p = Participant::from_entity(1, &entity, None);
        assert_eq!(p.source(), &CombatantRef::Entity(7));
        assert_eq!(p.max_hp(), 10);
    }

    #[test]
    fn from_entity_respects_hp_override() {
        let entity = Entity::new(7, "Boss Goblin".to_string(), EntityKind::Enemy, 10);
        let p = Participant::from_entity(1, &entity, Some(30));
        assert_eq!(p.max_hp(), 30);
    }

    #[test]
    fn apply_damage_clamps_to_zero() {
        let entity = Entity::new(1, "Goblin".to_string(), EntityKind::Enemy, 10);
        let mut p = Participant::from_entity(1, &entity, None);
        p.apply_hp_change(15);
        assert_eq!(p.current_hp(), 0);
        assert!(!p.is_alive());
    }

    #[test]
    fn apply_healing_clamps_to_max() {
        let player = Player::new(1, "Cleric".to_string());
        let mut p = Participant::from_player(1, &player, 20);
        p.apply_hp_change(5); // take 5 damage
        p.apply_hp_change(-10); // heal 10
        assert_eq!(p.current_hp(), 20);
    }

    #[test]
    fn initiative_order_sorted_descending() {
        let mut enc = make_encounter();
        let player = Player::new(1, "Slow".to_string());
        let mut p1 = Participant::from_player(1, &player, 10);
        p1.set_initiative(5);
        let entity = Entity::new(2, "Fast".to_string(), EntityKind::Enemy, 10);
        let mut p2 = Participant::from_entity(2, &entity, None);
        p2.set_initiative(18);
        let ally = Entity::new(3, "NoInit".to_string(), EntityKind::AllyNpc, 10);
        let p3 = Participant::from_entity(3, &ally, None);
        enc.add_participant(p1);
        enc.add_participant(p2);
        enc.add_participant(p3);

        let order = enc.initiative_order();
        assert_eq!(order[0].source(), &CombatantRef::Entity(2));
        assert_eq!(order[1].source(), &CombatantRef::Player(1));
        assert_eq!(order[2].source(), &CombatantRef::Entity(3));
    }
}
