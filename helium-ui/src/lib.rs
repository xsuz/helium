use helium_api::command::Command;
use helium_core::DataBase;
use std::sync::{Arc, Mutex, mpsc};

mod core_ui;
mod sensor_ui;

pub use core_ui::select_port::SelectPortUI;
pub use core_ui::control_panel::ControlPanelUI;

pub trait UI{
    fn update(&mut self,ctx:&egui::Context,ui:Option<&mut egui::Ui>,database:&Arc<Mutex<DataBase>>,tx_command:&mpsc::Sender<Command>);
}