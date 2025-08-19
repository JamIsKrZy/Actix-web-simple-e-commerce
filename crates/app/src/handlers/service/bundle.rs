
pub mod public {

}

pub mod user {

}

pub mod worker {

}

pub mod admin {
    use actix_web::web::ServiceConfig;

    pub fn scope(cfg: &mut ServiceConfig){
        
    }
}