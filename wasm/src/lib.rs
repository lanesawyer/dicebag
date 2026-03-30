use core::{
    AudioCatalog, Campaign, CampaignClaims, DiceType, Encounter, EntityRoster, PlayerRoster, Roll,
};
use ron::ser::PrettyConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn roll_d4() -> i64 {
    Roll::roll_one(DiceType::D4)
}

#[wasm_bindgen]
pub fn roll_d6() -> i64 {
    Roll::roll_one(DiceType::D6)
}

#[wasm_bindgen]
pub fn roll_d8() -> i64 {
    Roll::roll_one(DiceType::D8)
}

#[wasm_bindgen]
pub fn roll_d10() -> i64 {
    Roll::roll_one(DiceType::D10)
}

#[wasm_bindgen]
pub fn roll_d12() -> i64 {
    Roll::roll_one(DiceType::D12)
}

#[wasm_bindgen]
pub fn roll_d20() -> i64 {
    Roll::roll_one(DiceType::D20)
}

#[wasm_bindgen]
pub fn roll_d100() -> i64 {
    Roll::roll_one(DiceType::D100)
}

/// Parse RON player roster file into a JS object.
#[wasm_bindgen]
pub fn parse_player_roster(ron_str: &str) -> Result<JsValue, JsValue> {
    let roster: PlayerRoster =
        ron::from_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&roster).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Serialize a JS player roster back to RON string.
#[wasm_bindgen]
pub fn player_roster_to_ron(val: JsValue) -> Result<String, JsValue> {
    let roster: PlayerRoster =
        serde_wasm_bindgen::from_value(val).map_err(|e| JsValue::from_str(&e.to_string()))?;
    ron::ser::to_string_pretty(&roster, PrettyConfig::default())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parse RON entity roster file into a JS object.
#[wasm_bindgen]
pub fn parse_entity_roster(ron_str: &str) -> Result<JsValue, JsValue> {
    let roster: EntityRoster =
        ron::from_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&roster).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Serialize a JS entity roster back to RON string.
#[wasm_bindgen]
pub fn entity_roster_to_ron(val: JsValue) -> Result<String, JsValue> {
    let roster: EntityRoster =
        serde_wasm_bindgen::from_value(val).map_err(|e| JsValue::from_str(&e.to_string()))?;
    ron::ser::to_string_pretty(&roster, PrettyConfig::default())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parse RON file content into a JS object.
#[wasm_bindgen]
pub fn parse_campaign(ron_str: &str) -> Result<JsValue, JsValue> {
    let campaign: Campaign =
        ron::from_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&campaign).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Serialize a JS campaign object back to RON string.
#[wasm_bindgen]
pub fn campaign_to_ron(val: JsValue) -> Result<String, JsValue> {
    let campaign: Campaign =
        serde_wasm_bindgen::from_value(val).map_err(|e| JsValue::from_str(&e.to_string()))?;
    ron::ser::to_string_pretty(&campaign, PrettyConfig::default())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parse RON encounter file content into a JS object.
#[wasm_bindgen]
pub fn parse_encounter(ron_str: &str) -> Result<JsValue, JsValue> {
    let encounter: Encounter =
        ron::from_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&encounter).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Serialize a JS encounter object back to RON string.
#[wasm_bindgen]
pub fn encounter_to_ron(val: JsValue) -> Result<String, JsValue> {
    let encounter: Encounter =
        serde_wasm_bindgen::from_value(val).map_err(|e| JsValue::from_str(&e.to_string()))?;
    ron::ser::to_string_pretty(&encounter, PrettyConfig::default())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parse RON audio catalog file into a JS object. Accepts empty string for a fresh catalog.
#[wasm_bindgen]
pub fn parse_audio_catalog(ron_str: &str) -> Result<JsValue, JsValue> {
    let catalog: AudioCatalog = if ron_str.is_empty() {
        AudioCatalog::new()
    } else {
        ron::from_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?
    };
    serde_wasm_bindgen::to_value(&catalog).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Serialize a JS audio catalog back to RON string.
#[wasm_bindgen]
pub fn audio_catalog_to_ron(val: JsValue) -> Result<String, JsValue> {
    let catalog: AudioCatalog =
        serde_wasm_bindgen::from_value(val).map_err(|e| JsValue::from_str(&e.to_string()))?;
    ron::ser::to_string_pretty(&catalog, PrettyConfig::default())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Returns true if the player is claimed. Accepts empty string for a fresh claims file.
#[wasm_bindgen]
pub fn claims_is_claimed(ron_str: &str, player_id: i32) -> Result<bool, JsValue> {
    let claims: CampaignClaims = if ron_str.is_empty() {
        CampaignClaims::new()
    } else {
        ron::from_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?
    };
    Ok(claims.is_claimed(player_id))
}

/// Adds the player to the claims and returns the updated RON string.
#[wasm_bindgen]
pub fn claims_add(ron_str: &str, player_id: i32) -> Result<String, JsValue> {
    let mut claims: CampaignClaims = if ron_str.is_empty() {
        CampaignClaims::new()
    } else {
        ron::from_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?
    };
    claims.claim(player_id);
    ron::ser::to_string_pretty(&claims, PrettyConfig::default())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Removes the player from the claims and returns the updated RON string.
#[wasm_bindgen]
pub fn claims_remove(ron_str: &str, player_id: i32) -> Result<String, JsValue> {
    let mut claims: CampaignClaims = if ron_str.is_empty() {
        CampaignClaims::new()
    } else {
        ron::from_str(ron_str).map_err(|e| JsValue::from_str(&e.to_string()))?
    };
    claims.release(player_id);
    ron::ser::to_string_pretty(&claims, PrettyConfig::default())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
