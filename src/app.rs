use std::fs;
use rfd::FileDialog; // Correct file dialog import

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Project1 {
    pub text_buffer: String,
    pub file_path : Option<std::path::PathBuf>,
    pub is_window_pinned : bool
}

impl Default for Project1 {
    fn default() -> Self {
        Self {
            text_buffer: String::new(),
            file_path: None,
            is_window_pinned : false
        }
    }
}

impl Project1 {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert("UKaiTW".to_owned(),
            std::sync::Arc::new(
                egui::FontData::from_static(include_bytes!("../assets/UKaiTW.otf"))
            )
        );

        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .push("UKaiTW".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Project1 {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open").clicked() {
                            // Open file dialog (if on a desktop platform)
                            if let Some(path) = FileDialog::new().pick_file() {
                                // Read the contents of the file
                                if let Ok(contents) = fs::read_to_string(&path) {
                                    self.text_buffer = contents; // Load into text buffer
                                    self.file_path = Some(path);
                                } else {
                                    // Handle file read error
                                    eprintln!("Failed to read file");
                                }
                            }
                        }
                        if ui.button("Save").clicked() {
                            if let None = self.file_path {
                                self.file_path = FileDialog::new().save_file();
                            }
                            if let Some(path) = &self.file_path {
                                if let Err(_) = fs::write(&path, &self.text_buffer) {
                                    eprintln!("Failed to write to file");
                                }
                            }
                        }
                        if ui.button("Save As").clicked() {
                            if let Some(path) = FileDialog::new().save_file() {
                                if let Ok(()) = fs::write(&path, &self.text_buffer) {
                                    self.file_path = Some(path);
                                } else {
                                    eprintln!("Failed to write to file");
                                }
                            }
                        }
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);

                // Pin checkbox
                ui.checkbox(&mut self.is_window_pinned, "Pin");

                if self.is_window_pinned == true {
                    ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
                }
                else{
                    ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
                }
                
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Add a scroll area to enable scrolling if the text overflows
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_sized(ui.available_size(), 
                    egui::TextEdit::multiline(&mut self.text_buffer)
                        .font(egui::TextStyle::Body.resolve(&ui.style()).clone())
                );
            });
        });
    }
}
