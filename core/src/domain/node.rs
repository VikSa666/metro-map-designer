use crate::domain::{Id, Station, StationId};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkNode {
    id: NetworkNodeId,
    node: Node,
}

impl NetworkNode {
    pub fn new(id: &str, node: Node) -> Self {
        Self {
            id: id.into(),
            node,
        }
    }

    pub fn id(&self) -> &NetworkNodeId {
        &self.id
    }

    pub fn station(&self) -> Option<&Station> {
        self.node.station()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
pub enum NetworkNodeId {
    StationId(StationId),
    Edge,
}

impl From<&str> for NetworkNodeId {
    fn from(value: &str) -> Self {
        if value.starts_with("station-") {
            let node_id = value.replacen("station-", "node-", 1);
            Self::StationId(StationId::new(&node_id))
        } else {
            panic!("Invalid network node id: expected a station- prefix")
        }
    }
}

impl From<String> for NetworkNodeId {
    fn from(value: String) -> Self {
        if value.starts_with("station-") {
            let node_id = value.replacen("station-", "node-", 1);
            Self::StationId(StationId::new(&node_id))
        } else {
            panic!("Invalid network node id: expected a station- prefix")
        }
    }
}

impl std::fmt::Display for NetworkNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StationId(s) => write!(f, "{s}"),
            Self::Edge => todo!(),
        }
    }
}

impl From<Station> for NetworkNode {
    fn from(value: Station) -> Self {
        Self {
            id: NetworkNodeId::StationId(value.id().to_owned()),
            node: Node::Station(value),
        }
    }
}
