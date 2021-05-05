use crate::{
    props::Props,
    widget::{
        node::WidgetNode,
        unit::WidgetUnitData,
        utils::{Color, Transform},
        WidgetId,
    },
    PrefabValue, Scalar,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextBoxHorizontalAlign {
    Left,
    Center,
    Right,
}

impl Default for TextBoxHorizontalAlign {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextBoxVerticalAlign {
    Top,
    Middle,
    Bottom,
}

impl Default for TextBoxVerticalAlign {
    fn default() -> Self {
        Self::Top
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextBoxDirection {
    HorizontalLeftToRight,
    HorizontalRightToLeft,
    VerticalTopToBottom,
    VerticalBottomToTop,
}

impl Default for TextBoxDirection {
    fn default() -> Self {
        Self::HorizontalLeftToRight
    }
}

impl TextBoxDirection {
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
pub struct TextBoxFont {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub size: Scalar,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TextBoxSizeValue {
    Fill,
    Exact(Scalar),
}

impl Default for TextBoxSizeValue {
    fn default() -> Self {
        Self::Fill
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextBox {
    #[serde(default)]
    pub id: WidgetId,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub width: TextBoxSizeValue,
    #[serde(default)]
    pub height: TextBoxSizeValue,
    #[serde(default)]
    pub horizontal_align: TextBoxHorizontalAlign,
    #[serde(default)]
    pub vertical_align: TextBoxVerticalAlign,
    #[serde(default)]
    pub direction: TextBoxDirection,
    #[serde(default)]
    pub font: TextBoxFont,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub transform: Transform,
}

impl WidgetUnitData for TextBox {
    fn id(&self) -> &WidgetId {
        &self.id
    }
}

impl TryFrom<TextBoxNode> for TextBox {
    type Error = ();

    fn try_from(node: TextBoxNode) -> Result<Self, Self::Error> {
        let TextBoxNode {
            id,
            text,
            width,
            height,
            horizontal_align,
            vertical_align,
            direction,
            font,
            color,
            transform,
            ..
        } = node;
        Ok(Self {
            id,
            text,
            width,
            height,
            horizontal_align,
            vertical_align,
            direction,
            font,
            color,
            transform,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct TextBoxNode {
    pub id: WidgetId,
    pub props: Props,
    pub text: String,
    pub width: TextBoxSizeValue,
    pub height: TextBoxSizeValue,
    pub horizontal_align: TextBoxHorizontalAlign,
    pub vertical_align: TextBoxVerticalAlign,
    pub direction: TextBoxDirection,
    pub font: TextBoxFont,
    pub color: Color,
    pub transform: Transform,
}

impl TextBoxNode {
    pub fn remap_props<F>(&mut self, mut f: F)
    where
        F: FnMut(Props) -> Props,
    {
        let props = std::mem::take(&mut self.props);
        self.props = (f)(props);
    }
}

impl From<TextBoxNode> for WidgetNode {
    fn from(data: TextBoxNode) -> Self {
        Self::Unit(data.into())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct TextBoxNodePrefab {
    #[serde(default)]
    pub id: WidgetId,
    #[serde(default)]
    pub props: PrefabValue,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub width: TextBoxSizeValue,
    #[serde(default)]
    pub height: TextBoxSizeValue,
    #[serde(default)]
    pub horizontal_align: TextBoxHorizontalAlign,
    #[serde(default)]
    pub vertical_align: TextBoxVerticalAlign,
    #[serde(default)]
    pub direction: TextBoxDirection,
    #[serde(default)]
    pub font: TextBoxFont,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub transform: Transform,
}
