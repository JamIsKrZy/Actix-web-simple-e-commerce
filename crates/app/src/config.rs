use std::sync::{OnceLock};



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


        Config { 
            addr,
            port,
            workers
        }
    })
    .clone()
}


#[derive(Debug, Clone)]
pub struct Config{
    pub addr: String,
    pub port: u16,
    pub workers: usize
}

impl Config{

    pub fn bind_addr(&self) -> (&str, u16){
        (self.addr.as_str(), self.port)
    }

}