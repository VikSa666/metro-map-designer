use super::SvgPoint;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Station {
    name: String,
    // TODO: move to node
    svg_coordinates: SvgPoint,
    label_svg_coordinates: Option<SvgPoint>,
}

impl Station {
    pub fn new(
        name: &str,
        svg_coordinates: SvgPoint,
        label_svg_coordinates: Option<SvgPoint>,
    ) -> Self {
        Self {
            name: name.to_string(),
            svg_coordinates,
            label_svg_coordinates,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
