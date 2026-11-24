use super::Route;

#[derive(PartialEq, Debug, Eq, Hash, Clone, serde::Serialize, serde::Deserialize)]
pub struct LineId(String);

impl LineId {
    fn new(id: &str) -> Self {
        let normalized_id = if id.starts_with("line-") {
            id.to_string()
        } else {
            format!("line-{id}")
        };

        Self(normalized_id)
    }
}

impl From<&str> for LineId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for LineId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for LineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TODO! Implement different color formats
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LineColor(String);

impl LineColor {
    pub fn new(color: &str) -> Self {
        Self(color.to_string())
    }
}

impl From<&str> for LineColor {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for LineColor {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for LineColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Line {
    id: LineId,
    name: String,
    color: LineColor,
    routes: Vec<Route>,
}

impl Line {
    pub fn new(id: &str, name: &str, color: &str, routes: Vec<Route>) -> Self {
        Self {
            id: LineId::new(id),
            name: name.to_string(),
            color: LineColor::new(color),
            routes,
        }
    }

    pub fn id(&self) -> &LineId {
        &self.id
    }

    pub fn routes(&self) -> &Vec<Route> {
        &self.routes
    }

    pub fn color(&self) -> &LineColor {
        &self.color
    }

    pub fn routes_mut(&mut self) -> &mut Vec<Route> {
        &mut self.routes
    }
}
