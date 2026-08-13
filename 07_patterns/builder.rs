use reqwest::tls::TlsInfo;

#[derive(Debug, Clone)]
pub struct TLSCest{
    key: String,
    cert: String,
}
impl TLSCest {
    pub fn new(key: String, cert: String) -> Self{
        Self { key, cert }
    }
}
type ms = u32;

#[derive(Debug)]
pub struct Server{
    host: String,
    port: u16,
    tls: Option<TLSCest>,
    hot_reload: bool,
    timeout: ms,
}

impl Server{
    pub fn new(host: String, port: u16) -> ServerBuilder{
        ServerBuilder { host, port, tls: None, hot_reload: None, timeout: None }
    }

    // pub fn new_tls (host: String, port: u16, tls: TLSCest) -> Self{
    //             Self { host, port, tls: Some(tls), hot_reload: false, timeout: 2000 }
    // }

    // pub fn new_advance(
    //     host: String,
    //     port: u16,
    //     tls: Option<TLSCest>,
    //     hot_reload: bool,
    //     timeout: ms,) -> Self {
    //         Server { host, port, tls, hot_reload, timeout }
    // }
}

pub struct ServerBuilder{
    host: String,
    port: u16,
    tls: Option<TLSCest>,
    hot_reload: Option<bool>,
    timeout: Option<ms>,
}

impl ServerBuilder{
    pub fn tls(&mut self, tls: TLSCest) -> &mut Self{
        self.tls= Some(tls);
        self
    }
    pub fn hot_reload(&mut self, hot_reload: bool) -> &mut Self{
        self.hot_reload= Some(hot_reload);
        self
    }
    pub fn timeout(&mut self, timeout: ms) -> &mut Self{
        self.timeout= Some(timeout);
        self
    }

    pub fn build(&mut self) -> Server{
        Server { 
            host: self.host.clone(), 
            port: self.port, 
            tls: self.tls.clone(), 
            hot_reload: self.hot_reload.unwrap_or_default(), 
            timeout: self.timeout.unwrap_or(2000)
        }
    }
}
