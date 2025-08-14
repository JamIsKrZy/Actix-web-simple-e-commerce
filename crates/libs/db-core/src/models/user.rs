use std::marker::PhantomData;

use support_core::password_hasher::{GetPassword, PasswordHashifier};
use sqlx::{prelude::{FromRow, Type}, Database, Postgres};
use uuid::Uuid;

use crate::{ctx::Context, utils::DbPoolExtract, DbError};

use super::QueryResult;
use serde::{Deserialize, Serialize};

use support_core::password_hasher::HashPassword;

// region:    --- States

pub trait PasswordState{}
#[derive(Debug)]
pub struct RawPassword;
impl PasswordState for RawPassword{}

#[derive(Debug)]
pub struct HashedPassword;
impl PasswordState for HashedPassword{}
// endregion: --- States

#[derive(Debug, Type, Clone, Serialize, Deserialize)]
#[sqlx(type_name="user_role", rename_all="PascalCase")]
pub enum Role {
    Regular,
    Worker,
    Admin
}


// region:    --- Schemas
#[derive(Debug, Deserialize)]
pub struct Login<S: PasswordState>{
    pub username: String,
    pub password: String,
    #[serde(skip)]
    _phantom: PhantomData<S>
}

#[derive(Debug, Deserialize, FromRow)]
pub struct UserCredential{
    pub id: Uuid,
    pub password: String,
    pub role: Role
}



#[derive(Debug, Deserialize)]
pub struct SignUpUser<S: PasswordState>{
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_no: String,
    pub location: String,
    pub password: String,
    #[serde(skip)]
    _phantom: PhantomData<S>
}

// added user coming from admin previlage
#[derive(Debug, Deserialize, FromRow)]
pub struct AddUser<S: PasswordState>{
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_no: String,
    pub location: String,
    pub password: String,
    pub role: Role,
    #[serde(skip)]
    _phantom: PhantomData<S>
}


// endregion: --- Schemas


// region:    --- Impl Traits


impl GetPassword for Login<RawPassword>{
    fn password_bytes(&self) -> &[u8] {
        self.password.trim().as_bytes()
    }
}

impl GetPassword for SignUpUser<RawPassword>{
    fn password_bytes(&self) -> &[u8] {
        self.password.trim().as_bytes()
    }
}

impl GetPassword for AddUser<RawPassword>{
    fn password_bytes(&self) -> &[u8] {
        todo!()
    }
}

impl<H> HashPassword<H> for Login<RawPassword> where 
    H: PasswordHashifier + Send + Sync + 'static
{
    type Into = Login<HashedPassword>;

    fn to(self, hashed_password: String) -> Self::Into {
        Login::<HashedPassword>{
            username: self.username,
            password: hashed_password,
            _phantom: PhantomData,
        }
    }
}

impl<H> HashPassword<H> for SignUpUser<RawPassword> where 
    H: PasswordHashifier + Send + Sync + 'static
{
    type Into = SignUpUser<HashedPassword>;

    fn to(self, hashed_password: String) -> Self::Into {
        SignUpUser::<HashedPassword>{
            username: self.username,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            phone_no: self.phone_no,
            location: self.location,
            password: hashed_password,
            _phantom: PhantomData,
        }
    }
}

impl<H> HashPassword<H> for AddUser<RawPassword> where 
    H: PasswordHashifier + Send + Sync + 'static
{
    type Into = AddUser<HashedPassword>;

    fn to(self, hashed_password: String) -> Self::Into {
        todo!()
    }
}


// endregion: --- Impl Traits

// region:    --- Users Bmc

pub struct Bmc;

impl Bmc{

    pub async fn insert(
        model: SignUpUser<HashedPassword>,
        dm: &impl DbPoolExtract<Postgres>
    ) -> QueryResult<Context> {

        let SignUpUser { 
            username, 
            first_name, 
            last_name, 
            password, 
            email, 
            phone_no, 
            location, 
            _phantom 
        } = model;
    
        let id: Context = sqlx::query_as!(
            Context,
            "INSERT INTO users(
                email, password, username,
                first_name, last_name, location,
                phone_no
            ) 
            VALUES ( 
                $1, $2, $3,
                $4, $5, $6,
                $7
            ) 
            RETURNING id, role as \"role:Role\"",
            email, password, username,
            first_name, last_name, location,
            phone_no
        )
        .fetch_one(dm.pool())
        .await
        .map_err(|e|{
            crate::DbError::FailedInsert{log: e.to_string()}
        })?;

        Ok(id)
    }

    pub async fn insert_with_role(
        model: AddUser<HashedPassword>,
        dm: &impl DbPoolExtract<Postgres>
    ) -> QueryResult<Context> {

        let AddUser { 
            username, 
            first_name, 
            last_name, 
            password, 
            email, 
            phone_no, 
            location, 
            role,
            _phantom 
        } = model;
    
        let id: Context = sqlx::query_as!(
            Context,
            "INSERT INTO users(
                email, password, username,
                first_name, last_name, location,
                phone_no, role
            ) 
            VALUES ( 
                $1, $2, $3,
                $4, $5, $6,
                $7, $8::user_role
            ) 
            RETURNING id, role as \"role:Role\"",
            email, password, username,
            first_name, last_name, location,
            phone_no, role as _
        )
        .fetch_one(dm.pool())
        .await
        .map_err(|e|{
            crate::DbError::FailedInsert{log: e.to_string()}
        })?;

        Ok(id)
    }


    pub async fn fetch_one_user(
        username: impl AsRef<str>,
        dm: &impl DbPoolExtract<Postgres>
    ) -> QueryResult<UserCredential> {

        let user = sqlx::query_as!(
            UserCredential,
            "SELECT 
                id, password, role as \"role:Role\"
            FROM users
            WHERE username=$1 OR email=$1",
            username.as_ref()
        ) 
        .fetch_one(dm.pool())
        .await
        .map_err(|e| DbError::FailedSelect { log: e.to_string() })?;

        Ok(user)
    }



    pub fn list(
        username: String,
        dm: &impl DbPoolExtract<Postgres>
    ) -> QueryResult<Vec<()>>{
        todo!()
    }

    pub fn update(

    ) -> QueryResult<i32>{
        todo!()
    }
}


// endregion: --- Users Bmc
