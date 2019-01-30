//! A Rust-native implementation of Hashids.
//!
//! Hashids provides a few benefits. For a start, identifiers are more
//! difficult to confuse, provided that your selected alphabet does not
//! include easily-confused symbols such as o, 0, O, 1, l, and I.
//! Additionally, it becomes much more difficult for malicious or curious
//! users to increment an identifier to "see what happens" when they throw
//! it at your API. Lastly, Hashids values may combine several identifiers
//! into a single value.
//!
//! > **NOTE:** Hashids values are **not cryptographically secure.**
//! Regardless of the quality of your salt, this algorithm is fairly easy to
//! crack.
//!
//! Hashids should not be used for security purposes, but for your own
//! convenience.
//!
//! ## Creating and encoding
//!
//! [`Harsh`](./harsh/struct.Harsh.html) lacks a constructor (other than the
//! default constructor, which should not be used), and should be created
//! by the use of [`HarshBuilder`](./harsh/struct.HarshBuilder.html), which
//! allows for configuration with salts, alphabets, separators, and so forth.
//!
//! Initialization ensures that appropriate values have been provided for the
//! salt, alphabet, separators, and so forth, and in the event a struct cannot
//! be created in a usable state, an error will be returned.
//!
//! Encoding assumes zero or more input values and will return `None` in the
//! even that zero inputs have been provided.
//!
//! ```rust
//! # use harsh::{HarshBuilder, Result};
//! # fn main() -> Result<()> {
//! let harsh = HarshBuilder::new().salt("salt goes here!").init()?;
//!
//! let encoded = harsh.encode(&[1, 2, 3, 4, 5])
//!     .expect("Sorry, you have to pass in a value or two.");
//!
//! assert_eq!("xrUQTnhgu7", encoded);
//! # Ok(())
//! # }
//! ```
//!
//! ## Decoding
//!
//! Decoding likewise will return zero or more values in the form of a vector,
//! but may also return `None` in the event that the decoded value is not a
//! valid Hashid.
//!
//! ```rust
//! # use harsh::{HarshBuilder, Result};
//! # fn main() -> Result<()> {
//! # let harsh = HarshBuilder::new().salt("salt goes here!").init()?;
//! # let encoded = harsh.encode(&[1, 2, 3, 4, 5]).unwrap();
//! let decoded = harsh.decode(&encoded)
//!     .expect("This better work...");
//!
//! assert_eq!(&[1, 2, 3, 4, 5], &*decoded);
//! # Ok(())
//! # }
//! ```

#[cfg(test)]
mod tests;

mod error;
mod harsh;

pub use crate::{
    error::{Error, Result},
    harsh::{Harsh, HarshBuilder},
};
