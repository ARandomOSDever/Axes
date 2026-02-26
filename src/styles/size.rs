#[derive(Clone, Copy, Debug)]
pub enum Length {
    Fixed(f32),
    /// Range from 0.0 to 1.0
    Percent(f32),
}

impl Default for Length {
    fn default() -> Self {
        Self::Fixed(0.0)
    }
}

/// Size of node
#[derive(Clone, Copy, Debug, Default)]
pub struct Size {
    pub width: Length,
    pub height: Length,
}
