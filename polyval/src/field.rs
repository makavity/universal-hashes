//! Implementation of POLYVAL's finite field.
//!
//! From [RFC 8452 Section 3] which defines POLYVAL for use in AES-GCM_SIV:
//!
//! > "POLYVAL, like GHASH (the authenticator in AES-GCM; ...), operates in a
//! > binary field of size 2^128.  The field is defined by the irreducible
//! > polynomial x^128 + x^127 + x^126 + x^121 + 1."
//!
//! This implementation provides multiplication over GF(2^128) optimized using
//! Shay Gueron's PCLMULQDQ-based techniques.
//!
//! For more information on how these techniques work, see:
//! <https://blog.quarkslab.com/reversing-a-finite-field-multiplication-optimization.html>
//!
//! [RFC 8452 Section 3]: https://tools.ietf.org/html/rfc8452#section-3

#[cfg(all(
    target_feature = "pclmulqdq",
    target_feature = "sse2",
    target_feature = "sse4.1",
    any(target_arch = "x86", target_arch = "x86_64")
))]
mod pclmulqdq;
mod soft;

use core::ops::{Add, Mul};

#[cfg(all(
    target_feature = "pclmulqdq",
    target_feature = "sse2",
    target_feature = "sse4.1",
    any(target_arch = "x86", target_arch = "x86_64")
))]
use self::pclmulqdq::M128i;

#[allow(unused_imports)]
use self::soft::U64x2;

#[cfg(not(all(
    target_feature = "pclmulqdq",
    target_feature = "sse2",
    target_feature = "sse4.1",
    any(target_arch = "x86", target_arch = "x86_64")
)))]
type M128i = U64x2;

/// Size of GF(2^128) in bytes (16-bytes).
pub const FIELD_SIZE: usize = 16;

/// POLYVAL field element bytestrings (16-bytes)
pub type Block = [u8; FIELD_SIZE];

/// POLYVAL field element.
#[derive(Copy, Clone)]
pub struct Element(M128i);

impl Element {
    /// Load a `FieldElement` from its bytestring representation.
    pub fn from_bytes(bytes: Block) -> Self {
        Element(bytes.into())
    }

    /// Serialize this `FieldElement` as a bytestring.
    pub fn to_bytes(self) -> Block {
        self.0.into()
    }
}

impl Default for Element {
    fn default() -> Self {
        Self::from_bytes(Block::default())
    }
}

impl Add for Element {
    type Output = Self;

    /// Adds two POLYVAL field elements.
    ///
    /// From [RFC 8452 Section 3]:
    ///
    /// > "The sum of any two elements in the field is the result of XORing them."
    ///
    /// [RFC 8452 Section 3]: https://tools.ietf.org/html/rfc8452#section-3
    fn add(self, rhs: Self) -> Self {
        Element(self.0 + rhs.0)
    }
}

impl Mul for Element {
    type Output = Self;

    /// Computes POLYVAL multiplication over GF(2^128).
    ///
    /// From [RFC 8452 Section 3]:
    ///
    /// > "The product of any two elements is calculated using standard
    /// > (binary) polynomial multiplication followed by reduction modulo the
    /// > irreducible polynomial."
    ///
    /// [RFC 8452 Section 3]: https://tools.ietf.org/html/rfc8452#section-3
    fn mul(self, rhs: Self) -> Self {
        Element(self.0 * rhs.0)
    }
}
