pub use crate::error::*;
pub use log::trace;
pub use std::fs::read_to_string as read_file;
pub use std::path::PathBuf;
pub use syn::{File, Item};

macro_rules! bail {
    ($err:expr) => {
        return Err($err.into())
    };
}

pub(crate) use bail;
