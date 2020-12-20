use crate::{
    props::{Props, PropsDef},
    widget::{
        node::{WidgetNode, WidgetNodeDef},
        unit::{WidgetUnit, WidgetUnitData},
        utils::Rect,
        WidgetId,
    },
    Scalar,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FlexBoxItemLayout {
    #[serde(default)]
    pub basis: Option<Scalar>,
    #[serde(default)]
    pub fill: Scalar,
    #[serde(default)]
    pub grow: Scalar,
    #[serde(default)]
    pub shrink: Scalar,
    #[serde(default)]
    pub margin: Rect,
    #[serde(default)]
    pub align: Scalar,
}
implement_props_data!(FlexBoxItemLayout, "FlexBoxItemLayout");

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FlexBoxItem {
    #[serde(default)]
    pub slot: WidgetUnit,
    #[serde(default)]
    pub layout: FlexBoxItemLayout,
}

impl TryFrom<FlexBoxItemNode> for FlexBoxItem {
    type Error = ();

    fn try_from(node: FlexBoxItemNode) -> Result<Self, Self::Error> {
        let FlexBoxItemNode { slot, layout } = node;
        Ok(Self {
            slot: WidgetUnit::try_from(slot)?,
            layout,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct FlexBoxItemNode {
    pub slot: WidgetNode,
    pub layout: FlexBoxItemLayout,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FlexBoxItemNodeDef {
    #[serde(default)]
    pub slot: WidgetNodeDef,
    #[serde(default)]
    pub layout: FlexBoxItemLayout,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexBoxDirection {
    HorizontalLeftToRight,
    HorizontalRightToLeft,
    VerticalTopToBottom,
    VerticalBottomToTop,
}

impl Default for FlexBoxDirection {
    fn default() -> Self {
        Self::HorizontalLeftToRight
    }
}

impl FlexBoxDirection {
    pub fn is_horizontal(&self) -> bool {
        *self == Self::HorizontalLeftToRight || *self == Self::HorizontalRightToLeft
    }

    pub fn is_vertical(&self) -> bool {
        *self == Self::VerticalTopToBottom || *self == Self::VerticalBottomToTop
    }

    pub fn is_order_ascending(&self) -> bool {
        *self == Self::HorizontalLeftToRight || *self == Self::VerticalTopToBottom
    }

    pub fn is_order_descending(&self) -> bool {
        *self == Self::HorizontalRightToLeft || *self == Self::VerticalBottomToTop
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FlexBox {
    #[serde(default)]
    pub id: WidgetId,
    #[serde(default)]
    pub items: Vec<FlexBoxItem>,
    #[serde(default)]
    pub direction: FlexBoxDirection,
    #[serde(default)]
    pub separation: Scalar,
    #[serde(default)]
    pub wrap: bool,
}

impl WidgetUnitData for FlexBox {
    fn id(&self) -> &WidgetId {
        &self.id
    }

    fn get_children<'a>(&'a self) -> Vec<&'a WidgetUnit> {
        self.items.iter().map(|item| &item.slot).collect()
    }
}

impl TryFrom<FlexBoxNode> for FlexBox {
    type Error = ();

    fn try_from(node: FlexBoxNode) -> Result<Self, Self::Error> {
        let FlexBoxNode {
            id,
            items,
            direction,
            separation,
            wrap,
            ..
        } = node;
        let items = items
            .into_iter()
            .map(|item| FlexBoxItem::try_from(item))
            .collect::<Result<_, _>>()?;
        Ok(Self {
            id,
            items,
            direction,
            separation,
            wrap,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct FlexBoxNode {
    pub id: WidgetId,
    pub props: Props,
    pub items: Vec<FlexBoxItemNode>,
    pub direction: FlexBoxDirection,
    pub separation: Scalar,
    pub wrap: bool,
}

impl FlexBoxNode {
    pub fn remap_props<F>(&mut self, mut f: F)
    where
        F: FnMut(Props) -> Props,
    {
        let props = std::mem::replace(&mut self.props, Default::default());
        self.props = (f)(props);
    }
}

impl Into<WidgetNode> for FlexBoxNode {
    fn into(self) -> WidgetNode {
        WidgetNode::Unit(self.into())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FlexBoxNodeDef {
    #[serde(default)]
    pub id: WidgetId,
    #[serde(default)]
    pub props: PropsDef,
    #[serde(default)]
    pub items: Vec<FlexBoxItemNodeDef>,
    #[serde(default)]
    pub direction: FlexBoxDirection,
    #[serde(default)]
    pub separation: Scalar,
    #[serde(default)]
    pub wrap: bool,
}
