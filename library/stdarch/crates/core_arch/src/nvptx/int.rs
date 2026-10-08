// --- LLM-generated --- //
//! Integer arithmetic and bit manipulation instructions.

use super::modifiers::ClampMode;
use crate::marker::ConstParamTy;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.mulhi.s"]
    fn llvm_mulhi_s(a: i16, b: i16) -> i16;
    #[link_name = "llvm.nvvm.mulhi.us"]
    fn llvm_mulhi_us(a: u16, b: u16) -> u16;
    #[link_name = "llvm.nvvm.mulhi.i"]
    fn llvm_mulhi_i(a: i32, b: i32) -> i32;
    #[link_name = "llvm.nvvm.mulhi.ui"]
    fn llvm_mulhi_ui(a: u32, b: u32) -> u32;
    #[link_name = "llvm.nvvm.mulhi.ll"]
    fn llvm_mulhi_ll(a: i64, b: i64) -> i64;
    #[link_name = "llvm.nvvm.mulhi.ull"]
    fn llvm_mulhi_ull(a: u64, b: u64) -> u64;
    #[link_name = "llvm.nvvm.mul24.i"]
    fn llvm_mul24_i(a: i32, b: i32) -> i32;
    #[link_name = "llvm.nvvm.mul24.ui"]
    fn llvm_mul24_ui(a: u32, b: u32) -> u32;
    #[link_name = "llvm.nvvm.sad.s"]
    fn llvm_sad_s(a: i16, b: i16, c: i16) -> i16;
    #[link_name = "llvm.nvvm.sad.us"]
    fn llvm_sad_us(a: u16, b: u16, c: u16) -> u16;
    #[link_name = "llvm.nvvm.sad.i"]
    fn llvm_sad_i(a: i32, b: i32, c: i32) -> i32;
    #[link_name = "llvm.nvvm.sad.ui"]
    fn llvm_sad_ui(a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.sad.ll"]
    fn llvm_sad_ll(a: i64, b: i64, c: i64) -> i64;
    #[link_name = "llvm.nvvm.sad.ull"]
    fn llvm_sad_ull(a: u64, b: u64, c: u64) -> u64;
    #[link_name = "llvm.nvvm.idp4a.u.u"]
    fn llvm_idp4a_u_u(a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.idp4a.u.s"]
    fn llvm_idp4a_u_s(a: u32, b: i32, c: i32) -> i32;
    #[link_name = "llvm.nvvm.idp4a.s.u"]
    fn llvm_idp4a_s_u(a: i32, b: u32, c: i32) -> i32;
    #[link_name = "llvm.nvvm.idp4a.s.s"]
    fn llvm_idp4a_s_s(a: i32, b: i32, c: i32) -> i32;
    #[link_name = "llvm.nvvm.idp2a.u.u"]
    fn llvm_idp2a_u_u(a: u32, b: u32, hi: bool, c: u32) -> u32;
    #[link_name = "llvm.nvvm.idp2a.u.s"]
    fn llvm_idp2a_u_s(a: u32, b: i32, hi: bool, c: i32) -> i32;
    #[link_name = "llvm.nvvm.idp2a.s.u"]
    fn llvm_idp2a_s_u(a: i32, b: u32, hi: bool, c: i32) -> i32;
    #[link_name = "llvm.nvvm.idp2a.s.s"]
    fn llvm_idp2a_s_s(a: i32, b: i32, hi: bool, c: i32) -> i32;
    #[link_name = "llvm.nvvm.fshl.clamp"]
    fn llvm_fshl_clamp(a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.fshl.i32"]
    fn llvm_fshl_wrap(a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.fshr.clamp"]
    fn llvm_fshr_clamp(a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.fshr.i32"]
    fn llvm_fshr_wrap(a: u32, b: u32, c: u32) -> u32;
    #[link_name = "llvm.nvvm.bmsk.clamp"]
    fn llvm_bmsk_clamp(a: u32, b: u32) -> u32;
    #[link_name = "llvm.nvvm.bmsk.wrap"]
    fn llvm_bmsk_wrap(a: u32, b: u32) -> u32;
    #[link_name = "llvm.nvvm.sext.clamp"]
    fn llvm_sext_clamp(a: i32, b: u32) -> i32;
    #[link_name = "llvm.nvvm.sext.wrap"]
    fn llvm_sext_wrap(a: i32, b: u32) -> i32;
    #[link_name = "llvm.nvvm.zext.clamp"]
    fn llvm_zext_clamp(a: u32, b: u32) -> u32;
    #[link_name = "llvm.nvvm.zext.wrap"]
    fn llvm_zext_wrap(a: u32, b: u32) -> u32;
    #[link_name = "llvm.nvvm.fns"]
    fn llvm_fns(mask: u32, base: u32, offset: i32) -> u32;
    #[link_name = "llvm.nvvm.flo.u.i32"]
    fn llvm_flo_u_i32(a: u32, shiftamt: bool) -> u32;
    #[link_name = "llvm.nvvm.flo.u.i64"]
    fn llvm_flo_u_i64(a: u64, shiftamt: bool) -> u32;
    #[link_name = "llvm.nvvm.flo.s.i32"]
    fn llvm_flo_s_i32(a: i32, shiftamt: bool) -> u32;
    #[link_name = "llvm.nvvm.flo.s.i64"]
    fn llvm_flo_s_i64(a: i64, shiftamt: bool) -> u32;
}

/// Selects the half of the packed bytes used by the `dp2a` instructions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ConstParamTy)]
#[non_exhaustive]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub enum Dp2aMode {
    /// Use the lower two bytes of `b` (`.lo`).
    Lo,
    /// Use the upper two bytes of `b` (`.hi`).
    Hi,
}

/// Returns the high 16 bits of the 32-bit product of two signed 16-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-mul>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul_hi_s16(a: i16, b: i16) -> i16 {
    llvm_mulhi_s(a, b)
}

/// Returns the high 16 bits of the 32-bit product of two unsigned 16-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-mul>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul_hi_u16(a: u16, b: u16) -> u16 {
    llvm_mulhi_us(a, b)
}

/// Returns the high 32 bits of the 64-bit product of two signed 32-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-mul>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul_hi_s32(a: i32, b: i32) -> i32 {
    llvm_mulhi_i(a, b)
}

/// Returns the high 32 bits of the 64-bit product of two unsigned 32-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-mul>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul_hi_u32(a: u32, b: u32) -> u32 {
    llvm_mulhi_ui(a, b)
}

/// Returns the high 64 bits of the 128-bit product of two signed 64-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-mul>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul_hi_s64(a: i64, b: i64) -> i64 {
    llvm_mulhi_ll(a, b)
}

/// Returns the high 64 bits of the 128-bit product of two unsigned 64-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-mul>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul_hi_u64(a: u64, b: u64) -> u64 {
    llvm_mulhi_ull(a, b)
}

/// Returns the low 32 bits of the 48-bit product of the low 24 bits of two signed integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-mul24>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul24_lo_s32(a: i32, b: i32) -> i32 {
    llvm_mul24_i(a, b)
}

/// Returns the low 32 bits of the 48-bit product of the low 24 bits of two unsigned integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-mul24>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul24_lo_u32(a: u32, b: u32) -> u32 {
    llvm_mul24_ui(a, b)
}

/// Computes `|a - b| + c` for signed 16-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-sad>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sad_s16(a: i16, b: i16, c: i16) -> i16 {
    llvm_sad_s(a, b, c)
}

/// Computes `|a - b| + c` for unsigned 16-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-sad>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sad_u16(a: u16, b: u16, c: u16) -> u16 {
    llvm_sad_us(a, b, c)
}

/// Computes `|a - b| + c` for signed 32-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-sad>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sad_s32(a: i32, b: i32, c: i32) -> i32 {
    llvm_sad_i(a, b, c)
}

/// Computes `|a - b| + c` for unsigned 32-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-sad>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sad_u32(a: u32, b: u32, c: u32) -> u32 {
    llvm_sad_ui(a, b, c)
}

/// Computes `|a - b| + c` for signed 64-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-sad>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sad_s64(a: i64, b: i64, c: i64) -> i64 {
    llvm_sad_ll(a, b, c)
}

/// Computes `|a - b| + c` for unsigned 64-bit integers.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-sad>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sad_u64(a: u64, b: u64, c: u64) -> u64 {
    llvm_sad_ull(a, b, c)
}

/// Four-way byte dot product of unsigned `a` and `b`, accumulated into unsigned `c`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-dp4a>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn dp4a_u32_u32(a: u32, b: u32, c: u32) -> u32 {
    llvm_idp4a_u_u(a, b, c)
}

/// Four-way byte dot product of unsigned `a` and signed `b`, accumulated into signed `c`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-dp4a>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn dp4a_u32_s32(a: u32, b: i32, c: i32) -> i32 {
    llvm_idp4a_u_s(a, b, c)
}

/// Four-way byte dot product of signed `a` and unsigned `b`, accumulated into signed `c`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-dp4a>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn dp4a_s32_u32(a: i32, b: u32, c: i32) -> i32 {
    llvm_idp4a_s_u(a, b, c)
}

/// Four-way byte dot product of signed `a` and `b`, accumulated into signed `c`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-dp4a>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn dp4a_s32_s32(a: i32, b: i32, c: i32) -> i32 {
    llvm_idp4a_s_s(a, b, c)
}

/// Two-way dot product of unsigned 16-bit pairs in `a` with the selected bytes of unsigned
/// `b`, accumulated into unsigned `c`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-dp2a>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn dp2a_u32_u32<const MODE: Dp2aMode>(a: u32, b: u32, c: u32) -> u32 {
    match MODE {
        Dp2aMode::Lo => llvm_idp2a_u_u(a, b, false, c),
        Dp2aMode::Hi => llvm_idp2a_u_u(a, b, true, c),
    }
}

/// Two-way dot product of unsigned 16-bit pairs in `a` with the selected bytes of signed `b`,
/// accumulated into signed `c`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-dp2a>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn dp2a_u32_s32<const MODE: Dp2aMode>(a: u32, b: i32, c: i32) -> i32 {
    match MODE {
        Dp2aMode::Lo => llvm_idp2a_u_s(a, b, false, c),
        Dp2aMode::Hi => llvm_idp2a_u_s(a, b, true, c),
    }
}

/// Two-way dot product of signed 16-bit pairs in `a` with the selected bytes of unsigned `b`,
/// accumulated into signed `c`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-dp2a>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn dp2a_s32_u32<const MODE: Dp2aMode>(a: i32, b: u32, c: i32) -> i32 {
    match MODE {
        Dp2aMode::Lo => llvm_idp2a_s_u(a, b, false, c),
        Dp2aMode::Hi => llvm_idp2a_s_u(a, b, true, c),
    }
}

/// Two-way dot product of signed 16-bit pairs in `a` with the selected bytes of signed `b`,
/// accumulated into signed `c`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-dp2a>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn dp2a_s32_s32<const MODE: Dp2aMode>(a: i32, b: i32, c: i32) -> i32 {
    match MODE {
        Dp2aMode::Lo => llvm_idp2a_s_s(a, b, false, c),
        Dp2aMode::Hi => llvm_idp2a_s_s(a, b, true, c),
    }
}

/// Funnel shift left: shifts the 64-bit value `{b, a}` left by `c` and returns the upper 32 bits.
///
/// In [`ClampMode::Clamp`] mode the shift amount is clamped to `0..=32`; in [`ClampMode::Wrap`]
/// mode it is taken modulo 32.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#logic-and-shift-instructions-shf>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn shf_l<const MODE: ClampMode>(a: u32, b: u32, c: u32) -> u32 {
    match MODE {
        ClampMode::Clamp => llvm_fshl_clamp(b, a, c),
        ClampMode::Wrap => llvm_fshl_wrap(b, a, c),
    }
}

/// Funnel shift right: shifts the 64-bit value `{b, a}` right by `c` and returns the lower 32 bits.
///
/// In [`ClampMode::Clamp`] mode the shift amount is clamped to `0..=32`; in [`ClampMode::Wrap`]
/// mode it is taken modulo 32.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#logic-and-shift-instructions-shf>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn shf_r<const MODE: ClampMode>(a: u32, b: u32, c: u32) -> u32 {
    match MODE {
        ClampMode::Clamp => llvm_fshr_clamp(b, a, c),
        ClampMode::Wrap => llvm_fshr_wrap(b, a, c),
    }
}

/// Generates a 32-bit mask of width `b` starting at bit position `a`.
///
/// Requires PTX ISA 7.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-bmsk>
#[inline]
#[target_feature(enable = "ptx76")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn bmsk<const MODE: ClampMode>(a: u32, b: u32) -> u32 {
    match MODE {
        ClampMode::Clamp => llvm_bmsk_clamp(a, b),
        ClampMode::Wrap => llvm_bmsk_wrap(a, b),
    }
}

/// Sign-extends the low `b` bits of `a`.
///
/// Requires PTX ISA 7.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-szext>
#[inline]
#[target_feature(enable = "ptx76")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn szext_s32<const MODE: ClampMode>(a: i32, b: u32) -> i32 {
    match MODE {
        ClampMode::Clamp => llvm_sext_clamp(a, b),
        ClampMode::Wrap => llvm_sext_wrap(a, b),
    }
}

/// Zero-extends the low `b` bits of `a`.
///
/// Requires PTX ISA 7.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-szext>
#[inline]
#[target_feature(enable = "ptx76")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn szext_u32<const MODE: ClampMode>(a: u32, b: u32) -> u32 {
    match MODE {
        ClampMode::Clamp => llvm_zext_clamp(a, b),
        ClampMode::Wrap => llvm_zext_wrap(a, b),
    }
}

/// Finds the position of the `offset`-th set bit of `mask`, searching from bit `base`.
///
/// Returns `0xffffffff` if no such bit exists.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-fns>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fns(mask: u32, base: u32, offset: i32) -> u32 {
    llvm_fns(mask, base, offset)
}

/// Returns the bit position of the most significant non-sign bit of an unsigned 32-bit integer.
///
/// If `SHIFTAMT` is set, returns the left shift that moves that bit to the most significant
/// position instead. Returns `0xffffffff` if no bit is found.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-bfind>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn bfind_u32<const SHIFTAMT: bool>(a: u32) -> u32 {
    llvm_flo_u_i32(a, SHIFTAMT)
}

/// Returns the bit position of the most significant non-sign bit of a signed 32-bit integer.
///
/// If `SHIFTAMT` is set, returns the left shift that moves that bit to the most significant
/// position instead. Returns `0xffffffff` if no bit is found.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-bfind>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn bfind_s32<const SHIFTAMT: bool>(a: i32) -> u32 {
    llvm_flo_s_i32(a, SHIFTAMT)
}

/// Returns the bit position of the most significant non-sign bit of an unsigned 64-bit integer.
///
/// If `SHIFTAMT` is set, returns the left shift that moves that bit to the most significant
/// position instead. Returns `0xffffffff` if no bit is found.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-bfind>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn bfind_u64<const SHIFTAMT: bool>(a: u64) -> u32 {
    llvm_flo_u_i64(a, SHIFTAMT)
}

/// Returns the bit position of the most significant non-sign bit of a signed 64-bit integer.
///
/// If `SHIFTAMT` is set, returns the left shift that moves that bit to the most significant
/// position instead. Returns `0xffffffff` if no bit is found.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-bfind>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn bfind_s64<const SHIFTAMT: bool>(a: i64) -> u32 {
    llvm_flo_s_i64(a, SHIFTAMT)
}
