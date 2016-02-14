use std::error::Error;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::convert::From;

/**
 * Kind of error
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoteErrorKind {
    // Nothing here yet
}

fn note_error_type_as_str(e: &NoteErrorKind) -> &'static str {
    match e {
        _ => "",
    }
}

impl Display for NoteErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", note_error_type_as_str(self)));
        Ok(())
    }

}

/**
 * Store error type
 */
#[derive(Debug)]
pub struct NoteError {
    err_type: NoteErrorKind,
    cause: Option<Box<Error>>,
}

impl NoteError {

    /**
     * Build a new NoteError from an NoteErrorKind, optionally with cause
     */
    pub fn new(errtype: NoteErrorKind, cause: Option<Box<Error>>) -> NoteError {
        NoteError {
            err_type: errtype,
            cause: cause,
        }
    }

    /**
     * Get the error type of this NoteError
     */
    pub fn err_type(&self) -> NoteErrorKind {
        self.err_type.clone()
    }

}

impl Display for NoteError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", note_error_type_as_str(&self.err_type.clone())));
        Ok(())
    }

}

impl Error for NoteError {

    fn description(&self) -> &str {
        note_error_type_as_str(&self.err_type.clone())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

