
pub mod user;
pub mod product;
pub mod bundles;


pub(in crate::models) type QueryResult<T> = Result<T, crate::DbError>;

pub struct Pagination