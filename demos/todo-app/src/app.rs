use crate::ui::components::app::{app, AppMessage, AppState};
use raui_core::{prelude::*, widget::setup as setup_core};
use raui_material::setup as setup_material;
use raui_tetra_renderer::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
};
use tetra::{
    graphics::{self, Color},
    input::Key,
    Context, Event, State,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct AssetsManifest {
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub fonts: HashMap<String, (usize, Scalar, String)>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub images: HashMap<String, String>,
}

#[derive(MessageData, Debug, Clone)]
pub enum AppSignal {
    Ready(WidgetId),
    Save(AppState),
}

fn setup(app: &mut Application) {
    app.setup(setup_core);
    app.setup(setup_material);
}

pub struct TodoState {
    ui: TetraSimpleHost,
}

impl TodoState {
    pub fn new(context: &mut Context) -> tetra::Result<Self> {
        let assets = serde_json::from_str::<AssetsManifest>(
            &read_to_string("./resources/assets.json").expect("Could not load assets manifest"),
        )
        .expect("Could not parse assets manifest");
        let fonts = assets
            .fonts
            .iter()
            .map(|(k, (s, f, p))| PreloadedFont {
                id: k.as_str(),
                size: *s,
                scale: *f,
                path: p.as_str(),
            })
            .collect::<Vec<_>>();
        let textures = assets
            .images
            .iter()
            .map(|(k, p)| PreloadedTexture {
                id: k.as_str(),
                path: p.as_str(),
            })
            .collect::<Vec<_>>();
        let tree = widget! { (#{"app"} app) };
        let mut ui = TetraSimpleHost::new(context, tree, &fonts, &textures, setup)?;
        ui.interactions.single_scroll_units = (15.0, 15.0).into();
        Ok(Self { ui })
    }
}

impl State for TodoState {
    fn update(&mut self, context: &mut Context) -> tetra::Result {
        for (_, msg) in self.ui.update(context) {
            if let Some(msg) = msg.as_any().downcast_ref() {
                match msg {
                    AppSignal::Ready(id) => self.load(id),
                    AppSignal::Save(state) => Self::save(state),
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> tetra::Result {
        graphics::clear(context, Color::WHITE);
        self.ui.draw(context, PrintLogger)?;
        Ok(())
    }

    fn event(&mut self, context: &mut Context, event: Event) -> tetra::Result {
        self.ui.event(context, &event);
        if let Event::KeyPressed { key: Key::F2 } = event {
            println!("LAYOUT: {:#?}", self.ui.application.layout_data());
        }
        if let Event::KeyPressed { key: Key::F3 } = event {
            println!("INTERACTIONS: {:#?}", self.ui.interactions);
        }
        if let Event::KeyPressed { key: Key::F4 } = event {
            println!(
                "INSPECT TREE: {:#?}",
                self.ui.application.rendered_tree().inspect()
            );
        }
        Ok(())
    }
}

impl TodoState {
    fn load(&mut self, id: &WidgetId) {
        if let Ok(content) = read_to_string("./state.json") {
            if let Ok(state) = serde_json::from_str(&content) {
                self.ui
                    .application
                    .send_message(id, AppMessage::Load(state));
            }
        }
    }

    fn save(state: &AppState) {
        if let Ok(content) = serde_json::to_string_pretty(state) {
            let _ = write("./state.json", content);
        }
    }
}
