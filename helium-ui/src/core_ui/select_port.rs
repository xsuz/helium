use helium_api::query::Query;
use std::sync::mpsc;

pub struct SelectPortUI;

impl SelectPortUI {
    pub fn update(
        &mut self,
        _ctx: &egui::Context,
        ui: Option<&mut egui::Ui>,
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
