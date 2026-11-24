mod line;
pub use line::*;
mod route;
pub use route::*;
mod station;
pub use station::*;
mod track;
pub use track::*;
mod node;
pub use node::*;

use crate::errors::CustomError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetroMap {
    id: String,
    nodes: Vec<NetworkNode>,
    lines: Vec<Line>,
    tracks: Vec<Track>,
}

impl MetroMap {
    pub fn new(id: &str, nodes: Vec<NetworkNode>, lines: Vec<Line>, tracks: Vec<Track>) -> Self {
        Self {
            id: id.to_string(),
            nodes,
            lines,
            tracks,
        }
    }

    pub fn new_empty(id: &str) -> Self {
        Self {
            id: id.to_string(),
            nodes: vec![],
            lines: vec![],
            tracks: vec![],
        }
    }

    pub fn nodes(&self) -> &Vec<NetworkNode> {
        &self.nodes
    }

    pub fn node_exists(&self, node_id: &NetworkNodeId) -> bool {
        self.nodes().iter().any(|node| node.id() == node_id)
    }

    pub fn node_by_id(&self, node_id: &NetworkNodeId) -> Result<&NetworkNode, CustomError> {
        let mut node = self.nodes().iter().filter(|node| node.id() == node_id);
        let Some(value) = node.next() else {
            return Err(CustomError::NodeIdNotFound {
                id: node_id.to_string(),
            });
        };
        if node.next().is_some() {
            return Err(CustomError::Custom(format!(
                "Id {node_id} is present more than once in map."
            )));
        }
        Ok(value)
    }

    pub fn stations(&self) -> Vec<&Station> {
        self.nodes
            .iter()
            .filter_map(|node| node.station())
            .collect::<Vec<&Station>>()
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.lines
    }

    pub fn lines_mut(&mut self) -> &mut Vec<Line> {
        &mut self.lines
    }

    pub fn line_exists(&self, line_id: &LineId) -> bool {
        self.lines().iter().any(|line| line.id() == line_id)
    }

    pub fn get_line(&self, line_id: &LineId) -> Option<&Line> {
        if !self.line_exists(line_id) {
            return None;
        }
        self.lines().iter().find(|line| line.id() == line_id)
    }

    pub fn get_line_mut(&mut self, line_id: &LineId) -> Option<&mut Line> {
        if !self.line_exists(line_id) {
            return None;
        }
        self.lines_mut()
            .iter_mut()
            .find(|line| line.id() == line_id)
    }

    /// As routes are contained in lines, more than one route id could exist in different lines, so
    /// we need the line id and the route id to find it.
    pub fn get_route(&self, line_id: &LineId, route_id: String) -> Option<&Route> {
        let Some(line) = self.get_line(line_id) else {
            return None;
        };
        line.routes().iter().find(|route| route.id() == route_id)
    }

    /// As routes are contained in lines, more than one route id could exist in different lines, so
    /// we need the line id and the route id to find it.
    pub fn get_route_mut(&mut self, line_id: &LineId, route_id: String) -> Option<&mut Route> {
        let Some(line) = self.get_line_mut(line_id) else {
            return None;
        };
        line.routes_mut()
            .iter_mut()
            .find(|route| route.id() == route_id)
    }

    pub fn tracks(&self) -> &Vec<Track> {
        &self.tracks
    }

    pub fn track_exists(&self, track_id: &TrackId) -> bool {
        self.tracks().iter().any(|track| track.id() == track_id)
    }

    pub fn get_track(&self, track_id: &TrackId) -> Option<&Track> {
        if !self.track_exists(track_id) {
            return None;
        }
        self.tracks().iter().find(|track| track.id() == track_id)
    }

    pub fn push_station(&mut self, svg_coordinates: SvgIntPoint, station: Station) {
        let node = NetworkNode::new(Node::Station(station), svg_coordinates);
        self.nodes.push(node)
    }

    pub fn push_line(&mut self, line: Line) {
        self.lines.push(line)
    }

    pub fn push_track(&mut self, track: Track) {
        self.tracks.push(track)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SvgIntPoint {
    x: i32,
    y: i32,
}

impl SvgIntPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &i32 {
        &self.x
    }

    pub fn y(&self) -> &i32 {
        &self.y
    }
}

impl From<(i32, i32)> for SvgIntPoint {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
