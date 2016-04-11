# netmap_sys

Rust bindings to [netmap](http://info.iet.unipi.it/~luigi/netmap/), currently
at version 3.17 (Linux).

## Usage

To use within your own project, add:

```
[dependencies.netmap_sys]
version = "0.1.0"
# Uncomment this line where you wish to use features guarded by the
# NETMAP_WITH_LIBS macro in C.
#features = "netmap_with_libs"
```

To your Cargo.toml.

## Troubleshooting

### missing rust_netmap_user

If you get an error containing the message:

```
error: could not find native static library `rust_netmap_user`, perhaps an -L flag is missing?
```

You should check the following things:

 1. You have clang installed
    * If you do not have clang installed, and do not wish to, you can modify
      `build.rs` to use gcc instead (untested, but it should work).
 2. Make sure that `/usr/include/net/netmap.h` and
    `/usr/include/net/netmap_user.h` both exist. If they do not, you should
    check your netmap installation. You can either manually add these files or
    symlinks to the, or change the paths searched in `build.rs`.
 3. If you still have issues, please file an issue in the bug tracker, along
 with the output of `cargo build -v`, your operating system and distribution,
 how you installed netmap, and the output of `clang -DNETMAP_WITH_LIBS
 -Dstatic= -Dinline= -x c -fPIC -O2 -c /usr/include/net/netmap_user.h -o
 $(mktemp)`.
