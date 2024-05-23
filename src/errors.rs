use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum LicenseError {
    #[error("not implemented")]
    NotImplemented,
    #[error("io error")]
    IOError(#[from] io::Error),
}