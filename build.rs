use std::env;

extern crate clap;
use clap::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    println!("{:?}", outdir);
    let mut app = build_cli();
    app.gen_completions("scaffold-rs",
                        Shell::Bash,
                        "./target/release/");
}
