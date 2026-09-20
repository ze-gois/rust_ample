# ample

[![crates.io](https://img.shields.io/crates/v/ample.svg)](https://crates.io/crates/ample)
[![docs.rs](https://docs.rs/ample/badge.svg)](https://docs.rs/ample)

Small `no_std` primitives shared by the [userspace.party](https://userspace.party) ecosystem.

## Role

`ample` sits near the bottom of the dependency graph. It provides reusable building blocks that the higher-level crates can share without pulling in the Rust standard library.

The current surface includes:

- `Vec` and `String` implementations exported by the crate;
- list and node primitives;
- traits and result/error types;
- macros used across the family;
- math support suitable for `no_std` code.

`libm` is currently the intentional external runtime dependency.

## Use

```bash
cargo add ample
```

Or in `Cargo.toml`:

```toml
[dependencies]
ample = "0.2"
```

The crate is `#![no_std]` and uses `alloc`.

## Ecosystem

`ample` is developed independently but released as part of the coordinated `userspace_hub` publication unit.

- Ecosystem: https://userspace.party
- Crate homepage: https://userspace.party/ample
- API documentation: https://docs.rs/ample
- crates.io: https://crates.io/crates/ample
- Source: https://github.com/ze-gois/rust_ample
- Workspace hub: https://github.com/ze-gois/rust_userspace_hub

## Status

Experimental. The crate is actively being reduced toward a small set of primitives owned by the ecosystem rather than delegated to broad third-party dependency trees.

## License

See [LICENSE](LICENSE).
