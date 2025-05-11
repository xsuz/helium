use crate::UI;
use helium_api::query::Query;
use helium_core::DataBase;
use std::sync::{Arc, Mutex, mpsc};

use crate::sensor_ui;

pub struct ControlPanelUI{
    altitude_ui: sensor_ui::AltitudeUI,
    barometer_ui: sensor_ui::BarometerUI,
}

impl ControlPanelUI {
    pub fn new() -> Self {
        ControlPanelUI {
            altitude_ui: sensor_ui::AltitudeUI::new(),
            barometer_ui: sensor_ui::BarometerUI::new(),
        }
    }
}

impl UI for ControlPanelUI {
    fn update(
        &mut self,
        ctx: &egui::Context,
        ui: Option<&mut egui::Ui>,
        database: &Arc<Mutex<DataBase>>,
        tx_command: &mpsc::Sender<Query>,
    ) {
        if let Some(ui) = ui {
            if ui.button("Stop logging").clicked() {
                tx_command.send(Query::ClosePort).unwrap();
            }
            self.altitude_ui.update(ctx, None, database, tx_command);
            self.barometer_ui.update(ctx, None, database, tx_command);
        }
    }
}
