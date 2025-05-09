pub use helium_api::backend::Helium;
pub use helium_api::command::Command;

use eframe::egui;

mod app;

use crate::app::AppUI;

pub fn main()->Result<(),eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 320.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Meister App",
        options,
        Box::new(|_cc| {
            Ok(Box::<AppUI>::default())
        }),
    )
}
