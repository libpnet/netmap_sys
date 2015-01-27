// netmap doesn't provide these functions as a library, so we cheat, to save porting them manually
// to Rust. This is a very ugly hack.
#![allow(unstable)]
use std::io::Command;
use std::os;

fn main() {
    let out_dir = os::getenv("OUT_DIR").unwrap();

    if let Some(_) = os::getenv("CARGO_FEATURE_NETMAP_WITH_LIBS") {
        Command::new("clang").args(&["-DNETMAP_WITH_LIBS", "-Dstatic=", "-Dinline=",
                                     "-x", "c",
                                     "-fPIC",
                                     "-O2",
                                     "-c", "/usr/include/net/netmap_user.h",
                                     "-o"])
                           .arg(format!("{}/netmap_user.o", out_dir))
                           .status().unwrap();
        Command::new("ar").args(&["crus"])
                          .arg(format!("{}/librust_netmap_user.a", out_dir))
                          .arg(format!("{}/netmap_user.o", out_dir))
                          .status().unwrap();

        println!("cargo:rustc-flags=-L native={} -l static=rust_netmap_user", out_dir);
    }
}
