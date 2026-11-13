mod line;
pub use line::*;
mod route;
pub use route::*;
mod station;
pub use station::*;
mod track;
pub use track::*;

use crate::infrastructure::{MetroMapDto, SvgPointDto};

#[derive(Debug, Clone)]
pub struct MetroMap {
    id: String,
    stations: Vec<Station>,
    lines: Vec<Line>,
    tracks: Vec<Track>,
}

impl MetroMap {
    pub fn new(id: &str, stations: Vec<Station>, lines: Vec<Line>, tracks: Vec<Track>) -> Self {
        Self {
            id: id.to_string(),
            stations,
            lines,
            tracks,
        }
    }

    pub fn new_empty(id: &str) -> Self {
        Self {
            id: id.to_string(),
            stations: vec![],
            lines: vec![],
            tracks: vec![],
        }
    }

    pub fn stations(&self) -> &Vec<Station> {
        &self.stations
    }

    pub fn station_exists(&self, station_id: &StationId) -> bool {
        self.stations()
            .iter()
            .any(|station| station.id() == station_id)
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

    pub fn push_station(&mut self, station: Station) {
        self.stations.push(station)
    }

    pub fn push_line(&mut self, line: Line) {
        self.lines.push(line)
    }

    pub fn push_track(&mut self, track: Track) {
        self.tracks.push(track)
    }
}

impl From<MetroMap> for MetroMapDto {
    fn from(value: MetroMap) -> Self {
        Self {
            id: value.id,
            stations: value
                .stations
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
            lines: value.lines.into_iter().map(Into::into).collect::<Vec<_>>(),
            tracks: value.tracks.into_iter().map(Into::into).collect::<Vec<_>>(),
        }
    }
}

impl Into<MetroMap> for MetroMapDto {
    fn into(self) -> MetroMap {
        MetroMap::new(
            self.id.as_str(),
            self.stations
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
            self.lines.into_iter().map(Into::into).collect::<Vec<_>>(),
            self.tracks.into_iter().map(Into::into).collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct SvgPoint {
    x: f32,
    y: f32,
}

impl SvgPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for SvgPoint {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<SvgPoint> for SvgPointDto {
    fn from(point: SvgPoint) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<SvgPointDto> for SvgPoint {
    fn from(point: SvgPointDto) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}
