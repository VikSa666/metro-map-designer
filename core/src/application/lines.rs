use crate::{
    domain::{Line, MetroMap, Route},
    errors::CustomError,
};

impl MetroMap {
    /// # Line creator
    ///
    /// Creates a new line and adds it to the whole map.
    ///
    /// If the line's id is already present, it will not add it.
    ///
    /// TODO: Return an error when the line is already created
    pub fn create_line(
        self: &mut MetroMap,
        id: &str,
        name: &str,
        color: &str,
        routes: Vec<Route>,
    ) -> Result<(), CustomError> {
        let line = Line::new(id, name, color, routes);
        if !self.line_exists(line.id()) {
            self.push_line(line);
        }
        return Ok(());
    }
}
