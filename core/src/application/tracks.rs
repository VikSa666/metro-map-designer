use crate::{
    domain::{MetroMap, StationId, SvgPoint, Track},
    errors::CustomError,
};

impl MetroMap {
    pub fn create_track(
        self: &mut MetroMap,
        id: &str,
        origin_station: &str,
        destination_station: &str,
        edges: Vec<SvgPoint>,
    ) -> Result<(), CustomError> {
        let origin_station_id = StationId::from(origin_station);
        if !self.station_exists(&origin_station_id) {
            return Err(CustomError::StationIdNotFound {
                id: origin_station.to_string(),
            });
        }

        let destination_station_id = StationId::from(destination_station);
        if !self.station_exists(&destination_station_id) {
            return Err(CustomError::StationIdNotFound {
                id: destination_station.to_string(),
            });
        }

        let track = Track::new(id, origin_station_id, destination_station_id, edges);
        if !self.track_exists(track.id()) {
            self.push_track(track);
        }

        return Ok(());
    }
}
