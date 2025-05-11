use crate::query::Query;
use helium_core::cobs;
use std::sync::mpsc;
use std::io::prelude::*;
use std::fs::OpenOptions;

#[derive(Debug, Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub enum AppState{
    Unselect,
    Logging,
    Quit,
}

pub struct HeliumBackend {
    log: Vec<u8>,
    port: Option<Box<dyn serialport::SerialPort>>,
    state:AppState
}

impl HeliumBackend {
    pub fn new() -> Self {
        std::fs::create_dir_all(format!("log/{}",chrono::Local::now().format("%m%d"))).unwrap();
        HeliumBackend {
            log: Vec::new(),
            port: None,
            state: AppState::Unselect
        }
    }

    pub fn update(&mut self, tx_packet: &mpsc::Sender<(Vec<u8>,i64)>, rx_query: &mpsc::Receiver<Query>) {
        
        if let Ok(query) = rx_query.recv() {
            match query {
                Query::OpenPort(port) => self.open_port(port.as_str()),
                Query::ClosePort => self.close_port(),
                Query::SendData(data) => self.send(&data),
                Query::Quit=>{
                    self.state=AppState::Quit;
                },
                Query::GetSerialPort(handler)=>{
                    if let Ok(list)=serialport::available_ports(){
                        let list=list.iter().map(|info|info.port_name.clone()).collect();
                        handler.send(list).unwrap();
                    } else {
                        handler.send(vec![]).unwrap();
                    }
                },
                Query::GetAppState(handler)=>{
                    handler.send(self.get_state()).unwrap();
                }
            }
        }
        
        if let Some(port) = &mut self.port {
            let mut buffer = vec![0; 1024];
            match port.read(buffer.as_mut_slice()) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        self.log.extend_from_slice(&buffer[..bytes_read]);
                        let (mut decoded, mut rest) = cobs::decode(&self.log);
                        while decoded.len() > 0 {
                            let timestamp = chrono::Utc::now().timestamp_millis();
                            tx_packet.send((decoded.clone(), timestamp)).unwrap();

                            let mut file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .create(true)
                                .open(format!(
                                    "log/{}/id{:04x}.txt",
                                    chrono::Local::now().format("%m%d"),
                                    &decoded[0]
                                ))
                                .unwrap();
                            file.write_all(format!("{}:{:?}\n", timestamp, decoded).as_bytes())
                                .unwrap();

                            self.log = rest.to_vec();
                            (decoded, rest) = cobs::decode(&self.log);
                        }
                    }
                }

                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                Err(e) => {
                    println!("{:?}", e);
                    self.port = None;
                    self.state = AppState::Unselect;
                }
            }
        }
    }

    pub fn send(&mut self, data: &Vec<u8>) {
        if let Some(port) = &mut self.port {
            port.write_all(&cobs::encode(data)[..])
                .expect("Failed to write to port");
        } else {
            println!("Port is not open");
        }
    }

    pub fn open_port(&mut self, port_name: &str) {
        if self.port.is_none() {
            match serialport::new(port_name, 115200)
                .timeout(std::time::Duration::from_millis(10))
                .open()
            {
                Ok(port) => {
                    self.port = Some(port);
                    self.state = AppState::Logging;
                    println!("Port opened: {}", port_name);
                }
                Err(e) => {
                    println!("Failed to open port: {}", e);
                }
            }
        } else {
            println!("Port is already open");
        }
    }
    pub fn close_port(&mut self) {
        if let Some(port) = self.port.take() {
            drop(port);
            self.log.clear();
            self.state = AppState::Unselect;
            println!("Port closed");
        } else {
            println!("Port is not open");
        }
    }

    pub fn get_state(&self)->AppState{
        self.state
    }
}
