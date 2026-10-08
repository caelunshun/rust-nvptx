// --- LLM-generated --- //
//! Floating-point minimum and maximum instructions.

use super::packed::{bf16, bf16x2, f16x2};

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.fmin.f"]
    fn llvm_fmin_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmin.xorsign.abs.f"]
    fn llvm_fmin_xorsign_abs_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmin.nan.f"]
    fn llvm_fmin_nan_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmin.nan.xorsign.abs.f"]
    fn llvm_fmin_nan_xorsign_abs_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmin.ftz.f"]
    fn llvm_fmin_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmin.ftz.xorsign.abs.f"]
    fn llvm_fmin_ftz_xorsign_abs_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmin.ftz.nan.f"]
    fn llvm_fmin_ftz_nan_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmin.ftz.nan.xorsign.abs.f"]
    fn llvm_fmin_ftz_nan_xorsign_abs_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmin.f16"]
    fn llvm_fmin_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmin.xorsign.abs.f16"]
    fn llvm_fmin_xorsign_abs_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmin.nan.f16"]
    fn llvm_fmin_nan_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmin.nan.xorsign.abs.f16"]
    fn llvm_fmin_nan_xorsign_abs_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmin.ftz.f16"]
    fn llvm_fmin_ftz_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmin.ftz.xorsign.abs.f16"]
    fn llvm_fmin_ftz_xorsign_abs_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmin.ftz.nan.f16"]
    fn llvm_fmin_ftz_nan_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmin.ftz.nan.xorsign.abs.f16"]
    fn llvm_fmin_ftz_nan_xorsign_abs_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmin.f16x2"]
    fn llvm_fmin_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmin.xorsign.abs.f16x2"]
    fn llvm_fmin_xorsign_abs_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmin.nan.f16x2"]
    fn llvm_fmin_nan_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmin.nan.xorsign.abs.f16x2"]
    fn llvm_fmin_nan_xorsign_abs_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmin.ftz.f16x2"]
    fn llvm_fmin_ftz_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmin.ftz.xorsign.abs.f16x2"]
    fn llvm_fmin_ftz_xorsign_abs_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmin.ftz.nan.f16x2"]
    fn llvm_fmin_ftz_nan_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmin.ftz.nan.xorsign.abs.f16x2"]
    fn llvm_fmin_ftz_nan_xorsign_abs_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmin.bf16"]
    fn llvm_fmin_bf16(a: bf16, b: bf16) -> bf16;
    #[link_name = "llvm.nvvm.fmin.xorsign.abs.bf16"]
    fn llvm_fmin_xorsign_abs_bf16(a: bf16, b: bf16) -> bf16;
    #[link_name = "llvm.nvvm.fmin.nan.bf16"]
    fn llvm_fmin_nan_bf16(a: bf16, b: bf16) -> bf16;
    #[link_name = "llvm.nvvm.fmin.nan.xorsign.abs.bf16"]
    fn llvm_fmin_nan_xorsign_abs_bf16(a: bf16, b: bf16) -> bf16;
    #[link_name = "llvm.nvvm.fmin.bf16x2"]
    fn llvm_fmin_bf16x2(a: bf16x2, b: bf16x2) -> bf16x2;
    #[link_name = "llvm.nvvm.fmin.xorsign.abs.bf16x2"]
    fn llvm_fmin_xorsign_abs_bf16x2(a: bf16x2, b: bf16x2) -> bf16x2;
    #[link_name = "llvm.nvvm.fmin.nan.bf16x2"]
    fn llvm_fmin_nan_bf16x2(a: bf16x2, b: bf16x2) -> bf16x2;
    #[link_name = "llvm.nvvm.fmin.nan.xorsign.abs.bf16x2"]
    fn llvm_fmin_nan_xorsign_abs_bf16x2(a: bf16x2, b: bf16x2) -> bf16x2;
    #[link_name = "llvm.nvvm.fmin.d"]
    fn llvm_fmin_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.fmax.f"]
    fn llvm_fmax_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmax.xorsign.abs.f"]
    fn llvm_fmax_xorsign_abs_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmax.nan.f"]
    fn llvm_fmax_nan_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmax.nan.xorsign.abs.f"]
    fn llvm_fmax_nan_xorsign_abs_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmax.ftz.f"]
    fn llvm_fmax_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmax.ftz.xorsign.abs.f"]
    fn llvm_fmax_ftz_xorsign_abs_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmax.ftz.nan.f"]
    fn llvm_fmax_ftz_nan_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmax.ftz.nan.xorsign.abs.f"]
    fn llvm_fmax_ftz_nan_xorsign_abs_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.fmax.f16"]
    fn llvm_fmax_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmax.xorsign.abs.f16"]
    fn llvm_fmax_xorsign_abs_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmax.nan.f16"]
    fn llvm_fmax_nan_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmax.nan.xorsign.abs.f16"]
    fn llvm_fmax_nan_xorsign_abs_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmax.ftz.f16"]
    fn llvm_fmax_ftz_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmax.ftz.xorsign.abs.f16"]
    fn llvm_fmax_ftz_xorsign_abs_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmax.ftz.nan.f16"]
    fn llvm_fmax_ftz_nan_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmax.ftz.nan.xorsign.abs.f16"]
    fn llvm_fmax_ftz_nan_xorsign_abs_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.fmax.f16x2"]
    fn llvm_fmax_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmax.xorsign.abs.f16x2"]
    fn llvm_fmax_xorsign_abs_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmax.nan.f16x2"]
    fn llvm_fmax_nan_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmax.nan.xorsign.abs.f16x2"]
    fn llvm_fmax_nan_xorsign_abs_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmax.ftz.f16x2"]
    fn llvm_fmax_ftz_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmax.ftz.xorsign.abs.f16x2"]
    fn llvm_fmax_ftz_xorsign_abs_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmax.ftz.nan.f16x2"]
    fn llvm_fmax_ftz_nan_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmax.ftz.nan.xorsign.abs.f16x2"]
    fn llvm_fmax_ftz_nan_xorsign_abs_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fmax.bf16"]
    fn llvm_fmax_bf16(a: bf16, b: bf16) -> bf16;
    #[link_name = "llvm.nvvm.fmax.xorsign.abs.bf16"]
    fn llvm_fmax_xorsign_abs_bf16(a: bf16, b: bf16) -> bf16;
    #[link_name = "llvm.nvvm.fmax.nan.bf16"]
    fn llvm_fmax_nan_bf16(a: bf16, b: bf16) -> bf16;
    #[link_name = "llvm.nvvm.fmax.nan.xorsign.abs.bf16"]
    fn llvm_fmax_nan_xorsign_abs_bf16(a: bf16, b: bf16) -> bf16;
    #[link_name = "llvm.nvvm.fmax.bf16x2"]
    fn llvm_fmax_bf16x2(a: bf16x2, b: bf16x2) -> bf16x2;
    #[link_name = "llvm.nvvm.fmax.xorsign.abs.bf16x2"]
    fn llvm_fmax_xorsign_abs_bf16x2(a: bf16x2, b: bf16x2) -> bf16x2;
    #[link_name = "llvm.nvvm.fmax.nan.bf16x2"]
    fn llvm_fmax_nan_bf16x2(a: bf16x2, b: bf16x2) -> bf16x2;
    #[link_name = "llvm.nvvm.fmax.nan.xorsign.abs.bf16x2"]
    fn llvm_fmax_nan_xorsign_abs_bf16x2(a: bf16x2, b: bf16x2) -> bf16x2;
    #[link_name = "llvm.nvvm.fmax.d"]
    fn llvm_fmax_f64(a: f64, b: f64) -> f64;
}

/// Finds the minimum of two `f32` values.
///
/// `-0.0` is treated as less than `+0.0`.
///
/// If `NAN` is `false`, a NaN input is ignored when the other input is not NaN, and the result is
/// NaN only if both inputs are NaN. If `NAN` is `true`, the result is a canonical NaN if either
/// input is NaN.
///
/// If `FTZ` is `true`, subnormal inputs and results are flushed to sign-preserving zero.
///
/// If `XORSIGN_ABS` is `true`, the magnitude of the result is the minimum of the absolute values of
/// the inputs, and its sign bit is the XOR of the input sign bits. This is ignored if the result is
/// NaN.
///
/// `NAN` requires sm_80 and PTX ISA 7.0, and `XORSIGN_ABS` requires sm_86 and PTX ISA 7.2. The
/// plain and `FTZ` forms have no additional requirements.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-min>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn min_f32<const FTZ: bool, const NAN: bool, const XORSIGN_ABS: bool>(
    a: f32,
    b: f32,
) -> f32 {
    match (FTZ, NAN, XORSIGN_ABS) {
        (false, false, false) => llvm_fmin_f32(a, b),
        (false, false, true) => llvm_fmin_xorsign_abs_f32(a, b),
        (false, true, false) => llvm_fmin_nan_f32(a, b),
        (false, true, true) => llvm_fmin_nan_xorsign_abs_f32(a, b),
        (true, false, false) => llvm_fmin_ftz_f32(a, b),
        (true, false, true) => llvm_fmin_ftz_xorsign_abs_f32(a, b),
        (true, true, false) => llvm_fmin_ftz_nan_f32(a, b),
        (true, true, true) => llvm_fmin_ftz_nan_xorsign_abs_f32(a, b),
    }
}

/// Finds the minimum of two `f16` values.
///
/// `-0.0` is treated as less than `+0.0`.
///
/// If `NAN` is `false`, a NaN input is ignored when the other input is not NaN, and the result is
/// NaN only if both inputs are NaN. If `NAN` is `true`, the result is a canonical NaN if either
/// input is NaN.
///
/// If `FTZ` is `true`, subnormal inputs and results are flushed to sign-preserving zero.
///
/// If `XORSIGN_ABS` is `true`, the magnitude of the result is the minimum of the absolute values of
/// the inputs, and its sign bit is the XOR of the input sign bits. This is ignored if the result is
/// NaN.
///
/// Requires sm_80 and PTX ISA 7.0. `XORSIGN_ABS` additionally requires sm_86 and PTX ISA 7.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-min>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn min_f16<const FTZ: bool, const NAN: bool, const XORSIGN_ABS: bool>(
    a: f16,
    b: f16,
) -> f16 {
    match (FTZ, NAN, XORSIGN_ABS) {
        (false, false, false) => llvm_fmin_f16(a, b),
        (false, false, true) => llvm_fmin_xorsign_abs_f16(a, b),
        (false, true, false) => llvm_fmin_nan_f16(a, b),
        (false, true, true) => llvm_fmin_nan_xorsign_abs_f16(a, b),
        (true, false, false) => llvm_fmin_ftz_f16(a, b),
        (true, false, true) => llvm_fmin_ftz_xorsign_abs_f16(a, b),
        (true, true, false) => llvm_fmin_ftz_nan_f16(a, b),
        (true, true, true) => llvm_fmin_ftz_nan_xorsign_abs_f16(a, b),
    }
}

/// Finds the minimum of two `f16x2` values.
///
/// `-0.0` is treated as less than `+0.0`.
///
/// If `NAN` is `false`, a NaN input is ignored when the other input is not NaN, and the result is
/// NaN only if both inputs are NaN. If `NAN` is `true`, the result is a canonical NaN if either
/// input is NaN.
///
/// If `FTZ` is `true`, subnormal inputs and results are flushed to sign-preserving zero.
///
/// If `XORSIGN_ABS` is `true`, the magnitude of the result is the minimum of the absolute values of
/// the inputs, and its sign bit is the XOR of the input sign bits. This is ignored if the result is
/// NaN.
///
/// Requires sm_80 and PTX ISA 7.0. `XORSIGN_ABS` additionally requires sm_86 and PTX ISA 7.2.
///
/// Corresponds to the CUDA C intrinsics
/// [`__hmin2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__COMPARISON.html#group__CUDA__MATH____HALF2__COMPARISON_1g9e17a33f96061804166f3fbd395422b6)
/// (`NAN` = `false`) and
/// [`__hmin2_nan`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__COMPARISON.html#group__CUDA__MATH____HALF2__COMPARISON_1g8bb8f58e9294cc261d2f42c4d5aecd6b)
/// (`NAN` = `true`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-min>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn min_f16x2<const FTZ: bool, const NAN: bool, const XORSIGN_ABS: bool>(
    a: f16x2,
    b: f16x2,
) -> f16x2 {
    match (FTZ, NAN, XORSIGN_ABS) {
        (false, false, false) => llvm_fmin_f16x2(a, b),
        (false, false, true) => llvm_fmin_xorsign_abs_f16x2(a, b),
        (false, true, false) => llvm_fmin_nan_f16x2(a, b),
        (false, true, true) => llvm_fmin_nan_xorsign_abs_f16x2(a, b),
        (true, false, false) => llvm_fmin_ftz_f16x2(a, b),
        (true, false, true) => llvm_fmin_ftz_xorsign_abs_f16x2(a, b),
        (true, true, false) => llvm_fmin_ftz_nan_f16x2(a, b),
        (true, true, true) => llvm_fmin_ftz_nan_xorsign_abs_f16x2(a, b),
    }
}

/// Finds the minimum of two `bf16` values.
///
/// `-0.0` is treated as less than `+0.0`.
///
/// If `NAN` is `false`, a NaN input is ignored when the other input is not NaN, and the result is
/// NaN only if both inputs are NaN. If `NAN` is `true`, the result is a canonical NaN if either
/// input is NaN.
///
/// If `XORSIGN_ABS` is `true`, the magnitude of the result is the minimum of the absolute values of
/// the inputs, and its sign bit is the XOR of the input sign bits. This is ignored if the result is
/// NaN.
///
/// PTX has no `FTZ` form for `bf16`.
///
/// Requires sm_80 and PTX ISA 7.0. `XORSIGN_ABS` additionally requires sm_86 and PTX ISA 7.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-min>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn min_bf16<const NAN: bool, const XORSIGN_ABS: bool>(a: bf16, b: bf16) -> bf16 {
    match (NAN, XORSIGN_ABS) {
        (false, false) => llvm_fmin_bf16(a, b),
        (false, true) => llvm_fmin_xorsign_abs_bf16(a, b),
        (true, false) => llvm_fmin_nan_bf16(a, b),
        (true, true) => llvm_fmin_nan_xorsign_abs_bf16(a, b),
    }
}

/// Finds the minimum of two `bf16x2` values.
///
/// `-0.0` is treated as less than `+0.0`.
///
/// If `NAN` is `false`, a NaN input is ignored when the other input is not NaN, and the result is
/// NaN only if both inputs are NaN. If `NAN` is `true`, the result is a canonical NaN if either
/// input is NaN.
///
/// If `XORSIGN_ABS` is `true`, the magnitude of the result is the minimum of the absolute values of
/// the inputs, and its sign bit is the XOR of the input sign bits. This is ignored if the result is
/// NaN.
///
/// PTX has no `FTZ` form for `bf16`.
///
/// Requires sm_80 and PTX ISA 7.0. `XORSIGN_ABS` additionally requires sm_86 and PTX ISA 7.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-min>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn min_bf16x2<const NAN: bool, const XORSIGN_ABS: bool>(a: bf16x2, b: bf16x2) -> bf16x2 {
    match (NAN, XORSIGN_ABS) {
        (false, false) => llvm_fmin_bf16x2(a, b),
        (false, true) => llvm_fmin_xorsign_abs_bf16x2(a, b),
        (true, false) => llvm_fmin_nan_bf16x2(a, b),
        (true, true) => llvm_fmin_nan_xorsign_abs_bf16x2(a, b),
    }
}

/// Finds the minimum of two `f64` values.
///
/// `-0.0` is treated as less than `+0.0`.
///
/// If one input is NaN, the other input is returned. The result is NaN only if both inputs are NaN.
///
/// Requires sm_13 or higher.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-min>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn min_f64(a: f64, b: f64) -> f64 {
    llvm_fmin_f64(a, b)
}

/// Finds the maximum of two `f32` values.
///
/// `+0.0` is treated as greater than `-0.0`.
///
/// If `NAN` is `false`, a NaN input is ignored when the other input is not NaN, and the result is
/// NaN only if both inputs are NaN. If `NAN` is `true`, the result is a canonical NaN if either
/// input is NaN.
///
/// If `FTZ` is `true`, subnormal inputs and results are flushed to sign-preserving zero.
///
/// If `XORSIGN_ABS` is `true`, the magnitude of the result is the maximum of the absolute values of
/// the inputs, and its sign bit is the XOR of the input sign bits. This is ignored if the result is
/// NaN.
///
/// `NAN` requires sm_80 and PTX ISA 7.0, and `XORSIGN_ABS` requires sm_86 and PTX ISA 7.2. The
/// plain and `FTZ` forms have no additional requirements.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-max>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn max_f32<const FTZ: bool, const NAN: bool, const XORSIGN_ABS: bool>(
    a: f32,
    b: f32,
) -> f32 {
    match (FTZ, NAN, XORSIGN_ABS) {
        (false, false, false) => llvm_fmax_f32(a, b),
        (false, false, true) => llvm_fmax_xorsign_abs_f32(a, b),
        (false, true, false) => llvm_fmax_nan_f32(a, b),
        (false, true, true) => llvm_fmax_nan_xorsign_abs_f32(a, b),
        (true, false, false) => llvm_fmax_ftz_f32(a, b),
        (true, false, true) => llvm_fmax_ftz_xorsign_abs_f32(a, b),
        (true, true, false) => llvm_fmax_ftz_nan_f32(a, b),
        (true, true, true) => llvm_fmax_ftz_nan_xorsign_abs_f32(a, b),
    }
}

/// Finds the maximum of two `f16` values.
///
/// `+0.0` is treated as greater than `-0.0`.
///
/// If `NAN` is `false`, a NaN input is ignored when the other input is not NaN, and the result is
/// NaN only if both inputs are NaN. If `NAN` is `true`, the result is a canonical NaN if either
/// input is NaN.
///
/// If `FTZ` is `true`, subnormal inputs and results are flushed to sign-preserving zero.
///
/// If `XORSIGN_ABS` is `true`, the magnitude of the result is the maximum of the absolute values of
/// the inputs, and its sign bit is the XOR of the input sign bits. This is ignored if the result is
/// NaN.
///
/// Requires sm_80 and PTX ISA 7.0. `XORSIGN_ABS` additionally requires sm_86 and PTX ISA 7.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-max>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn max_f16<const FTZ: bool, const NAN: bool, const XORSIGN_ABS: bool>(
    a: f16,
    b: f16,
) -> f16 {
    match (FTZ, NAN, XORSIGN_ABS) {
        (false, false, false) => llvm_fmax_f16(a, b),
        (false, false, true) => llvm_fmax_xorsign_abs_f16(a, b),
        (false, true, false) => llvm_fmax_nan_f16(a, b),
        (false, true, true) => llvm_fmax_nan_xorsign_abs_f16(a, b),
        (true, false, false) => llvm_fmax_ftz_f16(a, b),
        (true, false, true) => llvm_fmax_ftz_xorsign_abs_f16(a, b),
        (true, true, false) => llvm_fmax_ftz_nan_f16(a, b),
        (true, true, true) => llvm_fmax_ftz_nan_xorsign_abs_f16(a, b),
    }
}

/// Finds the maximum of two `f16x2` values.
///
/// `+0.0` is treated as greater than `-0.0`.
///
/// If `NAN` is `false`, a NaN input is ignored when the other input is not NaN, and the result is
/// NaN only if both inputs are NaN. If `NAN` is `true`, the result is a canonical NaN if either
/// input is NaN.
///
/// If `FTZ` is `true`, subnormal inputs and results are flushed to sign-preserving zero.
///
/// If `XORSIGN_ABS` is `true`, the magnitude of the result is the maximum of the absolute values of
/// the inputs, and its sign bit is the XOR of the input sign bits. This is ignored if the result is
/// NaN.
///
/// Requires sm_80 and PTX ISA 7.0. `XORSIGN_ABS` additionally requires sm_86 and PTX ISA 7.2.
///
/// Corresponds to the CUDA C intrinsics
/// [`__hmax2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__COMPARISON.html#group__CUDA__MATH____HALF2__COMPARISON_1g59fc7fc7975d8127b202444a05e57e3d)
/// (`NAN` = `false`) and
/// [`__hmax2_nan`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__COMPARISON.html#group__CUDA__MATH____HALF2__COMPARISON_1g41623db7850e3074fd9daa80a14c3897)
/// (`NAN` = `true`).
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-max>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn max_f16x2<const FTZ: bool, const NAN: bool, const XORSIGN_ABS: bool>(
    a: f16x2,
    b: f16x2,
) -> f16x2 {
    match (FTZ, NAN, XORSIGN_ABS) {
        (false, false, false) => llvm_fmax_f16x2(a, b),
        (false, false, true) => llvm_fmax_xorsign_abs_f16x2(a, b),
        (false, true, false) => llvm_fmax_nan_f16x2(a, b),
        (false, true, true) => llvm_fmax_nan_xorsign_abs_f16x2(a, b),
        (true, false, false) => llvm_fmax_ftz_f16x2(a, b),
        (true, false, true) => llvm_fmax_ftz_xorsign_abs_f16x2(a, b),
        (true, true, false) => llvm_fmax_ftz_nan_f16x2(a, b),
        (true, true, true) => llvm_fmax_ftz_nan_xorsign_abs_f16x2(a, b),
    }
}

/// Finds the maximum of two `bf16` values.
///
/// `+0.0` is treated as greater than `-0.0`.
///
/// If `NAN` is `false`, a NaN input is ignored when the other input is not NaN, and the result is
/// NaN only if both inputs are NaN. If `NAN` is `true`, the result is a canonical NaN if either
/// input is NaN.
///
/// If `XORSIGN_ABS` is `true`, the magnitude of the result is the maximum of the absolute values of
/// the inputs, and its sign bit is the XOR of the input sign bits. This is ignored if the result is
/// NaN.
///
/// PTX has no `FTZ` form for `bf16`.
///
/// Requires sm_80 and PTX ISA 7.0. `XORSIGN_ABS` additionally requires sm_86 and PTX ISA 7.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-max>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn max_bf16<const NAN: bool, const XORSIGN_ABS: bool>(a: bf16, b: bf16) -> bf16 {
    match (NAN, XORSIGN_ABS) {
        (false, false) => llvm_fmax_bf16(a, b),
        (false, true) => llvm_fmax_xorsign_abs_bf16(a, b),
        (true, false) => llvm_fmax_nan_bf16(a, b),
        (true, true) => llvm_fmax_nan_xorsign_abs_bf16(a, b),
    }
}

/// Finds the maximum of two `bf16x2` values.
///
/// `+0.0` is treated as greater than `-0.0`.
///
/// If `NAN` is `false`, a NaN input is ignored when the other input is not NaN, and the result is
/// NaN only if both inputs are NaN. If `NAN` is `true`, the result is a canonical NaN if either
/// input is NaN.
///
/// If `XORSIGN_ABS` is `true`, the magnitude of the result is the maximum of the absolute values of
/// the inputs, and its sign bit is the XOR of the input sign bits. This is ignored if the result is
/// NaN.
///
/// PTX has no `FTZ` form for `bf16`.
///
/// Requires sm_80 and PTX ISA 7.0. `XORSIGN_ABS` additionally requires sm_86 and PTX ISA 7.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-max>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn max_bf16x2<const NAN: bool, const XORSIGN_ABS: bool>(a: bf16x2, b: bf16x2) -> bf16x2 {
    match (NAN, XORSIGN_ABS) {
        (false, false) => llvm_fmax_bf16x2(a, b),
        (false, true) => llvm_fmax_xorsign_abs_bf16x2(a, b),
        (true, false) => llvm_fmax_nan_bf16x2(a, b),
        (true, true) => llvm_fmax_nan_xorsign_abs_bf16x2(a, b),
    }
}

/// Finds the maximum of two `f64` values.
///
/// `+0.0` is treated as greater than `-0.0`.
///
/// If one input is NaN, the other input is returned. The result is NaN only if both inputs are NaN.
///
/// Requires sm_13 or higher.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-max>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn max_f64(a: f64, b: f64) -> f64 {
    llvm_fmax_f64(a, b)
}
