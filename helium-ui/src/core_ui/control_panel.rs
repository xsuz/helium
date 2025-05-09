use crate::UI;
use helium_api::command::Command;
use helium_core::DataBase;
use std::sync::{Arc, Mutex, mpsc};

pub struct ControlPanelUI;

impl UI for ControlPanelUI {
    fn update(
        &mut self,
        ctx: &egui::Context,
        ui: Option<&mut egui::Ui>,
        _database: &Arc<Mutex<DataBase>>,
        tx_command: &mpsc::Sender<Command>,
    ) {
        if let Some(ui) = ui {
            if ui.button("Stop logging").clicked() {
                tx_command.send(Command::ClosePort).unwrap();
            }
        }
    }
}
