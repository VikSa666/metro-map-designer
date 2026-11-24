use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    domain::{LineId, NetworkNodeId, Track, TrackId},
    errors::CustomError,
    MetroMap,
};

pub const DEFAULT_WIDTH: f32 = 2400.;
pub const DEFAULT_PADDING: f32 = 20.;

pub struct SvgOptions {
    width: Option<f32>,
    padding: Option<f32>,
    // TODO: More needed in the future
}

#[wasm_bindgen]
pub struct SvgMetroMap {
    svg: String,
    width: f32,
    height: f32,
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
fn escapeXml(value: &str) -> String {
    value
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("\'", "&apos;")
}

impl MetroMap {
    pub fn to_svg(&self, options: SvgOptions) -> Result<SvgMetroMap, CustomError> {
        let width = options.width.unwrap_or(DEFAULT_WIDTH);
        let padding = options.padding.unwrap_or(DEFAULT_PADDING);
        // Early return if no nodes in map
        if self.nodes().is_empty() {
            return Ok(SvgMetroMap {
                svg: format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width}" />"#),
                width,
                height: 0.,
            });
        }

        let xs = self.nodes().iter().map(|node| node.x() as f32);
        let ys = self.nodes().iter().map(|node| node.y() as f32);

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
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}""#,
            viewport.width, viewport.height
        );

        self.draw_tracks(&mut svg, &viewport)?;
        // TODO: self.draw_lines(&mut svg, &viewport)?;
        // TODO: self.draw_nodes(&mut svg, &viewport)?;

        writeln!(svg, "</svg>");
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

        for (i, (x, y)) in points.into_iter().enumerate() {
            if i == 0 {
                write!(svg, "M {x} {y}")?;
            } else {
                write!(svg, " L {x} {y}")?;
            }
        }

        Ok(())

        // let path = points_to_svg_path(&points);
        // write!(
        //     svg,
        //     r#"<path d="{path}" fill="none" stroke="{color}" stroke-width="6" stroke-linecap="round" stroke-linejoin="round" />"#
        // ).unwrap();
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
}
