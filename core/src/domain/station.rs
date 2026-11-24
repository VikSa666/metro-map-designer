use super::SvgIntPoint;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Station {
    name: String,
    label_svg_coordinates: Option<SvgIntPoint>,
}

impl Station {
    pub fn new(name: &str, label_svg_coordinates: Option<SvgIntPoint>) -> Self {
        Self {
            name: name.to_string(),
            label_svg_coordinates,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
