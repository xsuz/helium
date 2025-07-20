use eframe::egui::{self, IconData, FontData, FontDefinitions, FontFamily};

mod app;
mod ui;

use crate::app::AppUI;

pub fn main() -> Result<(), eframe::Error> {
    // Initialize the logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 320.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Meister App",
        options,
        Box::new(|cc| {
            let mut fonts = FontDefinitions::default();
            fonts.font_data.insert(
                "NotoSansJP-Regular".to_owned(),
                FontData::from_static(include_bytes!("../../assets/fonts/NotoSansJP-Regular.ttf"))
                    .into(),
            );
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "NotoSansJP-Regular".to_owned());

            // Put my font as last fallback for monospace:
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .push("NotoSansJP-Regular".to_owned());

            cc.egui_ctx.set_fonts(fonts);
            
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<AppUI>::default())
        }),
    )
}

pub(crate) fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../../assets/logo/logo.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
