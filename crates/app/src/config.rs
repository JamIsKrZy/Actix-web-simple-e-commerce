use std::sync::{OnceLock};

use actix_web::{cookie::Key, web::{self, Data}};
use db_core::PostgressDbManager;
use env_logger::Env;
use lib_core::JwtHandler;
use rustls::{pki_types::{pem::PemObject as _, CertificateDer, PrivateKeyDer}, ServerConfig};



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


pub fn config_envrironment(){
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    #[cfg(feature = "dev_env")]
    {
        println!("🛠️ Setting Dev Environement...");
        dotenvy::from_filename_override("./dev/.env").expect("Unable to locate dev environment");
    }
}

pub async fn app_data_resource() -> (web::Data<PostgressDbManager>, web::Data<JwtHandler>) {

    let dm = web::Data::new(
        if cfg!(feature = "dev_env") {
            PostgressDbManager::init_test_connection().await
        } else {
            PostgressDbManager::new(10).await
        }
    );

    let jwt_handler = web::Data::new(
        if cfg!(feature = "dev_env") {
            JwtHandler::default()
        } else {
            JwtHandler::new()
        }
    );


    (dm, jwt_handler)
}


pub fn load_rustls() -> rustls::ServerConfig{
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();



    // load TLS key/cert files
    let cert_chain = if cfg!(feature = "dev_env"){
        CertificateDer::pem_file_iter("./dev/cert.pem")
            .unwrap()
            .flatten()
            .collect()
    } else {
        CertificateDer::pem_file_iter("cert.pem")
            .unwrap()
            .flatten()
            .collect()    
    };
    

    let key_der = if cfg!(feature = "dev_env") {
        PrivateKeyDer::from_pem_file("./dev/key.pem").expect("Could not locate PKCS 8 private keys.")
    } else {
        PrivateKeyDer::from_pem_file("key.pem").expect("Could not locate PKCS 8 private keys.")
    };
    
    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)
        .unwrap()
}

#[derive(Debug, Clone)]
pub struct Config{
    pub addr: String,
    pub port: u16,
    pub workers: usize,
    pub secret: String,
}

impl Config{

    pub fn addr_tuple(&self) -> (&str, u16){
        (self.addr.as_str(), self.port)
    }

    pub fn address(&self) -> String{
        format!("{}:{}", self.addr, self.port) 
    }

    pub fn session_key(&self) -> Key{
        Key::from(self.secret.as_bytes())
    }

}