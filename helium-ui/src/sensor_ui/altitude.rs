use crate::UI;
use helium_api::query::Query;
use helium_core::{DataBase,Cursor};
use std::sync::{Arc, Mutex, mpsc};

pub struct AltitudeUI {
    cursor: Cursor,
}

impl AltitudeUI {
    pub fn new() -> Self {
        AltitudeUI { cursor: Cursor::new(30000, 0, None) }
    }
}

impl UI for AltitudeUI {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _ui: Option<&mut egui::Ui>,
        database: &Arc<Mutex<DataBase>>,
        _tx_command: &mpsc::Sender<Query>,
    ) {
        egui::Window::new("Altitude").show(ctx, |ui| {
            if let Ok(db) = database.lock() {
                if let Some((ultrasonic, timestamp)) = db.ultra_sonic.data.last() {
                    ui.label(format!(
                        "Ultrasonic: {} cm, Timestamp: {}",
                        ultrasonic.altitude, timestamp
                    ));
                }

                egui_plot::Plot::new("Altitude")
                    .legend(egui_plot::Legend::default())
                    .show(ui, |plt_ui| {
                        if db.ultra_sonic.data.len() > 100 {

                            self.cursor.update(&db.ultra_sonic);

                            let point_ultrasonic: egui_plot::PlotPoints = db.ultra_sonic.data
                                [self.cursor.index..]
                                .iter()
                                .map(|(ultrasonic, timestamp)| {
                                    let altitude = ultrasonic.altitude as f64;
                                    [*timestamp as f64, altitude]
                                })
                                .collect();
                            plt_ui.line(
                                egui_plot::Line::new("ultrasonic", point_ultrasonic)
                                    .color(egui::Color32::from_rgb(0, 64, 255))
                                    .fill(10.0)
                                    .width(2.0),
                            );
                            plt_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                                [(db.ultra_sonic.data.last().unwrap().1 - self.cursor.range) as f64, 10.0],
                                [db.ultra_sonic.data.last().unwrap().1 as f64, 40.0],
                            ));
                        }
                    });
            }
        });
    }
}
