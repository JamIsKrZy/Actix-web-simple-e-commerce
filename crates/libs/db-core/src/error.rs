

#[derive(Debug)]
pub enum DbError{
    FailedInsert{log: String},
    FailedSelect{log: String},
    FailedDelete{log: String},
    FailedPatch {log: String},

    InitTransactionErr,
    TransactionCommitErr,
    External()
}
