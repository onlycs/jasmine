#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;

use std::{env, fs, path::PathBuf};

use rustc_driver::Callbacks;
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;

enum JasmineCallbacks {
    Compile { jasmine_out: PathBuf },
    Metadata,
}

impl Callbacks for JasmineCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        match self {
            JasmineCallbacks::Compile { jasmine_out } => {
                jasmine_lower::run(tcx, &jasmine_out);
                rustc_driver::Compilation::Stop
            }
            JasmineCallbacks::Metadata => rustc_driver::Compilation::Continue,
        }
    }
}

fn cargo_target_dir(args: &[String]) -> Option<PathBuf> {
    let mut rustc_out_dir = args
        .iter()
        .position(|a| a == "--out-dir")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)?;

    while rustc_out_dir.file_name()? != "target" {
        rustc_out_dir.pop();
    }

    Some(rustc_out_dir)
}

fn main() {
    let mut argv: Vec<String> = env::args().collect();

    if argv.get(1).map(|s| s.trim() == "rustc").unwrap_or(false) {
        argv.remove(1);
    }

    let mut callbacks = match cargo_target_dir(&argv) {
        Some(target) => {
            let jasmine_out = target.join("jasmine");
            fs::create_dir_all(&jasmine_out).unwrap();
            JasmineCallbacks::Compile { jasmine_out }
        }
        None => JasmineCallbacks::Metadata,
    };

    rustc_driver::install_ice_hook("https://codeberg.org/tendulkar/jasmine/issues", |_| ());
    rustc_driver::run_compiler(&argv, &mut callbacks);
}
