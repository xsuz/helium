use std::sync::{mpsc, Arc, Mutex};
use helium_core::DataBase;

use crate::backend::AppState;
pub enum Query{
    OpenPort(String),
    ClosePort,
    SendData(Vec<u8>),
    GetDataBase(mpsc::Sender<Arc<Mutex<DataBase>>>),
    GetSerialPort(mpsc::Sender<Vec<String>>),
    GetAppState(mpsc::Sender<AppState>),
    Quit
}