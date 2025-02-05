use err_mac::create_err_with_impls;
use parking_lot::Mutex;
use std::{env::VarError, time::SystemTimeError};
use std::{net::TcpStream, sync::Arc};
use tungstenite::{self, connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

create_err_with_impls!(
    #[derive(Debug)]
    pub LatErr,
    FailParseSocketMsg(String),
    Var(VarError),
    Time(SystemTimeError),
    Url(url::ParseError),
    IO(std::io::Error),
    Json(serde_json::Error),
    Tungstenite(tungstenite::Error),
    ;
);

pub struct _Socket {
    _s: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl _Socket {
    fn _flush(&mut self) -> Result<(), LatErr> {
        self._s.flush()?;
        Ok(())
    }
    fn _write(&mut self, m: String) -> Result<(), LatErr> {
        self._s.write(Message::Text(m))?;
        Ok(())
    }
    fn _send(&mut self, m: String) -> Result<(), LatErr> {
        self._s.send(Message::Text(m))?;
        Ok(())
    }
    fn _read(&mut self) -> Result<String, LatErr> {
        let m = self._s.read()?.to_string();
        Ok(m)
    }
}

#[derive(Clone)]
pub struct Socket {
    _s: Arc<Mutex<_Socket>>,
}

impl Drop for _Socket {
    fn drop(&mut self) {
        let _ = self._s.close(None);
        let _ = self._s.flush();
    }
}

impl Socket {
    pub fn send_multi(&self, msgs: Vec<String>) -> Result<(), LatErr> {
        let mut s = self._s.lock();
        for m in msgs {
            s._write(m)?;
        }
        s._flush()
    }
    pub fn send(&self, msg: String) -> Result<(), LatErr> {
        self._s.lock()._send(msg)
    }
    pub fn read(&self) -> Result<String, LatErr> {
        self._s.lock()._read()
    }
    pub fn new(url: &str) -> Result<Self, LatErr> {
        let url = Url::parse(url)?;
        let (socket, _) = connect(url)?;
        println!("CONNECT SOCKET",);
        let a = Self {
            _s: Arc::new(Mutex::new(_Socket { _s: socket })),
        };
        Ok(a)
    }
}
