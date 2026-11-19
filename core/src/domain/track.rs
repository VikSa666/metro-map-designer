use crate::domain::Id;

use super::NetworkNodeId;

#[derive(
    Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct TrackId(String);

impl Id<'_> for TrackId {
    fn new(id: &str) -> Self {
        let normalized_id = if id.starts_with("track-") {
            id.to_string()
        } else {
            format!("track-{id}")
        };

        Self(normalized_id)
    }
}

impl From<&str> for TrackId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for TrackId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for TrackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// TODO: Directionality
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Track {
    id: TrackId,
    origin: NetworkNodeId,
    destination: NetworkNodeId,
}

impl Track {
    pub fn new(id: &str, origin: NetworkNodeId, destination: NetworkNodeId) -> Self {
        Self {
            id: TrackId::new(id),
            origin,
            destination,
        }
    }

    pub fn id(&self) -> &TrackId {
        &self.id
    }

    pub fn origin(&self) -> &NetworkNodeId {
        &self.origin
    }

    pub fn destination(&self) -> &NetworkNodeId {
        &self.destination
    }
}
