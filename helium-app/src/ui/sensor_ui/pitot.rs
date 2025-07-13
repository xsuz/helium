use crate::ui::sensor_ui::UI;
use helium_api::query::Query;
use helium_core::{Cursor, DataBase};
use std::sync::{Arc, Mutex, mpsc};

pub struct PitotUI {
    cursor: Cursor,
    range: i64,
}

impl PitotUI {
    pub fn new() -> Self {
        Self {
            cursor: Cursor::new(300.0 as i64 * 1000, 0),
            range: 300, // 300 seconds in milliseconds
        }
    }
}

impl UI for PitotUI {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _ui: Option<&mut egui::Ui>,
        database: &Arc<Mutex<DataBase>>,
        _tx_command: &mpsc::Sender<Query>,
    ) {
        // if let Some((pitot_data, _)) = data.get_pitot1_data().last() {
        //     egui::Window::new("Pitot1").vscroll(true).show(ctx, |ui| {
        //         ui.heading(format!(
        //             "IAS:\t{:2.2}m/s\ttimestamp:\t{}ms",
        //             pitot_data.velocity, pitot_data.timestamp
        //         ));
        //     });
        // }
        if let Ok(db) = database.lock() {
            if let Some((pitot_data, _)) = db.pitot.data.last() {
                egui::Window::new("Pitot").vscroll(true).show(ctx, |ui| {
                    ui.heading(format!(
                        "IAS:\t{:2.2}m/s\ttimestamp:\t{}ms",
                        pitot_data.velocity, pitot_data.timestamp
                    ));
                    ui.add(
                        egui::Slider::new(&mut self.range, 10..=3600 * 2)
                            .text("Range (seconds)")
                            .logarithmic(true),
                    );
                    let utc_now = chrono::Utc::now().timestamp_millis();
                    self.cursor.range = self.range * 1000;
                    self.cursor.update(&db.pitot, Some(utc_now));
                    egui_plot::Plot::new("velocity")
                        .legend(egui_plot::Legend::default())
                        .show(ui, |plt_ui| {
                            let point_ias: egui_plot::PlotPoints = db.pitot.data
                                [self.cursor.index..]
                                .iter()
                                .map(|(_data, utc)| {
                                    [(*utc - utc_now) as f64 / 1000.0, _data.velocity as f64]
                                })
                                .collect();

                            plt_ui.line(
                                egui_plot::Line::new("IAS", point_ias)
                                    .color(egui::Color32::from_rgb(255, 0, 0))
                                    .fill(0.0),
                            );

                            plt_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                                [-self.cursor.range as f64 / 1000.0, 10.0],
                                [0.0, 40.0],
                            ));
                        });
                });
            }
        }
    }
}
