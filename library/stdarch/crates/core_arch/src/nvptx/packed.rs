//! NVPTX Packed data types (SIMD)
//!
//! Packed Data Types is what PTX calls SIMD types. See [PTX ISA (Packed Data Types)](https://docs.nvidia.com/cuda/parallel-thread-execution/#packed-data-types) for a full reference.

types! {
    #![unstable(feature = "stdarch_nvptx", issue = "111199")]

    /// PTX-specific 32-bit wide floating point (f16 x 2) vector type
    pub struct f16x2(2 x f16);

    /// PTX-specific 32-bit wide floating point (bf16 x 2) vector type, stored as the raw
    /// bits of each [`bf16`] element.
    pub struct bf16x2(2 x u16);
}

/// The bfloat16 floating point type.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub struct bf16(u16);

impl bf16 {
    /// Raw transmutation from `u16`
    #[inline]
    #[must_use]
    #[unstable(feature = "stdarch_nvptx", issue = "111199")]
    pub const fn from_bits(bits: u16) -> bf16 {
        bf16(bits)
    }

    /// Raw transmutation to `u16`
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[unstable(feature = "stdarch_nvptx", issue = "111199")]
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
