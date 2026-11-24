use crate::{
    domain::{MetroMap, Station, SvgIntPoint},
    errors::CustomError,
};

impl MetroMap {
    /// Creates a station in the map.
    ///
    /// TODO! Check if station already matches an existing one!
    pub fn create_station(
        self: &mut MetroMap,
        name: &str,
        svg_coordinates: SvgIntPoint,
        label_svg_coordinates: Option<SvgIntPoint>,
    ) -> Result<(), CustomError> {
        let station = Station::new(name, label_svg_coordinates);

        self.push_station(svg_coordinates, station);
        Ok(())
    }
}
