mod settings;

use egui::{FontFamily, FontId, TextStyle};
use settings::Settings;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    /// App settings
    settings: Settings,
    /// Currently active template folder
    template_folder: Option<String>,

    // this how you opt-out of serialization of a member
    // TODO @dz remove after @TrueWarg sees this
    #[serde(skip)]
    value: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            template_folder: None,
            value: 2.7,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let app: App;

        setup_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            app = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        } else {
            app = Default::default();
        }

        cc.egui_ctx.set_visuals(if app.settings.use_light_mode {
            egui::Visuals::light()
        } else {
            egui::Visuals::dark()
        });

        app
    }
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "liberation".to_owned(),
        egui::FontData::from_static(include_bytes!("../fonts/LiberationSans-Regular.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "liberation".to_owned());
    ctx.set_fonts(fonts);
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(16.0, Monospace)),
        (TextStyle::Button, FontId::new(16.0, Proportional)),
        (TextStyle::Small, FontId::new(12.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // TODO @dz @Settings remove this from menu and add Settings window instead
                    if ui.button("Switch color theme").clicked() {
                        self.settings.use_light_mode = !self.settings.use_light_mode;
                        ctx.set_visuals(if self.settings.use_light_mode {
                            egui::Visuals::light()
                        } else {
                            egui::Visuals::dark()
                        });
                        ui.close_menu()
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
