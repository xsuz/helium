use crate::UI;
use helium_api::query::Query;
use helium_core::{Cursor, DataBase};
use std::sync::{Arc, Mutex, mpsc};

pub struct BarometerUI {
    cursor: Cursor,
}

impl BarometerUI {
    pub fn new() -> Self {
        BarometerUI {
            cursor: Cursor::new(30000, 0, None),
        }
    }
}

impl UI for BarometerUI {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _ui: Option<&mut egui::Ui>,
        database: &Arc<Mutex<DataBase>>,
        _tx_command: &mpsc::Sender<Query>,
    ) {
        egui::Window::new("Barometer").show(ctx, |ui| {
            if let Ok(db) = database.lock() {
                if let Some((barometer, timestamp)) = db.barometer.data.last() {
                    ui.label(format!(
                        "Barometer: {} °C, Timestamp: {}",
                        barometer.temperature, timestamp
                    ));
                }

                self.cursor.update(&db.barometer);

                egui_plot::Plot::new("Barometer")
                    .legend(egui_plot::Legend::default())
                    .show(ui, |plt_ui| {
                        if db.barometer.data.len() > 100 {
                            let point_barometer: egui_plot::PlotPoints = db.barometer.data
                                [self.cursor.index..]
                                .iter()
                                .map(|(barometer, timestamp)| {
                                    let temperature = barometer.temperature as f64;
                                    [*timestamp as f64, temperature]
                                })
                                .collect();
                            plt_ui.line(
                                egui_plot::Line::new("barometer", point_barometer)
                                    .color(egui::Color32::from_rgb(0, 64, 255))
                                    .fill(10.0)
                                    .width(2.0),
                            );
                            plt_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                                [
                                    (db.barometer.data.last().unwrap().1 - self.cursor.range)
                                        as f64,
                                    10.0,
                                ],
                                [db.barometer.data.last().unwrap().1 as f64, 40.0],
                            ));
                        }
                    });
            }
        });
    }
}
