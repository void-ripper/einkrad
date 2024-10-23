use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum PackageErrorKind {
    NotAPackage,
    Js,
}

#[derive(Debug)]
pub struct PackageError {
    pub kind: PackageErrorKind,
    pub msg: String,
}

impl PackageError {
    pub fn not_a_package() -> Self {
        Self {
            kind: PackageErrorKind::NotAPackage,
            msg: String::new(),
        }
    }
}

impl Error for PackageError {}

impl Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<rquickjs::Error> for PackageError {
    fn from(value: rquickjs::Error) -> Self {
        Self {
            kind: PackageErrorKind::Js,
            msg: value.to_string(),
        }
    }
}

impl<'js> From<rquickjs::CaughtError<'js>> for PackageError {
    fn from(value: rquickjs::CaughtError<'js>) -> Self {
        Self {
            kind: PackageErrorKind::Js,
            msg: value.to_string(),
        }
    }
}
