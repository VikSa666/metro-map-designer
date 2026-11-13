use std::collections::{BTreeSet, HashSet};

use crate::{domain::Track, errors::CustomError, infrastructure::RouteDto};

use super::TrackId;

/// Representing a route for a specific line. Some metros have different
/// routes in the same line (e.g., London Metro)
#[derive(Debug, Clone)]
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

impl From<Route> for RouteDto {
    fn from(route: Route) -> Self {
        Self {
            id: route.id,
            name: route.name,
            tracks: route
                .tracks
                .into_iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

impl From<RouteDto> for Route {
    fn from(route: RouteDto) -> Self {
        Self {
            id: route.id,
            name: route.name,
            tracks: route
                .tracks
                .into_iter()
                .map(|track_id| TrackId::from(track_id.as_str()))
                .collect(),
        }
    }
}
