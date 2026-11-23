use super::SvgPoint;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Station {
    name: String,
    label_svg_coordinates: Option<SvgPoint>,
}

impl Station {
    pub fn new(name: &str, label_svg_coordinates: Option<SvgPoint>) -> Self {
        Self {
            name: name.to_string(),
            label_svg_coordinates,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
