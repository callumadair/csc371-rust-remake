use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum WalletError {
    #[error("Unable to create new collection.")]
    CreationError,

    #[error("Unable to delete entry.")]
    DeletionError,

    #[error("Failure adding to collection.")]
    InsertionError,

    #[error("Error loading wallet from file.")]
    LoadError,

    #[error("Failure merging collections.")]
    MergeError,

    #[error("Failed to get data from collection.")]
    RetrievalError,

    #[error("Failure saving wallet.")]
    SaveError,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ArgsError {
    #[error("Invalid argument provided.")]
    InvalidAction,

    #[error("Invalid category argument.")]
    InvalidCategory,

    #[error("Invalid item argument.")]
    InvalidItem,

    #[error("Invalid entry argument.")]
    InvalidEntry,

    #[error("Missing argument.")]
    MissingAction,

    #[error("Missing category argument.")]
    MissingCategory,

    #[error("Missing item argument.")]
    MissingItem,

    #[error("Missing entry argument.")]
    MissingEntry,

    #[error("Missing parent object argument for children present.")]
    MissingParentObject,

    #[error("No category, item or entry argument provided.")]
    NoObjectsSpecified,
}
