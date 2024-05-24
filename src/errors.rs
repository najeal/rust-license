use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LicenseError {
    #[error("not implemented")]
    NotImplemented,
    #[error("io error")]
    IOError(#[from] io::Error),
}
