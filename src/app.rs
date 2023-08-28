mod dir_explorer;
mod settings;

use egui::{vec2, FontFamily, FontId, Layout, ScrollArea, TextEdit, TextStyle};
use settings::Settings;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use self::dir_explorer::{demo, DirExplorer};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    /// App settings
    settings: Settings,
    /// Currently active template folder
    template_folder: Option<String>,
    /// A text currently edited in the template area
    template_editor_file_text: Option<String>,
    /// File selected in the template file chooser
    template_editor_selected_file: Option<String>,
    /// Current active templates folder content
    templates_explored: Arc<Mutex<DirExplorer>>,

    #[serde(skip)]
    show_template_chooser: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            template_folder: None,
            template_editor_file_text: None,
            template_editor_selected_file: Some("app.rs".to_owned()),
            show_template_chooser: false,
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

        // let mut style = (*cc.egui_ctx.style()).clone();
        // style.debug.show_resize = true;
        // style.debug.debug_on_hover = true;
        // style.debug.show_expand_width = true;
        // style.debug.show_expand_height = true;
        // cc.egui_ctx.set_style(style);

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
                    if ui.button("Clear storage").clicked() {
                        *self = App::default();
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.template_folder {
                Some(_) => self.ui_main(ui),
                None => self.ui_intro(ui),
            };
            egui::warn_if_debug_build(ui);
        });
    }
}

impl App {
    fn ui_intro(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("Welcome!");
            ui.add_space(30.0);
            ui.allocate_ui(egui::vec2(350.0, 800.0), |ui| {
                ui.label("You can open any directory and start working on it as a template or you can open a directory with an existing template");
            });
            ui.add_space(30.0);
            ui.style_mut().spacing.button_padding = egui::vec2(8.0, 8.0);
            if ui.button("Open directory").clicked() {
                // temporary for testing, open dir explorer in future
                self.template_folder = Some("/tmp".to_string())
            }
        });
    }

    fn ui_main(&mut self, ui: &mut egui::Ui) {
        if self.show_template_chooser {
            demo(ui.ctx())
        }
        ui.columns(2, |columns| {
            self.ui_template_panel(&mut columns[0]);
            self.ui_result_panel(&mut columns[1]);
        });
    }

    fn ui_template_panel(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(
            egui::Layout::right_to_left(egui::Align::Min).with_cross_justify(true),
            |ui| {
                ui.separator();
                ui.vertical(|ui| {
                    ui.heading("Template");
                    ui.horizontal_top(|ui| {
                        ui.allocate_ui(vec2(200.0, ui.available_height()), |ui| {
                            self.ui_template_dir(ui);
                        });
                        //ui.horizontal_centered(|ui| ui.label("Select a template file to edit"))
                        self.ui_template_file_editor(ui);
                    });
                });
            },
        );
    }

    fn ui_template_dir(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.style_mut().spacing.button_padding = egui::vec2(8.0, 8.0);
            if ui.button("Change template").clicked() {
                self.show_template_chooser = true
            }
            ui.style_mut().spacing.button_padding = egui::vec2(2.0, 2.0);
            ScrollArea::vertical()
                .id_source("template")
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let stroke = ui.style().noninteractive().bg_stroke;
                    egui::Frame::none()
                        .fill(egui::Color32::WHITE)
                        .stroke(stroke)
                        .inner_margin(stroke.width + 2.0)
                        .show(ui, |ui| {
                            ui.with_layout(Layout::top_down_justified(egui::Align::Min), |ui| {
                                // TODO @dz use selected_label and remove "selected", derive directly from state
                                let mut selected = self
                                    .template_editor_selected_file
                                    .as_ref()
                                    .map(|x| x == "app.rs")
                                    .unwrap_or_default();
                                if ui.toggle_value(&mut selected, "app.rs").clicked() {
                                    self.template_editor_selected_file = Some("app.rs".to_owned());
                                    self.template_editor_file_text =
                                        Some("Contents of app.rs".to_owned())
                                }
                                selected = self
                                    .template_editor_selected_file
                                    .as_ref()
                                    .map(|x| x == "dir_explorer.rs")
                                    .unwrap_or_default();
                                if ui.toggle_value(&mut selected, "dir_explorer.rs").clicked() {
                                    self.template_editor_selected_file =
                                        Some("dir_explorer.rs".to_owned());
                                    self.template_editor_file_text =
                                        Some("Contents of dir_explorer.rs".to_owned())
                                }
                            });
                        });
                });
        });
    }

    fn ui_template_file_editor(&mut self, ui: &mut egui::Ui) {
        if let Some(text) = self.template_editor_file_text.as_mut() {
            TextEdit::multiline(text)
                .desired_width(f32::INFINITY)
                .code_editor()
                .show(ui);
        }
    }

    fn ui_result_panel(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(
            egui::Layout::right_to_left(egui::Align::Min).with_cross_justify(true),
            |ui| {
                ui.vertical(|ui| {
                    ui.heading("Result");
                    ui.columns(2, |columns| {
                        columns[0].with_layout(
                            egui::Layout::bottom_up(egui::Align::Center),
                            |ui| {
                                ScrollArea::vertical()
                                    .id_source("result")
                                    .auto_shrink([false; 2])
                                    .show(ui, |ui| {
                                        ui.vertical(|ui| {
                                            for i in 0..200 {
                                                ui.label(format!("File {}", i));
                                            }
                                        })
                                    });
                            },
                        );
                        columns[1]
                            .horizontal_centered(|ui| ui.label("Select a result file to view"))
                    });
                });
            },
        );
    }
}
