extern crate ctest;

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    cfg.header("sys/time.h")
       .header("sys/ioctl.h")
       .header("net/if.h")
       .header("net/netmap.h")
       .header("net/netmap_user.h");

    cfg.type_name(|ty, is_struct| {
        if is_struct || ty == "timeval" {
            format!("struct {}", ty)
        } else {
            ty.to_string()
        }
    });

    cfg.include("netmap/sys");
    cfg.include("/usr/include");

    cfg.generate("../src/lib.rs", "all.rs");
}
