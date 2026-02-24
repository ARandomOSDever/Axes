mod builder;
pub use builder::*;
mod node;
pub use node::*;
mod styles;
pub use styles::*;
mod style;
pub use style::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut builder = LayoutBuilder::new((900.0, 900.0));
        builder.child(Style {
            size: Size::default(),
            gap: Size {
                width: Some(30.0),
                height: None,
                percent_width: None,
                percent_height: None,
            },
            direction: Direction::Horizontal,
        });
        builder.child(Style {
            size: Size {
                width: Some(300.0),
                height: Some(300.0),
                percent_width: None,
                percent_height: None,
            },
            gap: Size::default(),
            direction: Direction::default(),
        });
        builder.child(Style {
            size: Size {
                width: Some(300.0),
                height: Some(300.0),
                percent_width: None,
                percent_height: None,
            },
            gap: Size::default(),
            direction: Direction::default(),
        });
        let resolved_layout = builder.finish();

        assert_eq!(resolved_layout[0].parent_height, 900.0);
        assert_eq!(resolved_layout[0].parent_width, 900.0);
        assert_eq!(resolved_layout[0].x, 0.0);
        assert_eq!(resolved_layout[0].y, 0.0);

        assert_eq!(resolved_layout[1].parent_height, 0.0);
        assert_eq!(resolved_layout[1].parent_width, 0.0);
        assert_eq!(resolved_layout[1].x, 0.0);
        assert_eq!(resolved_layout[1].y, 0.0);

        assert_eq!(resolved_layout[2].parent_height, 300.0);
        assert_eq!(resolved_layout[2].parent_width, 300.0);
        assert_eq!(resolved_layout[2].x, 30.0);
        assert_eq!(resolved_layout[2].y, 0.0);
    }
}
