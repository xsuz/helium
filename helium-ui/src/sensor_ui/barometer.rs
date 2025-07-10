use crate::UI;
use egui::RichText;
use helium_api::query::Query;
use helium_core::{Cursor, DataBase};
use std::sync::{Arc, Mutex, mpsc};

pub struct BarometerUI {
    cursor: Cursor,
}

impl BarometerUI {
    pub fn new() -> Self {
        BarometerUI {
            cursor: Cursor::new(30000, 0),
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
                let utc_now = chrono::Utc::now().timestamp_millis();

                if let Some((barometer, timestamp)) = db.barometer.data.last() {
                    if (utc_now - timestamp) > self.cursor.range {
                        ui.label(
                            RichText::new(format!(
                                "Barometer: {} °C, Timestamp: {} (out of range)",
                                barometer.temperature, timestamp
                            ))
                            .color(egui::Color32::RED),
                        );
                    } else {
                        ui.label(format!(
                            "Barometer: {} °C, Timestamp: {}",
                            barometer.temperature, timestamp
                        ));
                    }
                } else {
                    ui.label(
                        RichText::new("No barometer data available.").color(egui::Color32::RED),
                    );
                }

                self.cursor.update(&db.barometer, Some(utc_now));

                egui_plot::Plot::new("Barometer")
                    .legend(egui_plot::Legend::default())
                    .show(ui, |plt_ui| {
                        let point_barometer: egui_plot::PlotPoints = db.barometer.data
                            [self.cursor.index..]
                            .iter()
                            .map(|(barometer, utc)| {
                                let temperature = barometer.temperature as f64;
                                [(*utc - utc_now) as f64 / 1000.0, temperature]
                            })
                            .collect();
                        plt_ui.line(
                            egui_plot::Line::new("barometer", point_barometer)
                                .color(egui::Color32::from_rgb(0, 64, 255))
                                .fill(10.0)
                                .width(2.0),
                        );
                        plt_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                            [-self.cursor.range as f64 / 1000.0, 10.0],
                            [0.0, 40.0],
                        ));
                    });
            }
        });
    }
}
