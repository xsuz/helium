use helium_api::backend::{AppState, HeliumBackend};
use helium_api::query::Query;
use helium_core::DataBase;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use helium_ui::{ControlPanelUI, SelectPortUI, UI};

pub struct AppUI {
    database: Arc<Mutex<DataBase>>,
    tx_command: mpsc::Sender<Query>,
    control_panel_ui: ControlPanelUI,
    select_port_ui: SelectPortUI,
}

impl Default for AppUI {
    fn default() -> Self {
        let (tx_packet, rx_packet) = mpsc::channel::<(Vec<u8>, i64)>();
        let (tx_query, rx_query) = mpsc::channel::<Query>();
        let _handle = thread::spawn(move || {
            let mut helium_backend: HeliumBackend = HeliumBackend::new();

            loop {
                helium_backend.update(&tx_packet, &rx_query);
                match &helium_backend.get_state() {
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
            tx_command: tx_query,
            control_panel_ui: ControlPanelUI::new(),
            select_port_ui: SelectPortUI {},
        }
    }
}

impl eframe::App for AppUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "NotoSansJP-Regular".to_owned(),
            FontData::from_static(include_bytes!("../../assets/fonts/NotoSansJP-Regular.ttf")).into(),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "NotoSansJP-Regular".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("NotoSansJP-Regular".to_owned());
        
        ctx.set_fonts(fonts);

        egui::CentralPanel::default().show(ctx, |ui| {
            let (tx, rx) = mpsc::channel();
            self.tx_command.send(Query::GetAppState(tx)).unwrap();
            if let Ok(state) = rx.recv() {
                match state {
                    AppState::Quit => {
                        println!("Quit command received");
                        std::process::exit(0);
                    }
                    AppState::Unselect => {
                        self.select_port_ui
                            .update(ctx, Some(ui), &self.database, &self.tx_command);
                    }
                    AppState::Logging => {
                        self.control_panel_ui.update(
                            ctx,
                            Some(ui),
                            &self.database,
                            &self.tx_command,
                        );
                    }
                }
            }
            ctx.request_repaint_after(std::time::Duration::from_millis(25));
        });
    }
}

impl Drop for AppUI {
    fn drop(&mut self) {
        let _ = self.tx_command.send(Query::Quit);
    }
}
