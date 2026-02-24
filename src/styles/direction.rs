/// The direction that childs will be display
#[derive(PartialEq, Clone)]
pub enum Direction {
    Vertical,
    Horizontal,
    /// For child that will be not contains childs
    None,
}

impl Default for Direction {
    fn default() -> Self {
        Self::None
    }
}
