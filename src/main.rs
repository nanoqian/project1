#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
//#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "project1",
        native_options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();

            fonts.font_data.insert("UKaiTW".to_owned(),
                std::sync::Arc::new(
                    egui::FontData::from_static(include_bytes!("../assets/UKaiTW.otf"))
                )
            );

            // Add font as last fallback:
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
                .push("UKaiTW".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(project1::Project1::new(cc)))
        }),
    )
}