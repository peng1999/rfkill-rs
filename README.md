# rfkill-rs

A Rust wrapper around rfkill mechanism.

## Cross-compilation

This crate does not link to any libraries, but should use the `<linux/rfkill.h>` header of the target platform. Specify following environment variables during cross compilation:

```
BINDGEN_EXTRA_CLANG_ARGS='--sysroot /path/to/target/sysroot'
```