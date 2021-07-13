mod big_int_api_uncallable;
mod big_uint_api_uncallable;
mod elliptic_curve_api_uncallable;
mod error_api_uncallable;
mod send_api_uncallable;
mod storage_api_uncallable;

pub use big_int_api_uncallable::*;
pub use big_uint_api_uncallable::*;
pub use elliptic_curve_api_uncallable::*;
pub use error_api_uncallable::*;
pub use send_api_uncallable::*;
pub use storage_api_uncallable::*;

/// Dummy type with no implementation.
/// Provides context in ABI generators.
#[derive(Clone)]
pub struct UncallableApi;
