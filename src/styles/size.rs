/// Size of node
#[derive(Default, Clone, Debug)]
pub struct Size {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub percent_width: Option<f32>,
    pub percent_height: Option<f32>,
}

impl Size {
    /// Range from 0.0 to 1.0
    pub fn percent(width: f32, height: f32) -> Self {
        Self {
            percent_height: Some(height),
            percent_width: Some(width),
            ..Default::default()
        }
    }

    pub fn new(width: f32, height: f32) -> Self {
        Self {
            height: Some(height),
            width: Some(width),
            ..Default::default()
        }
    }
}
