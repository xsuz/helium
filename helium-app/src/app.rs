use helium_api::backend::{AppState, HeliumBackend};
use helium_api::query::Query;
use std::sync::mpsc;
use std::thread;

use eframe::egui;
use helium_ui::{ControlPanelUI, SelectPortUI};

pub struct AppUI {
    tx_command: mpsc::Sender<Query>,
    control_panel_ui: ControlPanelUI,
    select_port_ui: SelectPortUI,
}

impl Default for AppUI {
    fn default() -> Self {
        let (tx_query, rx_query) = mpsc::channel::<Query>();
        let _ = thread::spawn(move || {
            let mut helium_backend: HeliumBackend = HeliumBackend::new();

            loop {
                helium_backend.update(&rx_query);
                match &helium_backend.get_state() {
                    AppState::Quit => break,
                    _ => {}
                }
            }
        });

        Self {
            tx_command: tx_query,
            control_panel_ui: ControlPanelUI::new(),
            select_port_ui: SelectPortUI {},
        }
    }
}

impl eframe::App for AppUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

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
                        self.select_port_ui.update(ctx, Some(ui), &self.tx_command);
                    }
                    AppState::Logging => {
                        self.control_panel_ui
                            .update(ctx, Some(ui), &self.tx_command);
                    }
                }
            }
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        });
    }
}

impl Drop for AppUI {
    fn drop(&mut self) {
        let _ = self.tx_command.send(Query::Quit);
    }
}
