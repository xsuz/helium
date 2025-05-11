use crate::UI;
use helium_api::query::Query;
use helium_core::DataBase;
use std::sync::{Arc, Mutex, mpsc};

pub struct SelectPortUI;

impl UI for SelectPortUI {
    fn update(
        &mut self,
        _ctx: &egui::Context,
        ui: Option<&mut egui::Ui>,
        _database: &Arc<Mutex<DataBase>>,
        tx_command: &mpsc::Sender<Query>,
    ) {
        if let Some(ui) = ui {
            let (tx, rx) = mpsc::channel();
            tx_command.send(Query::GetSerialPort(tx)).unwrap();
            if let Ok(list) = rx.recv() {
                if list.len() == 0 {
                    ui.label("No serial ports available");
                } else {
                    egui::ComboBox::from_label("Serial Port")
                        .selected_text("Select a port")
                        .show_ui(ui, |ui| {
                            for port in list {
                                if ui.button(port.clone()).clicked() {
                                    tx_command.send(Query::OpenPort(port)).unwrap();
                                }
                            }
                        });
                }
            }
        }
    }
}
