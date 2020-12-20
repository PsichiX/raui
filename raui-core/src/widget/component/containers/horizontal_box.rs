use crate::{
    widget,
    widget::{
        component::containers::flex_box::{flex_box, FlexBoxProps},
        unit::flex::FlexBoxDirection,
    },
    widget_component, Scalar,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HorizontalBoxProps {
    #[serde(default)]
    pub separation: Scalar,
    #[serde(default)]
    pub reversed: bool,
}
implement_props_data!(HorizontalBoxProps, "HorizontalBoxProps");

widget_component! {
    pub horizontal_box(key, props, listed_slots) {
        let HorizontalBoxProps { separation, reversed } = props.read_cloned_or_default();
        let props = FlexBoxProps {
            direction: if reversed {
                FlexBoxDirection::HorizontalRightToLeft
            } else {
                FlexBoxDirection::HorizontalLeftToRight
            },
            separation,
            wrap: false,
        };

        widget! {
            (#{key} flex_box: {props} |[ listed_slots ]|)
        }
    }
}
