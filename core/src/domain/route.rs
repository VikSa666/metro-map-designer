use std::collections::BTreeSet;

use super::TrackId;

/// Representing a route for a specific line. Some metros have different
/// routes in the same line (e.g., London Metro)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Route {
    id: String,
    name: Option<String>,
    tracks: BTreeSet<TrackId>,
}

impl Route {
    pub fn new(id: String, name: Option<String>, tracks: BTreeSet<TrackId>) -> Self {
        Self { id, name, tracks }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn tracks(&self) -> &BTreeSet<TrackId> {
        &self.tracks
    }

    pub fn tracks_mut(&mut self) -> &mut BTreeSet<TrackId> {
        &mut self.tracks
    }
}
