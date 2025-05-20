use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataPlaneControllerError {
    #[error("NotCheckedError in Data plane controller. {0}")]
    NotCheckedError(String),
}
