use crate::{SvgIntPoint, domain::Station};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkNode {
    id: NetworkNodeId,
    node: Node,
    #[serde(rename(serialize = "SvgCoordinates", deserialize = "svg_coordinates"))]
    svg_coordinates: SvgIntPoint,
}

impl NetworkNode {
    pub fn new(node: Node, svg_coordinates: SvgIntPoint) -> Self {
        Self {
            id: NetworkNodeId::new(),
            node,
            svg_coordinates,
        }
    }

    pub fn id(&self) -> &NetworkNodeId {
        &self.id
    }

    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn station(&self) -> Option<&Station> {
        self.node.station()
    }

    pub fn x(&self) -> i32 {
        self.svg_coordinates.x
    }

    pub fn y(&self) -> i32 {
        self.svg_coordinates.y
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Node {
    Station(Station),
    Edge,
}

impl Node {
    pub fn new_station(station: Station) -> Self {
        Self::Station(station)
    }

    pub fn station(&self) -> Option<&Station> {
        match self {
            Self::Station(station) => Some(station),
            Self::Edge => None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkNodeId(String);

impl NetworkNodeId {
    fn new() -> Self {
        let uuid = uuid::Uuid::new_v4();
        Self(format!("station-{uuid}"))
    }
}

impl From<&str> for NetworkNodeId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for NetworkNodeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for NetworkNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
