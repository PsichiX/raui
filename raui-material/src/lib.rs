pub mod component;
pub mod theme;

use raui_core::application::Application;

pub fn setup(app: &mut Application) {
    app.register_props::<component::containers::paper::PaperProps>("PaperProps");
    app.register_props::<component::containers::modal_paper::ModalPaperProps>("ModalPaperProps");
    app.register_props::<component::containers::scroll_paper::SideScrollbarsPaperProps>(
        "SideScrollbarsPaperProps",
    );
    app.register_props::<component::icon_paper::IconPaperProps>("IconPaperProps");
    app.register_props::<component::interactive::text_field_paper::TextFieldPaperProps>(
        "TextFieldPaperProps",
    );
    app.register_props::<component::switch_paper::SwitchPaperProps>("SwitchPaperProps");
    app.register_props::<component::text_paper::TextPaperProps>("TextPaperProps");
    app.register_props::<theme::ThemedWidgetProps>("ThemedWidgetProps");
    app.register_props::<theme::ThemeProps>("ThemeProps");

    app.register_component(
        "nav_flex_paper",
        component::containers::flex_paper::nav_flex_paper,
    );
    app.register_component("flex_paper", component::containers::flex_paper::flex_paper);
    app.register_component(
        "nav_grid_paper",
        component::containers::grid_paper::nav_grid_paper,
    );
    app.register_component("grid_paper", component::containers::grid_paper::grid_paper);
    app.register_component(
        "nav_horizontal_paper",
        component::containers::horizontal_paper::nav_horizontal_paper,
    );
    app.register_component(
        "horizontal_paper",
        component::containers::horizontal_paper::horizontal_paper,
    );
    app.register_component(
        "modal_paper",
        component::containers::modal_paper::modal_paper,
    );
    app.register_component("paper", component::containers::paper::paper);
    app.register_component(
        "scroll_paper",
        component::containers::scroll_paper::scroll_paper,
    );
    app.register_component(
        "scroll_paper_side_scrollbars",
        component::containers::scroll_paper::scroll_paper_side_scrollbars,
    );
    app.register_component(
        "text_tooltip_paper",
        component::containers::text_tooltip_paper::text_tooltip_paper,
    );
    app.register_component(
        "nav_vertical_paper",
        component::containers::vertical_paper::nav_vertical_paper,
    );
    app.register_component(
        "vertical_paper",
        component::containers::vertical_paper::vertical_paper,
    );
    app.register_component("wrap_paper", component::containers::wrap_paper::wrap_paper);
    app.register_component("icon_paper", component::icon_paper::icon_paper);
    app.register_component(
        "button_paper",
        component::interactive::button_paper::button_paper,
    );
    app.register_component(
        "icon_button_paper",
        component::interactive::icon_button_paper::icon_button_paper,
    );
    app.register_component(
        "switch_button_paper",
        component::interactive::switch_button_paper::switch_button_paper,
    );
    app.register_component(
        "text_button_paper",
        component::interactive::text_button_paper::text_button_paper,
    );
    app.register_component(
        "text_field_paper",
        component::interactive::text_field_paper::text_field_paper,
    );
    app.register_component("switch_paper", component::switch_paper::switch_paper);
    app.register_component("text_paper", component::text_paper::text_paper);
}

pub mod prelude {
    pub use crate::{
        component::{
            containers::{
                flex_paper::*, grid_paper::*, horizontal_paper::*, modal_paper::*, paper::*,
                scroll_paper::*, scroll_paper::*, text_tooltip_paper::*, vertical_paper::*,
                wrap_paper::*,
            },
            icon_paper::*,
            interactive::{
                button_paper::*, icon_button_paper::*, switch_button_paper::*,
                text_button_paper::*, text_field_paper::*,
            },
            switch_paper::*,
            text_paper::*,
        },
        theme::*,
    };
}
