use eframe::egui;
use byteorder::{BigEndian, ByteOrder};
use super::AppUI;

pub struct AltitudeUI {
    offset:f32,
    raw_alt:f32
}

impl AltitudeUI {
    pub fn new() -> Self {
        Self {
            offset:-1.2,
            raw_alt : 0.0,
        }
    }
}

impl AppUI for AltitudeUI {
    fn update(&mut self, data: &mut crate::parse::Parser, ctx: &eframe::egui::Context) {
        egui::Window::new("Altitude").vscroll(true).show(ctx, |ui| {
            egui::SidePanel::right("Altitude_r_panel")
                .resizable(false)
                .default_width(400.0)
                .show_inside(ui, |ui| {
                    // structALTDisplayData{
                    //  uint8_t id;
                    //  float diff_alt_from_10;
                    // }
                    ui.add(egui::DragValue::new(&mut self.raw_alt));
                    if ui.button("Send diff altidude from 10m").clicked(){
                        let mut bytes : [u8;8]=[0;8];
                        bytes[0] = 0xD5; // message id
                        BigEndian::write_f32(&mut bytes[4..8], self.raw_alt-10.0);
                        data.write(&bytes.to_vec());
                        self.offset=-self.raw_alt+10.0;
                    }
                });
            egui::CentralPanel::default().show_inside(ui, |ui| {
                if let Some((alt_data, _)) = data.get_ultra_sonic_data().last() {
                    ui.heading(format!(
                        "altitude:\t{:2.2}m\ttimestamp:\t{}ms",
                        alt_data.altitude, alt_data.timestamp
                    ));
                }

                let mut p0 = 101300.0;
                if let Some((barometer_data, _)) = data.get_barometer_data(1).last() {
                    p0 = barometer_data.pressure;

                    if let Some((barometer_data0, _)) = data.get_barometer_data(0).last() {
                        ui.label(format!(
                            "Pressure: {:2.2}Pa\tTemperature: {:2.2}C",
                            barometer_data0.pressure, barometer_data0.temperature
                        ));
                        ui.heading(format!(
                            "altitude:\t{:2.2}m\ttimestamp:\t{}ms",
                            44330.0 * (1.0 - (barometer_data0.pressure / p0).powf(1.0 / 5.255)),
                            barometer_data0.timestamp
                        ));
                    }
                }

                egui_plot::Plot::new("Altitude")
                    .legend(egui_plot::Legend::default())
                    .show(ui, |plt_ui| {
                        let point_ultra_sonic: egui_plot::PlotPoints = data
                            .get_ultra_sonic_data()
                            .iter()
                            .map(|(data, time)| [*time as f64, data.altitude as f64])
                            .collect();

                        let point_barometer: egui_plot::PlotPoints = data
                            .get_barometer_data(0)
                            .iter()
                            .map(|(baro0_data, baro0_time)| {
                                if let Some((baro1_data, _)) = data
                                    .get_barometer_data(1)
                                    .iter()
                                    .rfind(|(_, baro1_time)| baro1_time <= baro0_time)
                                {
                                    [
                                        *baro0_time as f64,
                                        44330.0
                                            * (1.0
                                                - (baro0_data.pressure / baro1_data.pressure)
                                                    .powf(1.0 / 5.255))
                                                as f64,
                                    ]
                                } else {
                                    [
                                        *baro0_time as f64,
                                        44330.0
                                            * (1.0 - (baro0_data.pressure / p0).powf(1.0 / 5.255))
                                                as f64,
                                    ]
                                }
                            })
                            .collect();

                        plt_ui.line(
                            egui_plot::Line::new(point_ultra_sonic)
                                .color(egui::Color32::from_rgb(0, 64, 255))
                                .name("ultra sonic")
                                .fill(0.0),
                        );

                        plt_ui.line(
                            egui_plot::Line::new(point_barometer)
                                .color(egui::Color32::from_rgb(255, 64, 0))
                                .name("barometer")
                                .fill(0.0),
                        );
                    });
            });
        });
    }
}
