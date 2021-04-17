use crate::{
    widget,
    widget::{
        component::interactive::navigation::{
            use_nav_container_active, use_nav_item, use_nav_jump, NavContainerActive,
            NavItemActive, NavJumpActive,
        },
        unit::flex::{FlexBoxDirection, FlexBoxItemLayout, FlexBoxItemNode, FlexBoxNode},
        utils::Transform,
    },
    widget_component, Scalar,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FlexBoxProps {
    #[serde(default)]
    pub direction: FlexBoxDirection,
    #[serde(default)]
    pub separation: Scalar,
    #[serde(default)]
    pub wrap: bool,
    #[serde(default)]
    pub transform: Transform,
}
implement_props_data!(FlexBoxProps);

widget_component!(
    #[pre(use_nav_container_active, use_nav_jump, use_nav_item)]
    pub fn nav_flex_box(key: Key, props: Props, listed_slots: ListedSlots) {
        let props = props
            .clone()
            .without::<NavContainerActive>()
            .without::<NavJumpActive>()
            .without::<NavItemActive>();

        widget! {
            (#{key} flex_box: {props} |[listed_slots]|)
        }
    }
);

widget_component!(
    pub fn flex_box(id: Id, props: Props, listed_slots: ListedSlots) {
        let FlexBoxProps {
            direction,
            separation,
            wrap,
            transform,
        } = props.read_cloned_or_default();
        let items = listed_slots
            .into_iter()
            .filter_map(|slot| {
                if let Some(props) = slot.props() {
                    let layout = props.read_cloned_or_default::<FlexBoxItemLayout>();
                    Some(FlexBoxItemNode { slot, layout })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        widget! {{{
            FlexBoxNode {
                id: id.to_owned(),
                props: props.clone(),
                items,
                direction,
                separation,
                wrap,
                transform,
            }
        }}}
    }
);
