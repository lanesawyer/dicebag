use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// Tracks which players have been claimed by a browser in a campaign.
/// Stored as a single file per campaign alongside the campaign RON.
#[derive(Serialize, Deserialize, Default)]
pub struct CampaignClaims {
    claimed: HashSet<i32>,
}

impl CampaignClaims {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_claimed(&self, player_id: i32) -> bool {
        self.claimed.contains(&player_id)
    }

    pub fn claim(&mut self, player_id: i32) -> bool {
        self.claimed.insert(player_id)
    }

    pub fn release(&mut self, player_id: i32) {
        self.claimed.remove(&player_id);
    }
}
