// --- LLM-generated --- //
//! Conversions between integer and floating-point types, and rounding to integral values.

use super::modifiers::Rounding;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.f2i.rn"]
    fn llvm_f2i_rn(a: f32) -> i32;
    #[link_name = "llvm.nvvm.f2i.rn.ftz"]
    fn llvm_f2i_rn_ftz(a: f32) -> i32;
    #[link_name = "llvm.nvvm.f2i.rz"]
    fn llvm_f2i_rz(a: f32) -> i32;
    #[link_name = "llvm.nvvm.f2i.rz.ftz"]
    fn llvm_f2i_rz_ftz(a: f32) -> i32;
    #[link_name = "llvm.nvvm.f2i.rm"]
    fn llvm_f2i_rm(a: f32) -> i32;
    #[link_name = "llvm.nvvm.f2i.rm.ftz"]
    fn llvm_f2i_rm_ftz(a: f32) -> i32;
    #[link_name = "llvm.nvvm.f2i.rp"]
    fn llvm_f2i_rp(a: f32) -> i32;
    #[link_name = "llvm.nvvm.f2i.rp.ftz"]
    fn llvm_f2i_rp_ftz(a: f32) -> i32;
    #[link_name = "llvm.nvvm.f2ui.rn"]
    fn llvm_f2ui_rn(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2ui.rn.ftz"]
    fn llvm_f2ui_rn_ftz(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2ui.rz"]
    fn llvm_f2ui_rz(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2ui.rz.ftz"]
    fn llvm_f2ui_rz_ftz(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2ui.rm"]
    fn llvm_f2ui_rm(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2ui.rm.ftz"]
    fn llvm_f2ui_rm_ftz(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2ui.rp"]
    fn llvm_f2ui_rp(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2ui.rp.ftz"]
    fn llvm_f2ui_rp_ftz(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2ll.rn"]
    fn llvm_f2ll_rn(a: f32) -> i64;
    #[link_name = "llvm.nvvm.f2ll.rn.ftz"]
    fn llvm_f2ll_rn_ftz(a: f32) -> i64;
    #[link_name = "llvm.nvvm.f2ll.rz"]
    fn llvm_f2ll_rz(a: f32) -> i64;
    #[link_name = "llvm.nvvm.f2ll.rz.ftz"]
    fn llvm_f2ll_rz_ftz(a: f32) -> i64;
    #[link_name = "llvm.nvvm.f2ll.rm"]
    fn llvm_f2ll_rm(a: f32) -> i64;
    #[link_name = "llvm.nvvm.f2ll.rm.ftz"]
    fn llvm_f2ll_rm_ftz(a: f32) -> i64;
    #[link_name = "llvm.nvvm.f2ll.rp"]
    fn llvm_f2ll_rp(a: f32) -> i64;
    #[link_name = "llvm.nvvm.f2ll.rp.ftz"]
    fn llvm_f2ll_rp_ftz(a: f32) -> i64;
    #[link_name = "llvm.nvvm.f2ull.rn"]
    fn llvm_f2ull_rn(a: f32) -> u64;
    #[link_name = "llvm.nvvm.f2ull.rn.ftz"]
    fn llvm_f2ull_rn_ftz(a: f32) -> u64;
    #[link_name = "llvm.nvvm.f2ull.rz"]
    fn llvm_f2ull_rz(a: f32) -> u64;
    #[link_name = "llvm.nvvm.f2ull.rz.ftz"]
    fn llvm_f2ull_rz_ftz(a: f32) -> u64;
    #[link_name = "llvm.nvvm.f2ull.rm"]
    fn llvm_f2ull_rm(a: f32) -> u64;
    #[link_name = "llvm.nvvm.f2ull.rm.ftz"]
    fn llvm_f2ull_rm_ftz(a: f32) -> u64;
    #[link_name = "llvm.nvvm.f2ull.rp"]
    fn llvm_f2ull_rp(a: f32) -> u64;
    #[link_name = "llvm.nvvm.f2ull.rp.ftz"]
    fn llvm_f2ull_rp_ftz(a: f32) -> u64;
    #[link_name = "llvm.nvvm.d2i.rn"]
    fn llvm_d2i_rn(a: f64) -> i32;
    #[link_name = "llvm.nvvm.d2i.rz"]
    fn llvm_d2i_rz(a: f64) -> i32;
    #[link_name = "llvm.nvvm.d2i.rm"]
    fn llvm_d2i_rm(a: f64) -> i32;
    #[link_name = "llvm.nvvm.d2i.rp"]
    fn llvm_d2i_rp(a: f64) -> i32;
    #[link_name = "llvm.nvvm.d2ui.rn"]
    fn llvm_d2ui_rn(a: f64) -> u32;
    #[link_name = "llvm.nvvm.d2ui.rz"]
    fn llvm_d2ui_rz(a: f64) -> u32;
    #[link_name = "llvm.nvvm.d2ui.rm"]
    fn llvm_d2ui_rm(a: f64) -> u32;
    #[link_name = "llvm.nvvm.d2ui.rp"]
    fn llvm_d2ui_rp(a: f64) -> u32;
    #[link_name = "llvm.nvvm.d2ll.rn"]
    fn llvm_d2ll_rn(a: f64) -> i64;
    #[link_name = "llvm.nvvm.d2ll.rz"]
    fn llvm_d2ll_rz(a: f64) -> i64;
    #[link_name = "llvm.nvvm.d2ll.rm"]
    fn llvm_d2ll_rm(a: f64) -> i64;
    #[link_name = "llvm.nvvm.d2ll.rp"]
    fn llvm_d2ll_rp(a: f64) -> i64;
    #[link_name = "llvm.nvvm.d2ull.rn"]
    fn llvm_d2ull_rn(a: f64) -> u64;
    #[link_name = "llvm.nvvm.d2ull.rz"]
    fn llvm_d2ull_rz(a: f64) -> u64;
    #[link_name = "llvm.nvvm.d2ull.rm"]
    fn llvm_d2ull_rm(a: f64) -> u64;
    #[link_name = "llvm.nvvm.d2ull.rp"]
    fn llvm_d2ull_rp(a: f64) -> u64;
    #[link_name = "llvm.nvvm.i2f.rn"]
    fn llvm_i2f_rn(a: i32) -> f32;
    #[link_name = "llvm.nvvm.i2f.rz"]
    fn llvm_i2f_rz(a: i32) -> f32;
    #[link_name = "llvm.nvvm.i2f.rm"]
    fn llvm_i2f_rm(a: i32) -> f32;
    #[link_name = "llvm.nvvm.i2f.rp"]
    fn llvm_i2f_rp(a: i32) -> f32;
    #[link_name = "llvm.nvvm.ui2f.rn"]
    fn llvm_ui2f_rn(a: u32) -> f32;
    #[link_name = "llvm.nvvm.ui2f.rz"]
    fn llvm_ui2f_rz(a: u32) -> f32;
    #[link_name = "llvm.nvvm.ui2f.rm"]
    fn llvm_ui2f_rm(a: u32) -> f32;
    #[link_name = "llvm.nvvm.ui2f.rp"]
    fn llvm_ui2f_rp(a: u32) -> f32;
    #[link_name = "llvm.nvvm.ll2f.rn"]
    fn llvm_ll2f_rn(a: i64) -> f32;
    #[link_name = "llvm.nvvm.ll2f.rz"]
    fn llvm_ll2f_rz(a: i64) -> f32;
    #[link_name = "llvm.nvvm.ll2f.rm"]
    fn llvm_ll2f_rm(a: i64) -> f32;
    #[link_name = "llvm.nvvm.ll2f.rp"]
    fn llvm_ll2f_rp(a: i64) -> f32;
    #[link_name = "llvm.nvvm.ull2f.rn"]
    fn llvm_ull2f_rn(a: u64) -> f32;
    #[link_name = "llvm.nvvm.ull2f.rz"]
    fn llvm_ull2f_rz(a: u64) -> f32;
    #[link_name = "llvm.nvvm.ull2f.rm"]
    fn llvm_ull2f_rm(a: u64) -> f32;
    #[link_name = "llvm.nvvm.ull2f.rp"]
    fn llvm_ull2f_rp(a: u64) -> f32;
    #[link_name = "llvm.nvvm.i2d.rn"]
    fn llvm_i2d_rn(a: i32) -> f64;
    #[link_name = "llvm.nvvm.i2d.rz"]
    fn llvm_i2d_rz(a: i32) -> f64;
    #[link_name = "llvm.nvvm.i2d.rm"]
    fn llvm_i2d_rm(a: i32) -> f64;
    #[link_name = "llvm.nvvm.i2d.rp"]
    fn llvm_i2d_rp(a: i32) -> f64;
    #[link_name = "llvm.nvvm.ui2d.rn"]
    fn llvm_ui2d_rn(a: u32) -> f64;
    #[link_name = "llvm.nvvm.ui2d.rz"]
    fn llvm_ui2d_rz(a: u32) -> f64;
    #[link_name = "llvm.nvvm.ui2d.rm"]
    fn llvm_ui2d_rm(a: u32) -> f64;
    #[link_name = "llvm.nvvm.ui2d.rp"]
    fn llvm_ui2d_rp(a: u32) -> f64;
    #[link_name = "llvm.nvvm.ll2d.rn"]
    fn llvm_ll2d_rn(a: i64) -> f64;
    #[link_name = "llvm.nvvm.ll2d.rz"]
    fn llvm_ll2d_rz(a: i64) -> f64;
    #[link_name = "llvm.nvvm.ll2d.rm"]
    fn llvm_ll2d_rm(a: i64) -> f64;
    #[link_name = "llvm.nvvm.ll2d.rp"]
    fn llvm_ll2d_rp(a: i64) -> f64;
    #[link_name = "llvm.nvvm.ull2d.rn"]
    fn llvm_ull2d_rn(a: u64) -> f64;
    #[link_name = "llvm.nvvm.ull2d.rz"]
    fn llvm_ull2d_rz(a: u64) -> f64;
    #[link_name = "llvm.nvvm.ull2d.rm"]
    fn llvm_ull2d_rm(a: u64) -> f64;
    #[link_name = "llvm.nvvm.ull2d.rp"]
    fn llvm_ull2d_rp(a: u64) -> f64;
    #[link_name = "llvm.nvvm.d2f.rn"]
    fn llvm_d2f_rn(a: f64) -> f32;
    #[link_name = "llvm.nvvm.d2f.rn.ftz"]
    fn llvm_d2f_rn_ftz(a: f64) -> f32;
    #[link_name = "llvm.nvvm.d2f.rz"]
    fn llvm_d2f_rz(a: f64) -> f32;
    #[link_name = "llvm.nvvm.d2f.rz.ftz"]
    fn llvm_d2f_rz_ftz(a: f64) -> f32;
    #[link_name = "llvm.nvvm.d2f.rm"]
    fn llvm_d2f_rm(a: f64) -> f32;
    #[link_name = "llvm.nvvm.d2f.rm.ftz"]
    fn llvm_d2f_rm_ftz(a: f64) -> f32;
    #[link_name = "llvm.nvvm.d2f.rp"]
    fn llvm_d2f_rp(a: f64) -> f32;
    #[link_name = "llvm.nvvm.d2f.rp.ftz"]
    fn llvm_d2f_rp_ftz(a: f64) -> f32;
    #[link_name = "llvm.nvvm.round.f"]
    fn llvm_round_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.round.ftz.f"]
    fn llvm_round_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.trunc.f"]
    fn llvm_trunc_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.trunc.ftz.f"]
    fn llvm_trunc_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.floor.f"]
    fn llvm_floor_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.floor.ftz.f"]
    fn llvm_floor_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.ceil.f"]
    fn llvm_ceil_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.ceil.ftz.f"]
    fn llvm_ceil_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.round.d"]
    fn llvm_round_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.trunc.d"]
    fn llvm_trunc_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.floor.d"]
    fn llvm_floor_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.ceil.d"]
    fn llvm_ceil_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.saturate.f"]
    fn llvm_saturate_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.saturate.ftz.f"]
    fn llvm_saturate_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.saturate.d"]
    fn llvm_saturate_f64(a: f64) -> f64;
}

/// Converts an `f32` to a signed 32-bit integer, rounding with `RND`.
///
/// `RND` is applied as the integer rounding modifier (`.rni`, `.rzi`, `.rmi`, or `.rpi`). `FTZ`
/// flushes subnormal inputs to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_s32_f32<const RND: Rounding, const FTZ: bool>(a: f32) -> i32 {
    match (RND, FTZ) {
        (Rounding::Rn, false) => llvm_f2i_rn(a),
        (Rounding::Rn, true) => llvm_f2i_rn_ftz(a),
        (Rounding::Rz, false) => llvm_f2i_rz(a),
        (Rounding::Rz, true) => llvm_f2i_rz_ftz(a),
        (Rounding::Rm, false) => llvm_f2i_rm(a),
        (Rounding::Rm, true) => llvm_f2i_rm_ftz(a),
        (Rounding::Rp, false) => llvm_f2i_rp(a),
        (Rounding::Rp, true) => llvm_f2i_rp_ftz(a),
    }
}

/// Converts an `f32` to an unsigned 32-bit integer, rounding with `RND`.
///
/// `RND` is applied as the integer rounding modifier (`.rni`, `.rzi`, `.rmi`, or `.rpi`). `FTZ`
/// flushes subnormal inputs to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_u32_f32<const RND: Rounding, const FTZ: bool>(a: f32) -> u32 {
    match (RND, FTZ) {
        (Rounding::Rn, false) => llvm_f2ui_rn(a),
        (Rounding::Rn, true) => llvm_f2ui_rn_ftz(a),
        (Rounding::Rz, false) => llvm_f2ui_rz(a),
        (Rounding::Rz, true) => llvm_f2ui_rz_ftz(a),
        (Rounding::Rm, false) => llvm_f2ui_rm(a),
        (Rounding::Rm, true) => llvm_f2ui_rm_ftz(a),
        (Rounding::Rp, false) => llvm_f2ui_rp(a),
        (Rounding::Rp, true) => llvm_f2ui_rp_ftz(a),
    }
}

/// Converts an `f32` to a signed 64-bit integer, rounding with `RND`.
///
/// `RND` is applied as the integer rounding modifier (`.rni`, `.rzi`, `.rmi`, or `.rpi`). `FTZ`
/// flushes subnormal inputs to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_s64_f32<const RND: Rounding, const FTZ: bool>(a: f32) -> i64 {
    match (RND, FTZ) {
        (Rounding::Rn, false) => llvm_f2ll_rn(a),
        (Rounding::Rn, true) => llvm_f2ll_rn_ftz(a),
        (Rounding::Rz, false) => llvm_f2ll_rz(a),
        (Rounding::Rz, true) => llvm_f2ll_rz_ftz(a),
        (Rounding::Rm, false) => llvm_f2ll_rm(a),
        (Rounding::Rm, true) => llvm_f2ll_rm_ftz(a),
        (Rounding::Rp, false) => llvm_f2ll_rp(a),
        (Rounding::Rp, true) => llvm_f2ll_rp_ftz(a),
    }
}

/// Converts an `f32` to an unsigned 64-bit integer, rounding with `RND`.
///
/// `RND` is applied as the integer rounding modifier (`.rni`, `.rzi`, `.rmi`, or `.rpi`). `FTZ`
/// flushes subnormal inputs to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_u64_f32<const RND: Rounding, const FTZ: bool>(a: f32) -> u64 {
    match (RND, FTZ) {
        (Rounding::Rn, false) => llvm_f2ull_rn(a),
        (Rounding::Rn, true) => llvm_f2ull_rn_ftz(a),
        (Rounding::Rz, false) => llvm_f2ull_rz(a),
        (Rounding::Rz, true) => llvm_f2ull_rz_ftz(a),
        (Rounding::Rm, false) => llvm_f2ull_rm(a),
        (Rounding::Rm, true) => llvm_f2ull_rm_ftz(a),
        (Rounding::Rp, false) => llvm_f2ull_rp(a),
        (Rounding::Rp, true) => llvm_f2ull_rp_ftz(a),
    }
}

/// Converts an `f64` to a signed 32-bit integer, rounding with `RND`.
///
/// `RND` is applied as the integer rounding modifier (`.rni`, `.rzi`, `.rmi`, or `.rpi`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_s32_f64<const RND: Rounding>(a: f64) -> i32 {
    match RND {
        Rounding::Rn => llvm_d2i_rn(a),
        Rounding::Rz => llvm_d2i_rz(a),
        Rounding::Rm => llvm_d2i_rm(a),
        Rounding::Rp => llvm_d2i_rp(a),
    }
}

/// Converts an `f64` to an unsigned 32-bit integer, rounding with `RND`.
///
/// `RND` is applied as the integer rounding modifier (`.rni`, `.rzi`, `.rmi`, or `.rpi`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_u32_f64<const RND: Rounding>(a: f64) -> u32 {
    match RND {
        Rounding::Rn => llvm_d2ui_rn(a),
        Rounding::Rz => llvm_d2ui_rz(a),
        Rounding::Rm => llvm_d2ui_rm(a),
        Rounding::Rp => llvm_d2ui_rp(a),
    }
}

/// Converts an `f64` to a signed 64-bit integer, rounding with `RND`.
///
/// `RND` is applied as the integer rounding modifier (`.rni`, `.rzi`, `.rmi`, or `.rpi`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_s64_f64<const RND: Rounding>(a: f64) -> i64 {
    match RND {
        Rounding::Rn => llvm_d2ll_rn(a),
        Rounding::Rz => llvm_d2ll_rz(a),
        Rounding::Rm => llvm_d2ll_rm(a),
        Rounding::Rp => llvm_d2ll_rp(a),
    }
}

/// Converts an `f64` to an unsigned 64-bit integer, rounding with `RND`.
///
/// `RND` is applied as the integer rounding modifier (`.rni`, `.rzi`, `.rmi`, or `.rpi`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_u64_f64<const RND: Rounding>(a: f64) -> u64 {
    match RND {
        Rounding::Rn => llvm_d2ull_rn(a),
        Rounding::Rz => llvm_d2ull_rz(a),
        Rounding::Rm => llvm_d2ull_rm(a),
        Rounding::Rp => llvm_d2ull_rp(a),
    }
}

/// Converts a signed 32-bit integer to an `f32`, rounding with `RND` (`.rn`, `.rz`, `.rm`, or
/// `.rp`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f32_s32<const RND: Rounding>(a: i32) -> f32 {
    match RND {
        Rounding::Rn => llvm_i2f_rn(a),
        Rounding::Rz => llvm_i2f_rz(a),
        Rounding::Rm => llvm_i2f_rm(a),
        Rounding::Rp => llvm_i2f_rp(a),
    }
}

/// Converts an unsigned 32-bit integer to an `f32`, rounding with `RND` (`.rn`, `.rz`, `.rm`, or
/// `.rp`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f32_u32<const RND: Rounding>(a: u32) -> f32 {
    match RND {
        Rounding::Rn => llvm_ui2f_rn(a),
        Rounding::Rz => llvm_ui2f_rz(a),
        Rounding::Rm => llvm_ui2f_rm(a),
        Rounding::Rp => llvm_ui2f_rp(a),
    }
}

/// Converts a signed 64-bit integer to an `f32`, rounding with `RND` (`.rn`, `.rz`, `.rm`, or
/// `.rp`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f32_s64<const RND: Rounding>(a: i64) -> f32 {
    match RND {
        Rounding::Rn => llvm_ll2f_rn(a),
        Rounding::Rz => llvm_ll2f_rz(a),
        Rounding::Rm => llvm_ll2f_rm(a),
        Rounding::Rp => llvm_ll2f_rp(a),
    }
}

/// Converts an unsigned 64-bit integer to an `f32`, rounding with `RND` (`.rn`, `.rz`, `.rm`, or
/// `.rp`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f32_u64<const RND: Rounding>(a: u64) -> f32 {
    match RND {
        Rounding::Rn => llvm_ull2f_rn(a),
        Rounding::Rz => llvm_ull2f_rz(a),
        Rounding::Rm => llvm_ull2f_rm(a),
        Rounding::Rp => llvm_ull2f_rp(a),
    }
}

/// Converts a signed 32-bit integer to an `f64`, rounding with `RND` (`.rn`, `.rz`, `.rm`, or
/// `.rp`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f64_s32<const RND: Rounding>(a: i32) -> f64 {
    match RND {
        Rounding::Rn => llvm_i2d_rn(a),
        Rounding::Rz => llvm_i2d_rz(a),
        Rounding::Rm => llvm_i2d_rm(a),
        Rounding::Rp => llvm_i2d_rp(a),
    }
}

/// Converts an unsigned 32-bit integer to an `f64`, rounding with `RND` (`.rn`, `.rz`, `.rm`, or
/// `.rp`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f64_u32<const RND: Rounding>(a: u32) -> f64 {
    match RND {
        Rounding::Rn => llvm_ui2d_rn(a),
        Rounding::Rz => llvm_ui2d_rz(a),
        Rounding::Rm => llvm_ui2d_rm(a),
        Rounding::Rp => llvm_ui2d_rp(a),
    }
}

/// Converts a signed 64-bit integer to an `f64`, rounding with `RND` (`.rn`, `.rz`, `.rm`, or
/// `.rp`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f64_s64<const RND: Rounding>(a: i64) -> f64 {
    match RND {
        Rounding::Rn => llvm_ll2d_rn(a),
        Rounding::Rz => llvm_ll2d_rz(a),
        Rounding::Rm => llvm_ll2d_rm(a),
        Rounding::Rp => llvm_ll2d_rp(a),
    }
}

/// Converts an unsigned 64-bit integer to an `f64`, rounding with `RND` (`.rn`, `.rz`, `.rm`, or
/// `.rp`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f64_u64<const RND: Rounding>(a: u64) -> f64 {
    match RND {
        Rounding::Rn => llvm_ull2d_rn(a),
        Rounding::Rz => llvm_ull2d_rz(a),
        Rounding::Rm => llvm_ull2d_rm(a),
        Rounding::Rp => llvm_ull2d_rp(a),
    }
}

/// Converts an `f64` to an `f32`, rounding with `RND` (`.rn`, `.rz`, `.rm`, or `.rp`).
///
/// `FTZ` flushes subnormal results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f32_f64<const RND: Rounding, const FTZ: bool>(a: f64) -> f32 {
    match (RND, FTZ) {
        (Rounding::Rn, false) => llvm_d2f_rn(a),
        (Rounding::Rn, true) => llvm_d2f_rn_ftz(a),
        (Rounding::Rz, false) => llvm_d2f_rz(a),
        (Rounding::Rz, true) => llvm_d2f_rz_ftz(a),
        (Rounding::Rm, false) => llvm_d2f_rm(a),
        (Rounding::Rm, true) => llvm_d2f_rm_ftz(a),
        (Rounding::Rp, false) => llvm_d2f_rp(a),
        (Rounding::Rp, true) => llvm_d2f_rp_ftz(a),
    }
}

/// Rounds an `f32` to an integral value, returned as an `f32`.
///
/// `RND` is applied as the integer rounding modifier (`.rni`, `.rzi`, `.rmi`, or `.rpi`). `FTZ`
/// flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f32_f32<const RND: Rounding, const FTZ: bool>(a: f32) -> f32 {
    match (RND, FTZ) {
        (Rounding::Rn, false) => llvm_round_f32(a),
        (Rounding::Rn, true) => llvm_round_ftz_f32(a),
        (Rounding::Rz, false) => llvm_trunc_f32(a),
        (Rounding::Rz, true) => llvm_trunc_ftz_f32(a),
        (Rounding::Rm, false) => llvm_floor_f32(a),
        (Rounding::Rm, true) => llvm_floor_ftz_f32(a),
        (Rounding::Rp, false) => llvm_ceil_f32(a),
        (Rounding::Rp, true) => llvm_ceil_ftz_f32(a),
    }
}

/// Rounds an `f64` to an integral value, returned as an `f64`.
///
/// `RND` is applied as the integer rounding modifier (`.rni`, `.rzi`, `.rmi`, or `.rpi`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f64_f64<const RND: Rounding>(a: f64) -> f64 {
    match RND {
        Rounding::Rn => llvm_round_f64(a),
        Rounding::Rz => llvm_trunc_f64(a),
        Rounding::Rm => llvm_floor_f64(a),
        Rounding::Rp => llvm_ceil_f64(a),
    }
}

/// Clamps an `f32` to the range `[0.0, 1.0]`.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_sat_f32_f32<const FTZ: bool>(a: f32) -> f32 {
    match FTZ {
        false => llvm_saturate_f32(a),
        true => llvm_saturate_ftz_f32(a),
    }
}

/// Clamps an `f64` to the range `[0.0, 1.0]`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_sat_f64_f64(a: f64) -> f64 {
    llvm_saturate_f64(a)
}
