use serde::{Deserialize, Serialize};

/// Who the recording is associated with — a player (by id) or an entity (by id).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AudioSubject {
    Player(i32),
    Entity(i32),
}

/// Metadata for one audio recording. The actual audio bytes live on disk next to
/// the campaign files; this struct just tracks the filename and descriptive info.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AudioRecording {
    id: i32,
    /// Human-readable label, e.g. "Goblin war cry".
    label: String,
    /// Bare filename (no directory), e.g. "my-campaign-audio-1.webm".
    filename: String,
    /// Which player or entity this recording belongs to.
    subject: AudioSubject,
    /// Optional free-form notes about the recording.
    notes: String,
}

impl AudioRecording {
    pub fn new(id: i32, label: String, filename: String, subject: AudioSubject) -> Self {
        AudioRecording {
            id,
            label,
            filename,
            subject,
            notes: String::new(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn subject(&self) -> &AudioSubject {
        &self.subject
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
    }
}

/// Catalog of all audio recordings for a campaign.
#[derive(Serialize, Deserialize, Default)]
pub struct AudioCatalog {
    recordings: Vec<AudioRecording>,
}

impl AudioCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recordings(&self) -> &[AudioRecording] {
        &self.recordings
    }

    pub fn get(&self, id: i32) -> Option<&AudioRecording> {
        self.recordings.iter().find(|r| r.id == id)
    }

    pub fn add(&mut self, recording: AudioRecording) {
        self.recordings.push(recording);
    }

    /// Returns the filename of the removed recording so the caller can delete
    /// the audio file from disk, or `None` if no recording with that id exists.
    pub fn remove(&mut self, id: i32) -> Option<String> {
        let pos = self.recordings.iter().position(|r| r.id == id)?;
        let removed = self.recordings.remove(pos);
        Some(removed.filename)
    }

    pub fn next_id(&self) -> i32 {
        self.recordings.iter().map(|r| r.id).max().unwrap_or(0) + 1
    }

    /// All recordings belonging to a specific player.
    pub fn for_player(&self, player_id: i32) -> Vec<&AudioRecording> {
        self.recordings
            .iter()
            .filter(|r| r.subject == AudioSubject::Player(player_id))
            .collect()
    }

    /// All recordings belonging to a specific entity.
    pub fn for_entity(&self, entity_id: i32) -> Vec<&AudioRecording> {
        self.recordings
            .iter()
            .filter(|r| r.subject == AudioSubject::Entity(entity_id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_recording(id: i32, subject: AudioSubject) -> AudioRecording {
        AudioRecording::new(
            id,
            format!("Recording {id}"),
            format!("campaign-audio-{id}.webm"),
            subject,
        )
    }

    #[test]
    fn next_id_empty_catalog() {
        assert_eq!(AudioCatalog::new().next_id(), 1);
    }

    #[test]
    fn next_id_increments() {
        let mut cat = AudioCatalog::new();
        cat.add(make_recording(1, AudioSubject::Player(1)));
        cat.add(make_recording(3, AudioSubject::Entity(2)));
        assert_eq!(cat.next_id(), 4);
    }

    #[test]
    fn remove_returns_filename() {
        let mut cat = AudioCatalog::new();
        cat.add(make_recording(1, AudioSubject::Player(1)));
        let fname = cat.remove(1);
        assert_eq!(fname, Some("campaign-audio-1.webm".to_string()));
        assert!(cat.recordings().is_empty());
    }

    #[test]
    fn remove_missing_returns_none() {
        let mut cat = AudioCatalog::new();
        assert_eq!(cat.remove(99), None);
    }

    #[test]
    fn filter_by_subject() {
        let mut cat = AudioCatalog::new();
        cat.add(make_recording(1, AudioSubject::Player(1)));
        cat.add(make_recording(2, AudioSubject::Entity(5)));
        cat.add(make_recording(3, AudioSubject::Player(1)));
        assert_eq!(cat.for_player(1).len(), 2);
        assert_eq!(cat.for_entity(5).len(), 1);
        assert!(cat.for_player(99).is_empty());
    }
}
