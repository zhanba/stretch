#[test]
fn flex_shrink_flex_grow_row() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(500.0000),
                height: stretch::style::Dimension::Points(500.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 0.0000,
                    flex_shrink: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(500.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                stretch::style::Node {
                    flex_grow: 0.0000,
                    flex_shrink: 1.0000,
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(500.0000),
                        height: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 500.0000);
    assert_eq!(layout.size.height, 500.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 250.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 250.0000);
    assert_eq!(layout.children[1].size.height, 100.0000);
    assert_eq!(layout.children[1].location.x, 250.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);
}
