extern crate log;
extern crate simple_logger;

mod check;
mod prelude;
#[cfg(test)]
mod tests;

use prelude::*;

pub fn check(src: syn::File) {
    init_log();
}
