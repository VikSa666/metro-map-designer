mod application;
/// Model for the APP
mod domain;
pub use domain::{MetroMap, SvgIntPoint};
pub mod errors;
mod infrastructure;
pub use infrastructure::{deserialize, serialize};
use wasm_bindgen::prelude::wasm_bindgen;

pub use crate::infrastructure::svg::{SvgMetroMap, SvgOptions};

#[wasm_bindgen]
pub fn draw_map(map: &MetroMap, options: &SvgOptions) -> Result<SvgMetroMap, String> {
    map.to_svg(options).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {}
