//! Mathematical primitives exposed by `ample`.
//!
//! `libm` is the current implementation source. Consumers depend on
//! `ample::math`, so these symbols can later be replaced incrementally by
//! native `ample` implementations without changing their import surface.

pub use libm::*;
