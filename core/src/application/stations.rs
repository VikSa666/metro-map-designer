use crate::{
    domain::{MetroMap, Station, SvgPoint},
    errors::CustomError,
};

impl MetroMap {
    /// Creates a station in the map.
    pub fn create_station(
        self: &mut MetroMap,
        id: &str,
        name: &str,
        svg_coordinates: SvgPoint,
        label_svg_coordinates: Option<SvgPoint>,
    ) -> Result<(), CustomError> {
        let station = Station::new(id, name, svg_coordinates, label_svg_coordinates);
        if !self.station_exists(station.id()) {
            self.push_station(station);
            Ok(())
        } else {
            Err(CustomError::Custom(format!(
                "Station `{id}` already exists."
            )))
        }
    }
}
