use crate::UI;
use helium_api::query::Query;
use helium_core::DataBase;
use std::sync::{Arc, Mutex, mpsc};

use crate::sensor_ui;

pub struct ControlPanelUI {
    database: Option<Arc<Mutex<DataBase>>>,
    altitude_ui: sensor_ui::AltitudeUI,
    barometer_ui: sensor_ui::BarometerUI,
    gps_ui: sensor_ui::GPSUI,
    pitot_ui: sensor_ui::PitotUI,
    servo_ui: sensor_ui::ServoUI,
}

impl ControlPanelUI {
    pub fn new() -> Self {
        ControlPanelUI {
            database: None,
            altitude_ui: sensor_ui::AltitudeUI::new(),
            barometer_ui: sensor_ui::BarometerUI::new(),
            gps_ui: sensor_ui::GPSUI::new(),
            pitot_ui: sensor_ui::PitotUI::new(),
            servo_ui: sensor_ui::ServoUI::new(),
        }
    }
}

impl ControlPanelUI {
    pub fn update(
        &mut self,
        ctx: &egui::Context,
        ui: Option<&mut egui::Ui>,
        tx_command: &mpsc::Sender<Query>,
    ) {
        let (tx, rx) = mpsc::channel();
        tx_command.send(Query::GetDataBase(tx)).unwrap();
        if let Ok(db) = rx.recv() {
            self.database = Some(db);
        }
        if let Some(database) = &self.database {
            if let Some(ui) = ui {
                if ui.button("Stop logging").clicked() {
                    tx_command.send(Query::ClosePort).unwrap();
                }
                self.altitude_ui.update(ctx, None, database, tx_command);
                self.barometer_ui.update(ctx, None, database, tx_command);
                self.gps_ui.update(ctx, None, database, tx_command);
                self.pitot_ui.update(ctx, None, database, tx_command);
                self.servo_ui.update(ctx, None, database, tx_command);
            }
        }
    }
}
