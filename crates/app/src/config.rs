use std::sync::{OnceLock};

use actix_web::cookie::Key;



pub fn config() -> Config{
    static CFG: OnceLock<Config> = OnceLock::new();

    CFG.get_or_init(|| {

        let addr = dotenvy::var("SERVICE_ADDR").unwrap_or_else(|e|{
            panic!("Undefined \"SERVICE_ADDR\" Environment: {e:?}")
        });

        let port =  dotenvy::var("SERVICE_PORT").unwrap_or_else(|e|{
            panic!("Undefined \"SERVICE_PORT\" Environment: {e:?}")
        })
        .parse()
        .unwrap_or_else(|e| {
            panic!("Unable to parse variable: {e:?}")
        });

        let workers = dotenvy::var("SERVICE_WORKER").unwrap_or_else(|e|{
            panic!("Undefined \"SERVICE_WORKER\" Environment: {e:?}")
        })
        .parse()
        .unwrap_or_else(|e| {
            panic!("Unable to parse variable: {e:?}")
        });

        let secret = dotenvy::var("SECRET").unwrap_or_else(|e|{
            panic!("Undefined \"SECRET\" Environment: {e:?}")
        });


        Config { 
            addr,
            port,
            workers,
            secret
        }
    })
    .clone()
}


#[derive(Debug, Clone)]
pub struct Config{
    pub addr: String,
    pub port: u16,
    pub workers: usize,
    pub secret: String,
}

impl Config{

    pub fn bind_addr(&self) -> (&str, u16){
        (self.addr.as_str(), self.port)
    }

    pub fn session_key(&self) -> Key{
        Key::from(self.secret.as_bytes())
    }

}