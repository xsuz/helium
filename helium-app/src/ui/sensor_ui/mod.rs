pub mod altitude;
pub mod barometer;
pub mod gps;
pub mod pitot;
pub mod servo;

use std::sync::{Arc, Mutex, mpsc};
use helium_core::DataBase;
use helium_api::Query;

pub use altitude::AltitudeUI;
pub use barometer::BarometerUI;
pub use gps::GPSUI;
pub use pitot::PitotUI;
pub use servo::ServoUI;

pub trait UI{
    fn update(&mut self,ctx:&egui::Context,ui:Option<&mut egui::Ui>,database:&Arc<Mutex<DataBase>>,tx_command:&mpsc::Sender<Query>);
}