use raui::prelude::*;
use raui_quick_start::RauiQuickStartBuilder;

fn main() {
    let tree = make_widget!(content_box)
        .listed_slot(
            make_widget!(image_box).with_props(ImageBoxProps::colored(Color {
                r: 1.0,
                g: 0.25,
                b: 0.25,
                a: 1.0,
            })),
        )
        .listed_slot(
            make_widget!(image_box)
                .with_props(ImageBoxProps::colored(Color {
                    r: 0.25,
                    g: 1.0,
                    b: 0.25,
                    a: 1.0,
                }))
                .with_props(ContentBoxItemLayout {
                    margin: 64.0.into(),
                    ..Default::default()
                }),
        )
        .listed_slot(
            make_widget!(image_box)
                .with_props(ImageBoxProps::colored(Color {
                    r: 0.25,
                    g: 0.25,
                    b: 1.0,
                    a: 1.0,
                }))
                .with_props(ContentBoxItemLayout {
                    anchors: Rect {
                        left: 0.5,
                        right: 0.75,
                        top: 0.25,
                        bottom: 0.75,
                    },
                    ..Default::default()
                }),
        );

    RauiQuickStartBuilder::default()
        .window_title("Content Box".to_owned())
        .widget_tree(tree.into())
        .build()
        .unwrap()
        .run()
        .unwrap();
}
