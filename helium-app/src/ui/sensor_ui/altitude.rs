use crate::ui::sensor_ui::UI;
use egui::RichText;
use helium_api::query::Query;
use helium_core::{Cursor, DataBase};
use std::sync::{Arc, Mutex, mpsc};

pub struct AltitudeUI {
    cursor: Cursor,
    range: i64,
}

impl AltitudeUI {
    pub fn new() -> Self {
        AltitudeUI {
            cursor: Cursor::new(30000, 0),
            range: 30, // 30 seconds
        }
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
                let utc_now = chrono::Utc::now().timestamp_millis();
                if let Some((ultrasonic, timestamp)) = db.ultra_sonic.data.last() {
                    if (utc_now - timestamp) > self.cursor.range {
                        ui.label(
                            RichText::new(format!(
                                "Ultrasonic: {} cm, Timestamp: {} (out of range)",
                                ultrasonic.altitude, timestamp
                            ))
                            .color(egui::Color32::RED),
                        );
                    } else {
                        ui.label(format!(
                            "Ultrasonic: {} cm, Timestamp: {}",
                            ultrasonic.altitude, timestamp
                        ));
                    }
                    ui.add(
                        egui::Slider::new(&mut self.range, 10..=3600 * 2)
                            .text("Range (seconds)")
                            .logarithmic(true),
                    );
                } else {
                    ui.label(
                        RichText::new("No ultrasonic data available.").color(egui::Color32::RED),
                    );
                }

                egui_plot::Plot::new("Altitude")
                    .legend(egui_plot::Legend::default())
                    .show(ui, |plt_ui| {
                        self.cursor.range = self.range * 1000;
                        self.cursor.update(&db.ultra_sonic, Some(utc_now));

                        let point_ultrasonic: egui_plot::PlotPoints = db.ultra_sonic.data
                            [self.cursor.index..]
                            .iter()
                            .map(|(ultrasonic, utc)| {
                                let altitude = ultrasonic.altitude as f64;
                                [(*utc - utc_now) as f64 / 1000.0, altitude]
                            })
                            .collect();
                        plt_ui.line(
                            egui_plot::Line::new("ultrasonic", point_ultrasonic)
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
