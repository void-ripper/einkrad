use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum PackageErrorKind {
    NotAPackage,
    Lua,
    Io,
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

impl From<mlua::Error> for PackageError {
    fn from(value: mlua::Error) -> Self {
        Self {
            kind: PackageErrorKind::Lua,
            msg: value.to_string(),
        }
    }
}

impl From<std::io::Error> for PackageError {
    fn from(value: std::io::Error) -> Self {
        Self {
            kind: PackageErrorKind::Io,
            msg: value.to_string(),
        }
    }
}
