pub enum CustomError {
    StationIdNotFound { id: String },
    NodeIdNotFound { id: String },
    TrackIdNotFound { id: String },
    LineIdNotFound { id: String },
    RouteIdNotFound { line_id: String, route_id: String },
    RouteCreationFailed { custom: String },
    TrackConnectionFailed,
    Custom(String),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::StationIdNotFound { id } => {
                write!(f, "Station with id `{id}` not found in current map.")
            }
            CustomError::NodeIdNotFound { id } => {
                write!(f, "Node with id `{id}` not found in current map.")
            }
            CustomError::TrackIdNotFound { id } => {
                write!(f, "Track with id `{id}` not found in current map.")
            }
            CustomError::LineIdNotFound { id } => {
                write!(f, "Line with id `{id}` not found in current map.")
            }
            CustomError::RouteIdNotFound { line_id, route_id } => {
                write!(
                    f,
                    "Route with id `{route_id}` not found in line `{line_id}`"
                )
            }
            CustomError::RouteCreationFailed { custom } => {
                write!(
                    f,
                    "Route creation failed due to following error: `{custom}`"
                )
            }
            // TODO: Improve this error
            CustomError::TrackConnectionFailed => write!(f, "Cannot connect tracks"),
            CustomError::Custom(s) => write!(f, "{s}"),
        }
    }
}

impl From<CustomError> for std::io::Error {
    fn from(value: CustomError) -> Self {
        Self::new(std::io::ErrorKind::Other, value.to_string())
    }
}
