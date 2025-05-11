use std::sync::mpsc;

use crate::backend::AppState;
pub enum Query{
    OpenPort(String),
    ClosePort,
    SendData(Vec<u8>),
    GetSerialPort(mpsc::Sender<Vec<String>>),
    GetAppState(mpsc::Sender<AppState>),
    Quit
}