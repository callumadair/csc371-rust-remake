use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Failure adding to collection.")]
    InsertionError,
    #[error("Unable to delete entry.")]
    DeletionError,
    #[error("Failed to get data from collection.")]
    RetrievalError,
    #[error("Failure merging collections.")]
    MergeError,
}
