use crate::infrastructure::TrackDto;

use super::{StationId, SvgPoint};

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct TrackId(String);

impl TrackId {
    pub fn new(id: &str) -> Self {
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
#[derive(Debug, Clone)]
pub struct Track {
    id: TrackId,
    origin_station: StationId,
    destination_station: StationId,
    edges: Vec<SvgPoint>,
}

impl Track {
    pub fn new(
        id: &str,
        origin_station: StationId,
        destination_station: StationId,
        edges: Vec<SvgPoint>,
    ) -> Self {
        Self {
            id: TrackId::new(id),
            origin_station,
            destination_station,
            edges,
        }
    }

    pub fn id(&self) -> &TrackId {
        &self.id
    }

    pub fn origin(&self) -> &StationId {
        &self.origin_station
    }

    pub fn destination(&self) -> &StationId {
        &self.destination_station
    }
}

impl From<Track> for TrackDto {
    fn from(track: Track) -> Self {
        Self {
            id: track.id.to_string(),
            from: track.origin_station.to_string(),
            to: track.destination_station.to_string(),
            edges: track.edges.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<TrackDto> for Track {
    fn from(track: TrackDto) -> Self {
        Self {
            id: track.id.into(),
            origin_station: track.from.into(),
            destination_station: track.to.into(),
            edges: track.edges.into_iter().map(Into::into).collect(),
        }
    }
}
