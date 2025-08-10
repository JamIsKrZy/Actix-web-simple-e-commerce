

#[derive(Debug)]
pub enum Error{
    FailedInsert{log: String},
    FailedSelect{log: String},
    
    External()
}