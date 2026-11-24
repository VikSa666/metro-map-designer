use std::collections::{BTreeSet, HashMap};

use crate::{
    domain::{LineId, NetworkNodeId, Route, Track, TrackId},
    errors::CustomError,
    MetroMap,
};

impl MetroMap {
    /// # Route creator
    ///
    /// As the [`Route`]s are not saved in the `map`, they have to be created
    /// before to create a [`Line`].
    ///
    /// Routes use [`Track`]s, so if a track does not exist in the `map`, the
    /// route cannot be created.
    ///
    /// ## Errors
    /// - When a track id is not in the `map`.
    /// - When the given tracks are not connected. See [`connect_tracks`]
    pub fn create_route(
        self: &MetroMap,
        id: &str,
        name: Option<&str>,
        tracks: &[&str],
    ) -> Result<Route, CustomError> {
        let track_ids = self.connect_tracks(tracks)?;

        Ok(Route::new(
            id.to_string(),
            name.map(ToString::to_string),
            track_ids,
        ))
    }

    /// **Modifies** a route (of a given line) by appending an existing given track.
    ///
    /// The track must already have been declared in `map`.
    pub fn append_track_to_route(
        &mut self,
        line_id: LineId,
        route_id: String,
        new_track: Track,
        append_to_end: bool,
    ) -> Result<(), CustomError> {
        // Check if track already exists in the map
        if !self.track_exists(new_track.id()) {
            return Err(CustomError::TrackIdNotFound {
                id: new_track.id().to_string(),
            });
        }

        // Get the route boundary track, then release the mutable borrow before
        // looking up that track in the map.
        let boundary_track_id = {
            let Some(route) = self.get_route(&line_id, route_id.clone()) else {
                return Err(CustomError::RouteIdNotFound {
                    line_id: line_id.to_string(),
                    route_id,
                });
            };
            let boundary_track = if append_to_end {
                route.tracks().iter().last()
            } else {
                route.tracks().iter().next()
            };
            boundary_track
                .cloned()
                .ok_or(CustomError::TrackConnectionFailed)?
        };

        let is_connected = self
            .get_track(&boundary_track_id)
            .map(|track| {
                if append_to_end {
                    track.destination() == new_track.origin()
                } else {
                    track.origin() == new_track.destination()
                }
            })
            .ok_or(CustomError::TrackIdNotFound {
                id: boundary_track_id.to_string(),
            })?;

        if !is_connected {
            return Err(CustomError::TrackConnectionFailed);
        }

        let Some(route) = self.get_route_mut(&line_id, route_id.clone()) else {
            return Err(CustomError::RouteIdNotFound {
                line_id: line_id.to_string(),
                route_id,
            });
        };
        route.tracks_mut().insert(new_track.id().clone());
        Ok(())
    }

    /// Tries to connect a given array of tracks. To be fully connected, track
    /// `n` must end in the same station that track `n+1` starts.
    ///
    /// ## Errors
    /// - Unconnected tracks
    /// - Repeated tracks
    /// - Nonexisting tracks in the `map`.
    fn connect_tracks(&self, track_ids: &[&str]) -> Result<BTreeSet<TrackId>, CustomError> {
        let mut next: HashMap<&NetworkNodeId, &Track> = HashMap::new();
        for track_id in track_ids.into_iter().map(|id| TrackId::from(*id)) {
            let Some(track) = self.get_track(&track_id) else {
                return Err(CustomError::TrackIdNotFound {
                    id: track_id.to_string(),
                });
            };
            let start = track.origin();
            if next.contains_key(start) {
                return Err(CustomError::TrackConnectionFailed);
            }
            next.insert(start, track);
        }

        let mut sorted_tracks: BTreeSet<TrackId> = BTreeSet::new();
        let Some(first) = track_ids.first() else {
            return Err(CustomError::TrackConnectionFailed);
        };
        let Some(mut current_station) = self
            .get_track(&TrackId::from(*first))
            .map(|track| track.origin())
        else {
            return Err(CustomError::TrackIdNotFound {
                id: first.to_string(),
            });
        };

        while let Some(track) = next.remove(current_station) {
            current_station = track.destination();
            if !sorted_tracks.insert(track.id().clone()) {
                // Repeated track => error
                return Err(CustomError::TrackConnectionFailed);
            }
        }

        if !next.is_empty() {
            return Err(CustomError::TrackConnectionFailed);
        }
        return Ok(sorted_tracks);
    }
}
