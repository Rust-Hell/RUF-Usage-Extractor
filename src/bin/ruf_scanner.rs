#![feature(rustc_private)]

use ruf_extractor::run_scanner;
use std::{env, process::exit};

extern crate rustc_driver;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let exit_code = rustc_driver::catch_with_exit_code(|| {
        let check_info = match run_scanner(&args) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        println!("Check info: {:?}", check_info);

        Ok(())
    });

    exit(exit_code);
}
