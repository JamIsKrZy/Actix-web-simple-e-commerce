pub mod public {
    use std::borrow::Cow;

    use actix_session::Session;
    use actix_web::{
        HttpResponse, Responder, get,
        http::{StatusCode, header},
        post,
        web::{self, ServiceConfig},
    };
    use db_core::{
        PostgressDbManager,
        ctx::Context,
        models::user::{self, Login, RawPassword, SignUpUser},
    };
    use lib_core::{AppPasswordHasher, Claim, JwtHandler};
    use support_core::password_hasher::HashPassword;

    use crate::handlers::HandlerResult;

    pub fn scope(cfg: &mut ServiceConfig) {
        cfg.service(login).service(signup);
    }

    #[utoipa::path(
        path="/api/login",
        responses(
            (status = 200, description="User Granted"),
            (status = 401, description="Wrong password"),
            (status = 404, description="User Not Found"),
            (status = 500, description="Cause: Failed signing claim, Failed cookie session")
        )
    )]
    #[post("/login")]
    pub async fn login(
        info: web::Json<Login<RawPassword>>,
        pass_hash: web::Data<AppPasswordHasher>,
        db: web::Data<PostgressDbManager>,
        jwt: web::Data<JwtHandler>,
        session: Session,
    ) -> HandlerResult<HttpResponse> {
        let username = info.username.as_str();
        let dm = db.get_ref();
        let stored_user = user::Bmc::fetch_one_credential(username, dm)
            .await
            .map_err(|_| {
                crate::Error::FailedRequestProcess(
                    HttpResponse::NotFound().body("Invalid username or password!"),
                )
            })?;

        let hasher = pass_hash.into_inner();
        info.into_inner()
            .verify_password(hasher, stored_user.password)
            .await
            .map_err(|_| crate::Error::InternalError)?
            .map_err(|_| {
                crate::Error::FailedRequestProcess(
                    HttpResponse::Unauthorized().body("Invalid username or password!"),
                )
            })?;

        let claim = Claim::new().map_err(|_| crate::Error::InternalError)?;

        let claim = jwt
            .encode(&claim)
            .map_err(|_| crate::Error::InternalError)?;

        let ctx = Context::new(stored_user.id, stored_user.role);

        // Give User session
        let _ = session
            .insert("usr_ctx", ctx)
            .map_err(|_| crate::Error::InternalError);

        Ok(HttpResponse::Ok()
            .insert_header((header::AUTHORIZATION, format!("Bearer {claim}")))
            .body(""))
    }

    #[utoipa::path(
        path="/api/check",
        description="checks the user session cookie\nthis does check the client user account is existing",
        responses(
            (status = 200, description="User session id is valid"),
            (status = 404, description="Cause: invalid cookie"),
            (status = 500, description="Cause: Failed deserialization")
        )
    )]
    #[get("/check")]
    pub async fn check(session: Session) -> impl Responder {
        let contxt = session.get::<Context>("usr_ctx");

        match contxt {
            Ok(o) => match o {
                Some(ctxt) => HttpResponse::Ok().body("User session cookie is valid"),
                None => HttpResponse::NotFound().body("User Session is not found!"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Error Failed deserialize"),
        }
    }

    #[utoipa::path(
        path="/api/signup",
        responses(
            (status = 201, description="User created",
                headers(
                    ("Authorization" = String, description = "JWT token returned"),
                    ("Set-Cookie" = String, description = "Session cookie")
                )
            ),
            (status = 400, description="User already exist in the database"),
            (status = 500, description="Cuase: failed hash, jwt encoding, cookie insert, ")
       )
    )]
    #[post("/signup")]
    pub async fn signup(
        info: web::Form<SignUpUser<RawPassword>>,
        pass_hash: web::Data<AppPasswordHasher>,
        db: web::Data<PostgressDbManager>,
        jwt: web::Data<JwtHandler>,
        session: Session,
    ) -> HandlerResult<HttpResponse> {
        let hasher = pass_hash.clone().into_inner();
        let dm = db.get_ref();

        let hashed_data = info
            .into_inner()
            .hash_password(hasher)
            .await
            .map_err(|_| crate::Error::InternalError)? // web::Block Error
            .map_err(|_| crate::Error::InternalError)?; // Failed hash error

        let ctx = user::Bmc::insert(hashed_data, dm).await.map_err(|_| {
            crate::Error::ErrorResponse(StatusCode::CONFLICT, Cow::Borrowed("User Already exist"))
        })?;

        let claim = Claim::new().map_err(|_| crate::Error::InternalError)?;

        let claim = jwt
            .encode(&claim)
            .map_err(|_| crate::Error::InternalError)?;

        // Give User session
        let _ = session
            .insert("usr_ctx", ctx)
            .map_err(|_| crate::Error::InternalError);

        Ok(HttpResponse::Created()
            .insert_header((header::AUTHORIZATION, format!("Bearer {claim}")))
            .body(""))
    }
}

pub mod user {}

pub mod worker {}

pub mod admin {}
