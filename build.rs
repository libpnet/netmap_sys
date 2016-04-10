// netmap doesn't provide these functions as a library, so we cheat, to save porting them manually
// to Rust. This is a very ugly hack.
extern crate gcc;
use std::env;
use std::io::prelude::*;
use std::fs;
use std::path::Path;

fn main() {
    if let Some(_) = env::var_os("CARGO_FEATURE_NETMAP_WITH_LIBS") {
        let out_dir = env::var("OUT_DIR").unwrap();
        let tmp_path = Path::new(&out_dir).join("netmap.c");
        let mut tmp = fs::File::create(&tmp_path).unwrap();

        tmp.write_all(b"#include <net/netmap_user.h>\n").unwrap();
        gcc::Config::new()
            .file(&tmp_path)
            .define("NETMAP_WITH_LIBS", None)
            .define("static", Some(""))
            .define("inline", Some(""))
            .include("netmap/sys")
            .compile("librust_netmap_user.a");
        fs::remove_file(&tmp_path).unwrap();
    }
}
