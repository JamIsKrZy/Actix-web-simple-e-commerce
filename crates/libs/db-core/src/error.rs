

#[derive(Debug)]
pub enum DbError{
    FailedInsert{log: String},
    FailedSelect{log: String},
    
    External()
}