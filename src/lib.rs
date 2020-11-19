//! Crate for common bit operations. Mainly for
//! setting, toggling, unsetting and checking bits.
//!
//! These operations are provided by macros which accept
//! multiple patterns to allow easy bit fiddling.
//! These include operations on a single bit, range of bits, etc.
//! See macro docs for more details.
//!
//! Macros in this crate don't do overflow/underflow checks.
//! If invalid args are supplied, behaviour depends on the underlying
//! operators and may panic.
//!
//! # Example
//!
//! ```
//! use bit_fiddler::set;
//!
//! let mut bitmap: u8 = 0b_0000_0000;
//! set!(in bitmap, [3..6]);
//! assert_eq!(bitmap, 0b_0011_1000);
//! ```

mod bit_fiddle_macros;
