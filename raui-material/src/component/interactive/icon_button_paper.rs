use crate::component::{icon_paper::icon_paper, interactive::button_paper::button_paper};

widget_component!(
    pub fn icon_button_paper(key: Key, props: Props) {
        widget! {
            (#{key} button_paper: {props.clone()} {
                content = (#{"icon"} icon_paper: {props.clone()})
            })
        }
    }
);
