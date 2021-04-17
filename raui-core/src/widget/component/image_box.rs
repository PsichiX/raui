use crate::{
    widget,
    widget::{
        component::WidgetAlpha,
        unit::image::{ImageBoxAspectRatio, ImageBoxMaterial, ImageBoxNode, ImageBoxSizeValue},
        utils::Transform,
    },
    widget_component,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ImageBoxProps {
    #[serde(default)]
    pub width: ImageBoxSizeValue,
    #[serde(default)]
    pub height: ImageBoxSizeValue,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_keep_aspect_ratio: Option<ImageBoxAspectRatio>,
    #[serde(default)]
    pub material: ImageBoxMaterial,
    #[serde(default)]
    pub transform: Transform,
}
implement_props_data!(ImageBoxProps);

widget_component!(
    pub fn image_box(id: Id, props: Props, shared_props: SharedProps) {
        let ImageBoxProps {
            width,
            height,
            content_keep_aspect_ratio,
            mut material,
            transform,
        } = props.read_cloned_or_default();
        let alpha = shared_props.read_cloned_or_default::<WidgetAlpha>().0;
        match &mut material {
            ImageBoxMaterial::Color(image) => {
                image.color.a *= alpha;
            }
            ImageBoxMaterial::Image(image) => {
                image.tint.a *= alpha;
            }
            _ => {}
        }

        widget! {{{
            ImageBoxNode {
                id: id.to_owned(),
                props: props.clone(),
                width,
                height,
                content_keep_aspect_ratio,
                material,
                transform,
            }
        }}}
    }
);
