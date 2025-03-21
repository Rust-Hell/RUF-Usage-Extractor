# RUF-Usage-Extractor
This projects allows you to scan your crate's source and extrace the used unstable features based on your own building configuration.

## RUF-Scanner
The scanner is the main part of the project. It scans the source code of a Rust file and extracts the used unstable features. The scanner is based on `rustc interface`(nightly-2023-12-12 version) usage, and thus to use it, you first need to:
```bash
rustup toolchain install nightly-2023-12-12
rustup component add rustc-dev llvm-tools
```

The `ruf-scanner` accepts the same arguments as `rustc`, so it can bulid up the wanted configuration. The scanner will then scan the source code and extract the used unstable features. The scanner will output the used unstable features in a JSON format:
```bash
ruf_scanner test/single_test1.rs
Check info: CheckInfo { crate_name: "unknown", used_rufs: UsedRufs(["deprecated_safe"]), cfg: [] }
```

You can also access this functionality through the provided lib, just the same as `ruf_scanner` done:
```rust
#![feature(rustc_private)]

use ruf_extractor::run_scanner;
use std::{env, process::exit};

extern crate rustc_driver;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let exit_code = rustc_driver::catch_with_exit_code(|| {
        let check_info = match run_scanner(&args[1..]) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        println!("Check info: {:?}", check_info);

        Ok(())
    });

    exit(exit_code);
}
```
**NOTE**: Using the `rustc interface` need library support, and the link path may not be set by default. You may need to set the `LD_LIBRARY_PATH` mannualy, for example, to use the `ruf_scanner`, you first have to:
```bash
export LD_LIBRARY_PATH=/home/ubuntu/.rustup/toolchains/nightly-2023-12-12-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib
```
