use crate::infrastructure::StationDto;

use super::SvgPoint;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct StationId(String);

impl StationId {
    pub fn new(id: &str) -> Self {
        let normalized_id = if id.starts_with("station-") {
            id.to_string()
        } else {
            format!("station-{id}")
        };

        Self(normalized_id)
    }
}

impl From<&str> for StationId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for StationId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for StationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Station {
    id: StationId,
    name: String,
    svg_coordinates: SvgPoint,
    label_svg_coordinates: Option<SvgPoint>,
}

impl Station {
    pub fn new(
        id: &str,
        name: &str,
        svg_coordinates: SvgPoint,
        label_svg_coordinates: Option<SvgPoint>,
    ) -> Self {
        Self {
            id: StationId::new(id),
            name: name.to_string(),
            svg_coordinates,
            label_svg_coordinates,
        }
    }

    pub fn id(&self) -> &StationId {
        &self.id
    }
}

impl From<Station> for StationDto {
    fn from(station: crate::domain::Station) -> Self {
        Self {
            id: station.id.to_string(),
            name: station.name,
            svg_coordinates: station.svg_coordinates.into(),
            label_svg_coordinates: station.label_svg_coordinates.map(Into::into),
        }
    }
}

impl From<StationDto> for crate::domain::Station {
    fn from(station: StationDto) -> Self {
        Self {
            id: station.id.into(),
            name: station.name,
            svg_coordinates: station.svg_coordinates.into(),
            label_svg_coordinates: station.label_svg_coordinates.map(Into::into),
        }
    }
}
