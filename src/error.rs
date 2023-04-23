use thiserror::Error;

use crate::lock::Input;

#[derive(Debug, Error)]
#[error("index out of bounds: {0}")]
pub(crate) struct IndexOutOfBounds(pub usize);

#[derive(Debug, Error)]
#[error("invalid input: {0:?}")]
pub(crate) struct InvalidInput(pub Input);
