mod application;
/// Model for the APP
mod domain;
pub use domain::{MetroMap, SvgPoint};
pub mod errors;
mod infrastructure;
pub use infrastructure::{deserialize, serialize};

#[cfg(test)]
mod tests {}
