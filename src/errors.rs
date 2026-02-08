#![allow(unused)]

use ron::error::{Error as RonError, SpannedError as RonSpannedError};
use std::io::Error as IOError;

#[derive(Debug)]
pub enum DataStoreError {
  ReadError(ReadError),
  WriteError(WriteError),
  IOError(IOError),
  RonError(RonError),
  RonSpannedError(RonSpannedError),
}

#[derive(Debug)]
pub enum ReadError {
  EntryNotFound,
  DeserializationFailure,
}

#[derive(Debug)]
pub enum WriteError {
  SerializationFailure,
}
