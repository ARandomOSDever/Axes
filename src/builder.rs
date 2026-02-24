use crate::{Direction, Node, Size, Style};

pub struct LayoutBuilder {
    nodes: Vec<Style>,
    size: (f32, f32), // Width, Height
}

impl LayoutBuilder {
    pub fn new(size: (f32, f32)) -> Self {
        LayoutBuilder {
            nodes: Vec::new(),
            size: size,
        }
    }

    pub fn child(&mut self, style: Style) -> usize {
        self.nodes.push(style);
        self.nodes.len() - 1
    }

    /// Get the info about every Node
    pub fn finish(&mut self) -> Vec<Node> {
        let mut resolved_nodes: Vec<Node> = Vec::with_capacity(self.nodes.len());

        // X of the current node
        let mut x = 0.0;
        // Y of the current node
        let mut y = 0.0;

        let mut gap = Size::default();

        // Parent width of the current node
        let mut parent_width = self.size.0;
        // Parent height of the current node
        let mut parent_height = self.size.1;

        for node in &self.nodes {
            let (gap_width, gap_height) = get_size(&gap, &parent_width, &parent_height);
            x += gap_width;
            y += gap_height;
            let (width, height) = get_size(&node.size, &parent_width, &parent_height);

            // If node has childs (column, row)
            if node.direction != Direction::None {
                let (current_gap_width, current_gap_height) =
                    get_size(&node.gap, &parent_width, &parent_height);

                // Vertical
                if node.direction == Direction::Vertical {
                    gap = Size {
                        height: Some(current_gap_height),
                        ..Default::default()
                    };
                }

                if node.direction == Direction::Horizontal {
                    gap = Size {
                        width: Some(current_gap_width),
                        ..Default::default()
                    };
                }
            }

            resolved_nodes.push(Node {
                x: x,
                y: y,
                parent_height: parent_height,
                parent_width: parent_width,
            });

            parent_width = width;
            parent_height = height;
        }

        resolved_nodes
    }
}

fn get_size(size: &Size, parent_width: &f32, parent_height: &f32) -> (f32, f32) {
    let mut width = 0.0;
    let mut height = 0.0;

    if let Some(node_width) = size.width {
        width = node_width;
    }

    if let Some(node_height) = size.height {
        height = node_height;
    }

    if let Some(percentage) = size.percent_width {
        width = parent_width * percentage;
    }

    if let Some(percentage) = size.percent_height {
        height = parent_height * percentage;
    }
    (width, height)
}
