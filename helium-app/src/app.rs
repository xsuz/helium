use helium_api::backend::{AppState,Helium};
use helium_api::command::Command;
use helium_core::DataBase;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use helium_ui::{UI,SelectPortUI,ControlPanelUI};

pub struct AppUI {
    database: Arc<Mutex<DataBase>>,
    tx_command: mpsc::Sender<Command>,
}

impl Default for AppUI {
    fn default() -> Self {
        let (tx_packet, rx_packet) = mpsc::channel::<(Vec<u8>, i64)>();
        let (tx_command, rx_command) = mpsc::channel::<Command>();
        let _handle = thread::spawn(move || {
            let mut helium: Helium = Helium::new();

            loop {
                helium.update(&tx_packet, &rx_command);
                match &helium.get_state() {
                    AppState::Quit => break,
                    _ => {}
                }
            }
        });
        let database: Arc<Mutex<DataBase>> = Arc::new(Mutex::new(DataBase::new()));

        thread::spawn({
            let database = Arc::clone(&database);
            move || {
                for (decoded, timestamp) in rx_packet {
                    if let Ok(mut data) = database.lock() {
                        data.update(&decoded, Some(timestamp));
                    }
                }
            }
        });

        Self {
            database,
            tx_command
        }
    }
}


impl eframe::App for AppUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (tx, rx) = mpsc::channel();
            self.tx_command.send(Command::GetAppState(tx)).unwrap();
            if let Ok(state) = rx.recv() {
                match state {
                    AppState::Quit => {
                        println!("Quit command received");
                        std::process::exit(0);
                    }
                    AppState::Unselect => {
                        SelectPortUI.update(ctx, Some(ui), &self.database, &self.tx_command);
                    }
                    AppState::Logging => {
                        ControlPanelUI.update(ctx, Some(ui), &self.database, &self.tx_command);
                    }
                }
            }
            ctx.request_repaint_after(std::time::Duration::from_millis(25));
        });
    }
}

impl Drop for AppUI {
    fn drop(&mut self) {
        let _ = self.tx_command.send(Command::Quit);
    }
}
