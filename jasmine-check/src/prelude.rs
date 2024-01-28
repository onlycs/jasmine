pub use log::{debug, error, info, trace, warn};
pub use syn::*;

pub fn init_log() {
    simple_logger::init().unwrap();
}
