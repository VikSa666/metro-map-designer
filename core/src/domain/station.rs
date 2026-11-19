use crate::domain::Id;

use super::SvgPoint;

#[derive(PartialEq, Eq, Hash, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StationId(String);

impl Id<'_> for StationId {
    fn new(id: &str) -> Self {
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Station {
    id: StationId,
    name: String,
    // TODO: move to node
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
