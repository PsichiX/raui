use raui_core::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct ColorRectProps {
    #[serde(default)]
    pub color: Color,
}
implement_props_data!(ColorRectProps);

widget_component!(
    pub fn color_rect(key: Key, props: Props) {
        let color = props.read_cloned_or_default::<ColorRectProps>().color;
        let props = props.clone().with(ImageBoxProps {
            material: ImageBoxMaterial::Color(ImageBoxColor {
                color,
                scaling: ImageBoxImageScaling::Frame((10.0, true).into()),
            }),
            ..Default::default()
        });

        widget! {
            (#{key} image_box: {props})
        }
    }
);
