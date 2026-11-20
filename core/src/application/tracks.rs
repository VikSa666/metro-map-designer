use crate::{
    domain::{MetroMap, NetworkNodeId, Track},
    errors::CustomError,
    SvgPoint,
};

impl MetroMap {
    pub fn create_track(
        self: &mut MetroMap,
        id: &str,
        origin: &str,
        destination: &str,
        geometry: Vec<SvgPoint>,
    ) -> Result<(), CustomError> {
        let origin_node_id = NetworkNodeId::from(origin);
        if !self.node_exists(&origin_node_id) {
            return Err(CustomError::NodeIdNotFound {
                id: origin.to_string(),
            });
        }

        let destination_node_id = NetworkNodeId::from(destination);
        if !self.node_exists(&destination_node_id) {
            return Err(CustomError::NodeIdNotFound {
                id: destination.to_string(),
            });
        }

        let track = Track::new(id, origin_node_id, destination_node_id, geometry);
        if !self.track_exists(track.id()) {
            self.push_track(track);
        }

        return Ok(());
    }
}
