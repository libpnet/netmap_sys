# rust-netmap

Rust bindings to [netmap](http://info.iet.unipi.it/~luigi/netmap/), currently
at version 3.17 (Linux).

## Usage

To use within your own project, add:

```
[dependencies.rust-netmap]
git = "https://github.com/libpnet/rust-netmap.git"
# Uncomment this line where you wish to use features guarded by the
# NETMAP_WITH_LIBS macro in C.
#features = "netmap_with_libs"
```

To your Cargo.toml.
