use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::domain::NetworkNode;
use crate::{
    MetroMap,
    domain::{LineId, Track, TrackId},
    errors::CustomError,
};

pub const DEFAULT_WIDTH: f32 = 2400.;
pub const DEFAULT_PADDING: f32 = 20.;
const TRACK_STROKE_WIDTH: f32 = 6.;
const TRACK_LANE_SPACING: f32 = TRACK_STROKE_WIDTH * 1.1;

#[wasm_bindgen]
pub struct SvgOptions {
    pub width: Option<f32>,
    pub padding: Option<f32>,
    // TODO: More needed in the future
}

#[wasm_bindgen]
#[allow(unused)]
pub struct SvgMetroMap {
    svg: String,
    width: f32,
    height: f32,
}

#[wasm_bindgen]
impl SvgMetroMap {
    pub fn svg(&self) -> String {
        self.svg.to_string()
    }
}

struct SvgViewport {
    width: f32,
    height: f32,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    min_x: f32,
    min_y: f32,
}

impl SvgViewport {
    fn transform(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.offset_x + (x - self.min_x) * self.scale,
            self.offset_y + (y - self.min_y) * self.scale,
        )
    }
}

/// Replace special characters by HTML entities.
fn escape_xml(value: &str) -> String {
    value
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("\'", "&apos;")
}

impl MetroMap {
    pub fn to_svg(&self, options: &SvgOptions) -> Result<SvgMetroMap, CustomError> {
        let width = options.width.unwrap_or(DEFAULT_WIDTH);
        let padding = options.padding.unwrap_or(DEFAULT_PADDING);
        // Early return if no nodes in map
        if self.nodes().is_empty() {
            return Ok(SvgMetroMap {
                svg: format!(
                    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} 0" />"#
                ),
                width,
                height: 0.,
            });
        }

        let xs = self.nodes().iter().map(|node| node.x() as f32).chain(
            self.tracks()
                .iter()
                .flat_map(|track| track.geometry().iter().map(|point| *point.x() as f32)),
        );
        let ys = self.nodes().iter().map(|node| node.y() as f32).chain(
            self.tracks()
                .iter()
                .flat_map(|track| track.geometry().iter().map(|point| *point.y() as f32)),
        );

        let Some(min_x) = xs.clone().min_by(f32::total_cmp) else {
            return Err(CustomError::SvgExportError(
                "could not calculate the minimum of x coordinates".to_string(),
            ));
        };
        let Some(max_x) = xs.max_by(f32::total_cmp) else {
            return Err(CustomError::SvgExportError(
                "could not calculate the maximum of x coordinates".to_string(),
            ));
        };
        let Some(min_y) = ys.clone().min_by(f32::total_cmp) else {
            return Err(CustomError::SvgExportError(
                "could not calculate the minimum of y coordinates".to_string(),
            ));
        };
        let Some(max_y) = ys.max_by(f32::total_cmp) else {
            return Err(CustomError::SvgExportError(
                "could not calculate the maximum of y coordinates".to_string(),
            ));
        };

        if max_x - min_x <= 0. {
            return Err(CustomError::SvgExportError(format!(
                "Coordinates X have max {max_x} and min {min_x}"
            )));
        }

        if max_y - min_y <= 0. {
            return Err(CustomError::SvgExportError(format!(
                "Coordinates Y have max {max_y} and min {min_y}"
            )));
        }

        let (range_x, range_y) = (max_x - min_x, max_y - min_y);
        let height = width * (range_y / range_x);

        let scale = f32::min(
            (width - 2. * padding) / range_x,
            (height - 2. * padding) / range_y,
        );

        let offset_x = (width - range_x * scale) / 2.;
        let offset_y = (height - range_y * scale) / 2.;

        let viewport = SvgViewport {
            width,
            height,
            scale,
            offset_x,
            offset_y,
            min_x,
            min_y,
        };

        let mut svg = String::new();

        writeln!(
            svg,
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}">"#,
            viewport.width, viewport.height
        )?;

        self.draw_tracks(&mut svg, &viewport)?;
        for node in self.nodes() {
            self.draw_node(&mut svg, &viewport, node)?;
        }

        writeln!(svg, "</svg>")?;
        Ok(SvgMetroMap { svg, width, height })
    }

    /// Builds an index to know which line uses each track
    ///
    /// ```txt
    /// TrackId -> [LineId, LineId, ...]
    /// ```
    fn build_track_line_index(&self) -> HashMap<TrackId, HashSet<&LineId>> {
        let mut index: HashMap<TrackId, HashSet<&LineId>> = HashMap::new();

        for line in self.lines() {
            for route in line.routes() {
                for track_id in route.tracks() {
                    index
                        .entry(track_id.clone())
                        .or_insert_with(HashSet::new)
                        .insert(line.id());
                }
            }
        }
        index
    }

    fn draw_track(
        &self,
        svg: &mut String,
        track: &Track,
        viewport: &SvgViewport,
        line_ids: &HashSet<&LineId>,
    ) -> Result<(), CustomError> {
        let mut points = Vec::new();
        let node_from = self.node_by_id(track.origin())?;
        let node_to = self.node_by_id(track.destination())?;

        points.push(viewport.transform(node_from.x() as f32, node_from.y() as f32));

        for point in track.geometry() {
            points.push(viewport.transform(*point.x() as f32, *point.y() as f32));
        }

        points.push(viewport.transform(node_to.x() as f32, node_to.y() as f32));

        let mut line_ids = line_ids.iter().collect::<Vec<_>>();
        line_ids.sort_by_key(|line_id| line_id.to_string());

        for (index, line_id) in line_ids.iter().enumerate() {
            let line = self
                .get_line(line_id)
                .ok_or_else(|| CustomError::LineIdNotFound {
                    id: line_id.to_string(),
                })?;
            let offset = (index as f32 - (line_ids.len() - 1) as f32 / 2.) * TRACK_LANE_SPACING;
            let lane_points = offset_polyline(&points, offset);
            let mut path = String::new();

            for (point_index, (x, y)) in lane_points.into_iter().enumerate() {
                if point_index == 0 {
                    write!(path, "M {x} {y}")?;
                } else {
                    write!(path, " L {x} {y}")?;
                }
            }

            writeln!(
                svg,
                r#"<path d="{path}" fill="none" stroke="{}" stroke-width="{TRACK_STROKE_WIDTH}" stroke-linecap="round" stroke-linejoin="round" />"#,
                escape_xml(&line.color().to_string())
            )?;
        }

        Ok(())
    }

    fn draw_tracks(&self, svg: &mut String, viewport: &SvgViewport) -> Result<(), CustomError> {
        let track_line_index = self.build_track_line_index();
        for track in self.tracks() {
            let lines = track_line_index.get(track.id());
            if let Some(lines) = lines {
                self.draw_track(svg, &track, viewport, lines)?;
            }
        }
        Ok(())
    }

    fn draw_node(
        &self,
        svg: &mut String,
        viewport: &SvgViewport,
        node: &NetworkNode,
    ) -> Result<(), CustomError> {
        if matches!(node.node(), crate::domain::Node::Edge) {
            return Ok(());
        }

        let track_line_index = self.build_track_line_index();
        let mut line_ids = HashSet::new();
        for track in self.tracks() {
            if track.origin() == node.id() || track.destination() == node.id() {
                if let Some(lines) = track_line_index.get(track.id()) {
                    line_ids.extend(lines.iter().copied());
                }
            }
        }

        if line_ids.is_empty() {
            return Ok(());
        }

        let (x, y) = viewport.transform(node.x() as f32, node.y() as f32);
        if line_ids.len() == 1 {
            let line = self
                .get_line(line_ids.iter().next().expect("one line id"))
                .ok_or_else(|| CustomError::LineIdNotFound {
                    id: line_ids.iter().next().expect("one line id").to_string(),
                })?;
            writeln!(
                svg,
                r#"<circle cx="{x}" cy="{y}" r="{}" fill="{}" />"#,
                TRACK_STROKE_WIDTH * 2.,
                escape_xml(&line.color().to_string())
            )?;
        } else {
            writeln!(
                svg,
                r#"<circle cx="{x}" cy="{y}" r="{}" fill="white" stroke="black" stroke-width="{TRACK_STROKE_WIDTH}" />"#,
                TRACK_STROKE_WIDTH * 2.
            )?;
        }

        Ok(())
    }
}

/// Returns a polyline shifted by a signed distance along its segment normals.
///
/// Positive offsets move points to the left of the direction of travel. At
/// vertices, neighboring normals are combined with a bounded miter to keep the
/// offset continuous. If the offset is zero or fewer than two points are given,
/// the input points are returned unchanged; degenerate vertices without a usable
/// normal are also left in place.
fn offset_polyline(points: &[(f32, f32)], offset: f32) -> Vec<(f32, f32)> {
    if offset == 0. || points.len() < 2 {
        return points.to_vec();
    }

    let normals = points
        .windows(2)
        .map(|pair| {
            let (dx, dy) = (pair[1].0 - pair[0].0, pair[1].1 - pair[0].1);
            let length = dx.hypot(dy);
            if length == 0. {
                None
            } else {
                Some((-dy / length, dx / length))
            }
        })
        .collect::<Vec<_>>();

    points
        .iter()
        .enumerate()
        .map(|(index, &(x, y))| {
            let previous = normals[..index].iter().rev().find_map(|normal| *normal);
            let next = normals[index..].iter().find_map(|normal| *normal);

            let Some(normal) = miter_normal(previous, next) else {
                return (x, y);
            };
            (x + normal.0 * offset, y + normal.1 * offset)
        })
        .collect()
}

/// Computes the offset direction at a polyline vertex from its neighboring normals.
///
/// At a corner, the normalized sum of the normals is scaled to keep the offset
/// aligned with both segments. The scale is limited to avoid excessively long
/// miters; at endpoints or degenerate corners, a single available normal is used.
/// Returns `None` when neither neighboring segment has a usable normal.
fn miter_normal(previous: Option<(f32, f32)>, next: Option<(f32, f32)>) -> Option<(f32, f32)> {
    match (previous, next) {
        (Some(previous), Some(next)) => {
            let (x, y) = (previous.0 + next.0, previous.1 + next.1);
            let length = x.hypot(y);
            if length == 0. {
                Some(next)
            } else {
                let miter = (x / length, y / length);
                let projection = miter.0 * next.0 + miter.1 * next.1;
                if projection.abs() < f32::EPSILON {
                    Some(next)
                } else {
                    let scale = (1. / projection).clamp(-4., 4.);
                    Some((miter.0 * scale, miter.1 * scale))
                }
            }
        }
        (Some(normal), None) | (None, Some(normal)) => Some(normal),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::domain::{Line, NetworkNode, Node, Route, Track};

    use super::*;

    #[test]
    fn shared_track_is_drawn_as_separate_colored_parallel_paths() {
        let origin = NetworkNode::new(Node::Edge, (0, 0).into());
        let destination = NetworkNode::new(Node::Edge, (100, 100).into());
        let track_id = TrackId::from("track-1");
        let route = || {
            Route::new(
                "route-1".to_string(),
                None,
                BTreeSet::from([track_id.clone()]),
            )
        };
        let metro_map = MetroMap::new(
            "map",
            vec![origin.clone(), destination.clone()],
            vec![
                Line::new("line-a", "A", "#ff0000", vec![route()]),
                Line::new("line-b", "B", "#0000ff", vec![route()]),
            ],
            vec![Track::new(
                "track-1",
                origin.id().clone(),
                destination.id().clone(),
                vec![],
            )],
        );

        let svg = metro_map
            .to_svg(&SvgOptions {
                width: Some(200.),
                padding: Some(20.),
            })
            .unwrap_or_else(|error| panic!("{error}"))
            .svg;

        assert!(svg.starts_with("<svg "));
        assert!(svg.contains("stroke=\"#ff0000\""));
        assert!(svg.contains("stroke=\"#0000ff\""));
        assert_eq!(svg.matches("<path ").count(), 2);

        let path_data = svg
            .lines()
            .filter_map(|line| line.split_once(r#" d=""#).map(|(_, rest)| rest))
            .map(|rest| rest.split('"').next().unwrap())
            .collect::<Vec<_>>();
        assert_ne!(path_data[0], path_data[1]);
    }

    #[test]
    fn geometry_outside_station_bounds_is_kept_inside_the_viewport() {
        let origin = NetworkNode::new(Node::Edge, (0, 0).into());
        let destination = NetworkNode::new(Node::Edge, (100, 100).into());
        let track_id = TrackId::from("track-1");
        let route = Route::new("route-1".to_string(), None, BTreeSet::from([track_id]));
        let metro_map = MetroMap::new(
            "map",
            vec![origin.clone(), destination.clone()],
            vec![Line::new("line-a", "A", "#ff0000", vec![route])],
            vec![Track::new(
                "track-1",
                origin.id().clone(),
                destination.id().clone(),
                vec![(50, -50).into()],
            )],
        );

        let svg = metro_map
            .to_svg(&SvgOptions {
                width: Some(200.),
                padding: Some(20.),
            })
            .unwrap_or_else(|error| panic!("{error}"))
            .svg;

        assert!(svg.contains("L 100 30"));
    }

    #[test]
    fn shared_axis_aligned_tracks_have_visible_lane_spacing() {
        let first = NetworkNode::new(Node::Edge, (0, 0).into());
        let corner = NetworkNode::new(Node::Edge, (100, 0).into());
        let last = NetworkNode::new(Node::Edge, (100, 100).into());
        let track_ids = BTreeSet::from([TrackId::from("track-1"), TrackId::from("track-2")]);
        let route = || Route::new("route-1".to_string(), None, track_ids.clone());
        let metro_map = MetroMap::new(
            "map",
            vec![first.clone(), corner.clone(), last.clone()],
            vec![
                Line::new("line-a", "A", "#ff0000", vec![route()]),
                Line::new("line-b", "B", "#0000ff", vec![route()]),
            ],
            vec![
                Track::new("track-1", first.id().clone(), corner.id().clone(), vec![]),
                Track::new("track-2", corner.id().clone(), last.id().clone(), vec![]),
            ],
        );

        let svg = metro_map
            .to_svg(&SvgOptions {
                width: Some(200.),
                padding: Some(20.),
            })
            .unwrap_or_else(|error| panic!("{error}"))
            .svg;
        let path_data = svg
            .lines()
            .filter_map(|line| line.split_once(r#" d=""#).map(|(_, rest)| rest))
            .map(|rest| rest.split('"').next().unwrap())
            .collect::<Vec<_>>();
        let coordinates = |path: &str| {
            path.split_whitespace()
                .filter_map(|value| value.parse::<f32>().ok())
                .collect::<Vec<_>>()
        };
        let horizontal_a = coordinates(path_data[0]);
        let horizontal_b = coordinates(path_data[1]);
        let vertical_a = coordinates(path_data[2]);
        let vertical_b = coordinates(path_data[3]);

        assert!((horizontal_a[1] - horizontal_b[1]).abs() > TRACK_STROKE_WIDTH);
        assert!((vertical_a[2] - vertical_b[2]).abs() > TRACK_STROKE_WIDTH);
    }
}
