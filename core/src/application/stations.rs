use crate::{
    domain::{MetroMap, Station, SvgPoint},
    errors::CustomError,
};

impl MetroMap {
    /// Creates a station in the map.
    ///
    /// TODO! Check if station already matches an existing one!
    pub fn create_station(
        self: &mut MetroMap,
        name: &str,
        svg_coordinates: SvgPoint,
        label_svg_coordinates: Option<SvgPoint>,
    ) -> Result<(), CustomError> {
        let station = Station::new(name, svg_coordinates, label_svg_coordinates);

        self.push_station(station);
        Ok(())
    }
}
