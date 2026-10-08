// --- LLM-generated --- //
//! Conversions to and from packed 8-bit, 6-bit and 4-bit floating-point formats.
//!
//! Functions are named `cvt_<dst>_<src>` after the PTX `cvt` instruction, with the destination
//! first. The formats are:
//! - `e4m3x2` and `e5m2x2`: packed pairs of FP8 values (E4M3 and E5M2).
//! - `e2m3x2` and `e3m2x2`: packed pairs of FP6 values (E2M3 and E3M2).
//! - `e2m1x2`: packed pairs of FP4 values (E2M1).
//! - `s2f6x2`: packed pairs of 6-bit values, only converted together with a `ue8m0` scale.
//! - `ue8m0x2`: packed pairs of unsigned 8-bit exponent scale factors (E8M0).
//!
//! Packed narrow values are plain integers: `u16` for the two-value forms (the `e2m1x2` forms
//! use only the low 8 bits), `u16` for `e2m1x4` and `u32` for the other four-value forms. In the
//! two-value forms, `a` is the first PTX source operand and is placed in the upper half of the
//! result, and `b` is placed in the lower half.
//!
//! The `rs` forms convert four `f32` values with stochastic rounding, using `rbits` as the random
//! bits. The values are `a`, `b`, `c` and `d` in PTX operand order.

use super::modifiers::Rounding;
use super::packed::{bf16x2, f16x2};
use crate::core_arch::simd::{f32x4, i8x4};
use crate::mem::transmute;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.ff.to.e4m3x2.rn"]
    fn llvm_ff_to_e4m3x2_rn(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.e4m3x2.rn.relu"]
    fn llvm_ff_to_e4m3x2_rn_relu(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.e5m2x2.rn"]
    fn llvm_ff_to_e5m2x2_rn(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.e5m2x2.rn.relu"]
    fn llvm_ff_to_e5m2x2_rn_relu(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.e2m1x2.rn.satfinite"]
    fn llvm_ff_to_e2m1x2_rn_satfinite(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.e2m1x2.rn.relu.satfinite"]
    fn llvm_ff_to_e2m1x2_rn_relu_satfinite(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.e2m3x2.rn.satfinite"]
    fn llvm_ff_to_e2m3x2_rn_satfinite(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.e2m3x2.rn.relu.satfinite"]
    fn llvm_ff_to_e2m3x2_rn_relu_satfinite(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.e3m2x2.rn.satfinite"]
    fn llvm_ff_to_e3m2x2_rn_satfinite(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.e3m2x2.rn.relu.satfinite"]
    fn llvm_ff_to_e3m2x2_rn_relu_satfinite(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.f16x2.to.e4m3x2.rn"]
    fn llvm_f16x2_to_e4m3x2_rn(a: f16x2) -> u16;
    #[link_name = "llvm.nvvm.f16x2.to.e4m3x2.rn.relu"]
    fn llvm_f16x2_to_e4m3x2_rn_relu(a: f16x2) -> u16;
    #[link_name = "llvm.nvvm.f16x2.to.e5m2x2.rn"]
    fn llvm_f16x2_to_e5m2x2_rn(a: f16x2) -> u16;
    #[link_name = "llvm.nvvm.f16x2.to.e5m2x2.rn.relu"]
    fn llvm_f16x2_to_e5m2x2_rn_relu(a: f16x2) -> u16;
    #[link_name = "llvm.nvvm.f16x2.to.e2m1x2.rn.satfinite"]
    fn llvm_f16x2_to_e2m1x2_rn_satfinite(a: f16x2) -> u16;
    #[link_name = "llvm.nvvm.f16x2.to.e2m1x2.rn.relu.satfinite"]
    fn llvm_f16x2_to_e2m1x2_rn_relu_satfinite(a: f16x2) -> u16;
    #[link_name = "llvm.nvvm.f16x2.to.e2m3x2.rn.satfinite"]
    fn llvm_f16x2_to_e2m3x2_rn_satfinite(a: f16x2) -> u16;
    #[link_name = "llvm.nvvm.f16x2.to.e2m3x2.rn.relu.satfinite"]
    fn llvm_f16x2_to_e2m3x2_rn_relu_satfinite(a: f16x2) -> u16;
    #[link_name = "llvm.nvvm.f16x2.to.e3m2x2.rn.satfinite"]
    fn llvm_f16x2_to_e3m2x2_rn_satfinite(a: f16x2) -> u16;
    #[link_name = "llvm.nvvm.f16x2.to.e3m2x2.rn.relu.satfinite"]
    fn llvm_f16x2_to_e3m2x2_rn_relu_satfinite(a: f16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.e4m3x2.rn.satfinite"]
    fn llvm_bf16x2_to_e4m3x2_rn_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.e4m3x2.rn.relu.satfinite"]
    fn llvm_bf16x2_to_e4m3x2_rn_relu_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.e5m2x2.rn.satfinite"]
    fn llvm_bf16x2_to_e5m2x2_rn_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.e5m2x2.rn.relu.satfinite"]
    fn llvm_bf16x2_to_e5m2x2_rn_relu_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.e2m1x2.rn.satfinite"]
    fn llvm_bf16x2_to_e2m1x2_rn_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.e2m1x2.rn.relu.satfinite"]
    fn llvm_bf16x2_to_e2m1x2_rn_relu_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.e2m3x2.rn.satfinite"]
    fn llvm_bf16x2_to_e2m3x2_rn_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.e2m3x2.rn.relu.satfinite"]
    fn llvm_bf16x2_to_e2m3x2_rn_relu_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.e3m2x2.rn.satfinite"]
    fn llvm_bf16x2_to_e3m2x2_rn_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.e3m2x2.rn.relu.satfinite"]
    fn llvm_bf16x2_to_e3m2x2_rn_relu_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.e4m3x2.to.f16x2.rn"]
    fn llvm_e4m3x2_to_f16x2_rn(a: u16) -> f16x2;
    #[link_name = "llvm.nvvm.e4m3x2.to.f16x2.rn.relu"]
    fn llvm_e4m3x2_to_f16x2_rn_relu(a: u16) -> f16x2;
    #[link_name = "llvm.nvvm.e5m2x2.to.f16x2.rn"]
    fn llvm_e5m2x2_to_f16x2_rn(a: u16) -> f16x2;
    #[link_name = "llvm.nvvm.e5m2x2.to.f16x2.rn.relu"]
    fn llvm_e5m2x2_to_f16x2_rn_relu(a: u16) -> f16x2;
    #[link_name = "llvm.nvvm.e2m1x2.to.f16x2.rn"]
    fn llvm_e2m1x2_to_f16x2_rn(a: u16) -> f16x2;
    #[link_name = "llvm.nvvm.e2m1x2.to.f16x2.rn.relu"]
    fn llvm_e2m1x2_to_f16x2_rn_relu(a: u16) -> f16x2;
    #[link_name = "llvm.nvvm.e2m3x2.to.f16x2.rn"]
    fn llvm_e2m3x2_to_f16x2_rn(a: u16) -> f16x2;
    #[link_name = "llvm.nvvm.e2m3x2.to.f16x2.rn.relu"]
    fn llvm_e2m3x2_to_f16x2_rn_relu(a: u16) -> f16x2;
    #[link_name = "llvm.nvvm.e3m2x2.to.f16x2.rn"]
    fn llvm_e3m2x2_to_f16x2_rn(a: u16) -> f16x2;
    #[link_name = "llvm.nvvm.e3m2x2.to.f16x2.rn.relu"]
    fn llvm_e3m2x2_to_f16x2_rn_relu(a: u16) -> f16x2;
    #[link_name = "llvm.nvvm.e2m1x2.to.bf16x2.rn.scale.n2.ue8m0"]
    fn llvm_e2m1x2_to_bf16x2_rn_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e2m1x2.to.bf16x2.rn.satfinite.scale.n2.ue8m0"]
    fn llvm_e2m1x2_to_bf16x2_rn_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e2m1x2.to.bf16x2.rn.relu.scale.n2.ue8m0"]
    fn llvm_e2m1x2_to_bf16x2_rn_relu_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e2m1x2.to.bf16x2.rn.relu.satfinite.scale.n2.ue8m0"]
    fn llvm_e2m1x2_to_bf16x2_rn_relu_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e2m3x2.to.bf16x2.rn.scale.n2.ue8m0"]
    fn llvm_e2m3x2_to_bf16x2_rn_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e2m3x2.to.bf16x2.rn.satfinite.scale.n2.ue8m0"]
    fn llvm_e2m3x2_to_bf16x2_rn_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e2m3x2.to.bf16x2.rn.relu.scale.n2.ue8m0"]
    fn llvm_e2m3x2_to_bf16x2_rn_relu_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e2m3x2.to.bf16x2.rn.relu.satfinite.scale.n2.ue8m0"]
    fn llvm_e2m3x2_to_bf16x2_rn_relu_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e3m2x2.to.bf16x2.rn.scale.n2.ue8m0"]
    fn llvm_e3m2x2_to_bf16x2_rn_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e3m2x2.to.bf16x2.rn.satfinite.scale.n2.ue8m0"]
    fn llvm_e3m2x2_to_bf16x2_rn_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e3m2x2.to.bf16x2.rn.relu.scale.n2.ue8m0"]
    fn llvm_e3m2x2_to_bf16x2_rn_relu_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e3m2x2.to.bf16x2.rn.relu.satfinite.scale.n2.ue8m0"]
    fn llvm_e3m2x2_to_bf16x2_rn_relu_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e4m3x2.to.bf16x2.rn.scale.n2.ue8m0"]
    fn llvm_e4m3x2_to_bf16x2_rn_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e4m3x2.to.bf16x2.rn.satfinite.scale.n2.ue8m0"]
    fn llvm_e4m3x2_to_bf16x2_rn_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e4m3x2.to.bf16x2.rn.relu.scale.n2.ue8m0"]
    fn llvm_e4m3x2_to_bf16x2_rn_relu_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e4m3x2.to.bf16x2.rn.relu.satfinite.scale.n2.ue8m0"]
    fn llvm_e4m3x2_to_bf16x2_rn_relu_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e5m2x2.to.bf16x2.rn.scale.n2.ue8m0"]
    fn llvm_e5m2x2_to_bf16x2_rn_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e5m2x2.to.bf16x2.rn.satfinite.scale.n2.ue8m0"]
    fn llvm_e5m2x2_to_bf16x2_rn_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e5m2x2.to.bf16x2.rn.relu.scale.n2.ue8m0"]
    fn llvm_e5m2x2_to_bf16x2_rn_relu_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.e5m2x2.to.bf16x2.rn.relu.satfinite.scale.n2.ue8m0"]
    fn llvm_e5m2x2_to_bf16x2_rn_relu_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.s2f6x2.to.bf16x2.rn.scale.n2.ue8m0"]
    fn llvm_s2f6x2_to_bf16x2_rn_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.s2f6x2.to.bf16x2.rn.satfinite.scale.n2.ue8m0"]
    fn llvm_s2f6x2_to_bf16x2_rn_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.s2f6x2.to.bf16x2.rn.relu.scale.n2.ue8m0"]
    fn llvm_s2f6x2_to_bf16x2_rn_relu_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.s2f6x2.to.bf16x2.rn.relu.satfinite.scale.n2.ue8m0"]
    fn llvm_s2f6x2_to_bf16x2_rn_relu_satfinite_scale(a: u16, scale: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.ff.to.ue8m0x2.rz"]
    fn llvm_ff_to_ue8m0x2_rz(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.ue8m0x2.rz.satfinite"]
    fn llvm_ff_to_ue8m0x2_rz_satfinite(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.ue8m0x2.rp"]
    fn llvm_ff_to_ue8m0x2_rp(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.ff.to.ue8m0x2.rp.satfinite"]
    fn llvm_ff_to_ue8m0x2_rp_satfinite(a: f32, b: f32) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.ue8m0x2.rz"]
    fn llvm_bf16x2_to_ue8m0x2_rz(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.ue8m0x2.rz.satfinite"]
    fn llvm_bf16x2_to_ue8m0x2_rz_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.ue8m0x2.rp"]
    fn llvm_bf16x2_to_ue8m0x2_rp(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.bf16x2.to.ue8m0x2.rp.satfinite"]
    fn llvm_bf16x2_to_ue8m0x2_rp_satfinite(a: bf16x2) -> u16;
    #[link_name = "llvm.nvvm.ue8m0x2.to.bf16x2"]
    fn llvm_ue8m0x2_to_bf16x2(a: u16) -> bf16x2;
    #[link_name = "llvm.nvvm.f32x4.to.e2m1x4.rs.satfinite"]
    fn llvm_f32x4_to_e2m1x4_rs_satfinite(a: f32x4, rbits: u32) -> u16;
    #[link_name = "llvm.nvvm.f32x4.to.e2m1x4.rs.relu.satfinite"]
    fn llvm_f32x4_to_e2m1x4_rs_relu_satfinite(a: f32x4, rbits: u32) -> u16;
    #[link_name = "llvm.nvvm.f32x4.to.e2m3x4.rs.satfinite"]
    fn llvm_f32x4_to_e2m3x4_rs_satfinite(a: f32x4, rbits: u32) -> i8x4;
    #[link_name = "llvm.nvvm.f32x4.to.e2m3x4.rs.relu.satfinite"]
    fn llvm_f32x4_to_e2m3x4_rs_relu_satfinite(a: f32x4, rbits: u32) -> i8x4;
    #[link_name = "llvm.nvvm.f32x4.to.e3m2x4.rs.satfinite"]
    fn llvm_f32x4_to_e3m2x4_rs_satfinite(a: f32x4, rbits: u32) -> i8x4;
    #[link_name = "llvm.nvvm.f32x4.to.e3m2x4.rs.relu.satfinite"]
    fn llvm_f32x4_to_e3m2x4_rs_relu_satfinite(a: f32x4, rbits: u32) -> i8x4;
    #[link_name = "llvm.nvvm.f32x4.to.e4m3x4.rs.satfinite"]
    fn llvm_f32x4_to_e4m3x4_rs_satfinite(a: f32x4, rbits: u32) -> i8x4;
    #[link_name = "llvm.nvvm.f32x4.to.e4m3x4.rs.relu.satfinite"]
    fn llvm_f32x4_to_e4m3x4_rs_relu_satfinite(a: f32x4, rbits: u32) -> i8x4;
    #[link_name = "llvm.nvvm.f32x4.to.e5m2x4.rs.satfinite"]
    fn llvm_f32x4_to_e5m2x4_rs_satfinite(a: f32x4, rbits: u32) -> i8x4;
    #[link_name = "llvm.nvvm.f32x4.to.e5m2x4.rs.relu.satfinite"]
    fn llvm_f32x4_to_e5m2x4_rs_relu_satfinite(a: f32x4, rbits: u32) -> i8x4;
}

/// Converts two `f32` values to packed `e4m3x2` values, rounding to nearest even.
/// `a` is stored in the upper half of the result and `b` in the lower half. `RELU` clamps
/// negative results to zero.
///
/// Requires `sm_89` with PTX ISA 8.1, or `sm_90` with PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e4m3x2_f32<const RELU: bool>(a: f32, b: f32) -> u16 {
    match RELU {
        false => llvm_ff_to_e4m3x2_rn(a, b),
        true => llvm_ff_to_e4m3x2_rn_relu(a, b),
    }
}

/// Converts two `f32` values to packed `e5m2x2` values, rounding to nearest even.
/// `a` is stored in the upper half of the result and `b` in the lower half. `RELU` clamps
/// negative results to zero.
///
/// Requires `sm_89` with PTX ISA 8.1, or `sm_90` with PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e5m2x2_f32<const RELU: bool>(a: f32, b: f32) -> u16 {
    match RELU {
        false => llvm_ff_to_e5m2x2_rn(a, b),
        true => llvm_ff_to_e5m2x2_rn_relu(a, b),
    }
}

/// Converts two `f32` values to packed `e2m1x2` values, rounding to nearest even.
/// `a` is stored in the upper half of the result and `b` in the lower half. `RELU` clamps
/// negative results to zero.
///
/// Requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.6; `sm_100f`, `sm_101f` or `sm_120f`
/// with PTX ISA 8.8; or `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e2m1x2_f32<const RELU: bool>(a: f32, b: f32) -> u16 {
    match RELU {
        false => llvm_ff_to_e2m1x2_rn_satfinite(a, b),
        true => llvm_ff_to_e2m1x2_rn_relu_satfinite(a, b),
    }
}

/// Converts two `f32` values to packed `e2m3x2` values, rounding to nearest even.
/// `a` is stored in the upper half of the result and `b` in the lower half. `RELU` clamps
/// negative results to zero.
///
/// Requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.6; `sm_100f`, `sm_101f` or `sm_120f`
/// with PTX ISA 8.8; or `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e2m3x2_f32<const RELU: bool>(a: f32, b: f32) -> u16 {
    match RELU {
        false => llvm_ff_to_e2m3x2_rn_satfinite(a, b),
        true => llvm_ff_to_e2m3x2_rn_relu_satfinite(a, b),
    }
}

/// Converts two `f32` values to packed `e3m2x2` values, rounding to nearest even.
/// `a` is stored in the upper half of the result and `b` in the lower half. `RELU` clamps
/// negative results to zero.
///
/// Requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.6; `sm_100f`, `sm_101f` or `sm_120f`
/// with PTX ISA 8.8; or `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e3m2x2_f32<const RELU: bool>(a: f32, b: f32) -> u16 {
    match RELU {
        false => llvm_ff_to_e3m2x2_rn_satfinite(a, b),
        true => llvm_ff_to_e3m2x2_rn_relu_satfinite(a, b),
    }
}

/// Converts a packed `f16x2` value to packed `e4m3x2` values, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_89` with PTX ISA 8.1, or `sm_90` with PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e4m3x2_f16x2<const RELU: bool>(a: f16x2) -> u16 {
    match RELU {
        false => llvm_f16x2_to_e4m3x2_rn(a),
        true => llvm_f16x2_to_e4m3x2_rn_relu(a),
    }
}

/// Converts a packed `f16x2` value to packed `e5m2x2` values, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_89` with PTX ISA 8.1, or `sm_90` with PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e5m2x2_f16x2<const RELU: bool>(a: f16x2) -> u16 {
    match RELU {
        false => llvm_f16x2_to_e5m2x2_rn(a),
        true => llvm_f16x2_to_e5m2x2_rn_relu(a),
    }
}

/// Converts a packed `f16x2` value to packed `e2m1x2` values, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e2m1x2_f16x2<const RELU: bool>(a: f16x2) -> u16 {
    match RELU {
        false => llvm_f16x2_to_e2m1x2_rn_satfinite(a),
        true => llvm_f16x2_to_e2m1x2_rn_relu_satfinite(a),
    }
}

/// Converts a packed `f16x2` value to packed `e2m3x2` values, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e2m3x2_f16x2<const RELU: bool>(a: f16x2) -> u16 {
    match RELU {
        false => llvm_f16x2_to_e2m3x2_rn_satfinite(a),
        true => llvm_f16x2_to_e2m3x2_rn_relu_satfinite(a),
    }
}

/// Converts a packed `f16x2` value to packed `e3m2x2` values, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e3m2x2_f16x2<const RELU: bool>(a: f16x2) -> u16 {
    match RELU {
        false => llvm_f16x2_to_e3m2x2_rn_satfinite(a),
        true => llvm_f16x2_to_e3m2x2_rn_relu_satfinite(a),
    }
}

/// Converts a packed `bf16x2` value to packed `e4m3x2` values, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e4m3x2_bf16x2<const RELU: bool>(a: bf16x2) -> u16 {
    match RELU {
        false => llvm_bf16x2_to_e4m3x2_rn_satfinite(a),
        true => llvm_bf16x2_to_e4m3x2_rn_relu_satfinite(a),
    }
}

/// Converts a packed `bf16x2` value to packed `e5m2x2` values, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e5m2x2_bf16x2<const RELU: bool>(a: bf16x2) -> u16 {
    match RELU {
        false => llvm_bf16x2_to_e5m2x2_rn_satfinite(a),
        true => llvm_bf16x2_to_e5m2x2_rn_relu_satfinite(a),
    }
}

/// Converts a packed `bf16x2` value to packed `e2m1x2` values, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e2m1x2_bf16x2<const RELU: bool>(a: bf16x2) -> u16 {
    match RELU {
        false => llvm_bf16x2_to_e2m1x2_rn_satfinite(a),
        true => llvm_bf16x2_to_e2m1x2_rn_relu_satfinite(a),
    }
}

/// Converts a packed `bf16x2` value to packed `e2m3x2` values, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e2m3x2_bf16x2<const RELU: bool>(a: bf16x2) -> u16 {
    match RELU {
        false => llvm_bf16x2_to_e2m3x2_rn_satfinite(a),
        true => llvm_bf16x2_to_e2m3x2_rn_relu_satfinite(a),
    }
}

/// Converts a packed `bf16x2` value to packed `e3m2x2` values, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_e3m2x2_bf16x2<const RELU: bool>(a: bf16x2) -> u16 {
    match RELU {
        false => llvm_bf16x2_to_e3m2x2_rn_satfinite(a),
        true => llvm_bf16x2_to_e3m2x2_rn_relu_satfinite(a),
    }
}

/// Converts packed `e4m3x2` values to a packed `f16x2` value, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_89` with PTX ISA 8.1, or `sm_90` with PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f16x2_e4m3x2<const RELU: bool>(a: u16) -> f16x2 {
    match RELU {
        false => llvm_e4m3x2_to_f16x2_rn(a),
        true => llvm_e4m3x2_to_f16x2_rn_relu(a),
    }
}

/// Converts packed `e5m2x2` values to a packed `f16x2` value, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_89` with PTX ISA 8.1, or `sm_90` with PTX ISA 7.8.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f16x2_e5m2x2<const RELU: bool>(a: u16) -> f16x2 {
    match RELU {
        false => llvm_e5m2x2_to_f16x2_rn(a),
        true => llvm_e5m2x2_to_f16x2_rn_relu(a),
    }
}

/// Converts packed `e2m1x2` values to a packed `f16x2` value, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.6; `sm_100f`, `sm_101f` or `sm_120f`
/// with PTX ISA 8.8; or `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f16x2_e2m1x2<const RELU: bool>(a: u16) -> f16x2 {
    match RELU {
        false => llvm_e2m1x2_to_f16x2_rn(a),
        true => llvm_e2m1x2_to_f16x2_rn_relu(a),
    }
}

/// Converts packed `e2m3x2` values to a packed `f16x2` value, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.6; `sm_100f`, `sm_101f` or `sm_120f`
/// with PTX ISA 8.8; or `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f16x2_e2m3x2<const RELU: bool>(a: u16) -> f16x2 {
    match RELU {
        false => llvm_e2m3x2_to_f16x2_rn(a),
        true => llvm_e2m3x2_to_f16x2_rn_relu(a),
    }
}

/// Converts packed `e3m2x2` values to a packed `f16x2` value, rounding to nearest even.
/// `RELU` clamps negative results to zero.
///
/// Requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.6; `sm_100f`, `sm_101f` or `sm_120f`
/// with PTX ISA 8.8; or `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f16x2_e3m2x2<const RELU: bool>(a: u16) -> f16x2 {
    match RELU {
        false => llvm_e3m2x2_to_f16x2_rn(a),
        true => llvm_e3m2x2_to_f16x2_rn_relu(a),
    }
}

/// Converts packed `e2m1x2` values to a packed `bf16x2` value, applying the `ue8m0` scale
/// factor `scale` (`.scale::n2::ue8m0`). `RELU` clamps negative results to zero and
/// `SATFINITE` clamps out-of-range results to the finite range.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_scale_bf16x2_e2m1x2<const RELU: bool, const SATFINITE: bool>(
    a: u16,
    scale: u16,
) -> bf16x2 {
    match (RELU, SATFINITE) {
        (false, false) => llvm_e2m1x2_to_bf16x2_rn_scale(a, scale),
        (false, true) => llvm_e2m1x2_to_bf16x2_rn_satfinite_scale(a, scale),
        (true, false) => llvm_e2m1x2_to_bf16x2_rn_relu_scale(a, scale),
        (true, true) => llvm_e2m1x2_to_bf16x2_rn_relu_satfinite_scale(a, scale),
    }
}

/// Converts packed `e2m3x2` values to a packed `bf16x2` value, applying the `ue8m0` scale
/// factor `scale` (`.scale::n2::ue8m0`). `RELU` clamps negative results to zero and
/// `SATFINITE` clamps out-of-range results to the finite range.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_scale_bf16x2_e2m3x2<const RELU: bool, const SATFINITE: bool>(
    a: u16,
    scale: u16,
) -> bf16x2 {
    match (RELU, SATFINITE) {
        (false, false) => llvm_e2m3x2_to_bf16x2_rn_scale(a, scale),
        (false, true) => llvm_e2m3x2_to_bf16x2_rn_satfinite_scale(a, scale),
        (true, false) => llvm_e2m3x2_to_bf16x2_rn_relu_scale(a, scale),
        (true, true) => llvm_e2m3x2_to_bf16x2_rn_relu_satfinite_scale(a, scale),
    }
}

/// Converts packed `e3m2x2` values to a packed `bf16x2` value, applying the `ue8m0` scale
/// factor `scale` (`.scale::n2::ue8m0`). `RELU` clamps negative results to zero and
/// `SATFINITE` clamps out-of-range results to the finite range.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_scale_bf16x2_e3m2x2<const RELU: bool, const SATFINITE: bool>(
    a: u16,
    scale: u16,
) -> bf16x2 {
    match (RELU, SATFINITE) {
        (false, false) => llvm_e3m2x2_to_bf16x2_rn_scale(a, scale),
        (false, true) => llvm_e3m2x2_to_bf16x2_rn_satfinite_scale(a, scale),
        (true, false) => llvm_e3m2x2_to_bf16x2_rn_relu_scale(a, scale),
        (true, true) => llvm_e3m2x2_to_bf16x2_rn_relu_satfinite_scale(a, scale),
    }
}

/// Converts packed `e4m3x2` values to a packed `bf16x2` value, applying the `ue8m0` scale
/// factor `scale` (`.scale::n2::ue8m0`). `RELU` clamps negative results to zero and
/// `SATFINITE` clamps out-of-range results to the finite range.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_scale_bf16x2_e4m3x2<const RELU: bool, const SATFINITE: bool>(
    a: u16,
    scale: u16,
) -> bf16x2 {
    match (RELU, SATFINITE) {
        (false, false) => llvm_e4m3x2_to_bf16x2_rn_scale(a, scale),
        (false, true) => llvm_e4m3x2_to_bf16x2_rn_satfinite_scale(a, scale),
        (true, false) => llvm_e4m3x2_to_bf16x2_rn_relu_scale(a, scale),
        (true, true) => llvm_e4m3x2_to_bf16x2_rn_relu_satfinite_scale(a, scale),
    }
}

/// Converts packed `e5m2x2` values to a packed `bf16x2` value, applying the `ue8m0` scale
/// factor `scale` (`.scale::n2::ue8m0`). `RELU` clamps negative results to zero and
/// `SATFINITE` clamps out-of-range results to the finite range.
///
/// Requires `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.2.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_scale_bf16x2_e5m2x2<const RELU: bool, const SATFINITE: bool>(
    a: u16,
    scale: u16,
) -> bf16x2 {
    match (RELU, SATFINITE) {
        (false, false) => llvm_e5m2x2_to_bf16x2_rn_scale(a, scale),
        (false, true) => llvm_e5m2x2_to_bf16x2_rn_satfinite_scale(a, scale),
        (true, false) => llvm_e5m2x2_to_bf16x2_rn_relu_scale(a, scale),
        (true, true) => llvm_e5m2x2_to_bf16x2_rn_relu_satfinite_scale(a, scale),
    }
}

/// Converts packed `s2f6x2` values to a packed `bf16x2` value, applying the `ue8m0` scale
/// factor `scale` (`.scale::n2::ue8m0`). `RELU` clamps negative results to zero and
/// `SATFINITE` clamps out-of-range results to the finite range.
///
/// Requires `sm_100a`, `sm_103a`, `sm_110a`, `sm_120a` or `sm_121a` with PTX ISA 9.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_scale_bf16x2_s2f6x2<const RELU: bool, const SATFINITE: bool>(
    a: u16,
    scale: u16,
) -> bf16x2 {
    match (RELU, SATFINITE) {
        (false, false) => llvm_s2f6x2_to_bf16x2_rn_scale(a, scale),
        (false, true) => llvm_s2f6x2_to_bf16x2_rn_satfinite_scale(a, scale),
        (true, false) => llvm_s2f6x2_to_bf16x2_rn_relu_scale(a, scale),
        (true, true) => llvm_s2f6x2_to_bf16x2_rn_relu_satfinite_scale(a, scale),
    }
}

/// Converts a pair of `f32` values to packed `ue8m0x2` scale factors. `a` is stored in the
/// upper half of the result and `b` in the lower half. Only `Rounding::Rz` and
/// `Rounding::Rp` are supported.
///
/// Requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.6; `sm_100f`, `sm_101f` or `sm_120f`
/// with PTX ISA 8.8; or `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_ue8m0x2_f32<const RND: Rounding, const SATFINITE: bool>(a: f32, b: f32) -> u16 {
    static_assert!(
        matches!(RND, Rounding::Rz | Rounding::Rp),
        "cvt_ue8m0x2_f32 only supports the Rz and Rp rounding modes"
    );
    match (RND, SATFINITE) {
        (Rounding::Rz, false) => llvm_ff_to_ue8m0x2_rz(a, b),
        (Rounding::Rz, true) => llvm_ff_to_ue8m0x2_rz_satfinite(a, b),
        (Rounding::Rp, false) => llvm_ff_to_ue8m0x2_rp(a, b),
        (Rounding::Rp, true) => llvm_ff_to_ue8m0x2_rp_satfinite(a, b),
        (Rounding::Rn | Rounding::Rm, _) => unreachable!(),
    }
}

/// Converts a packed `bf16x2` value to packed `ue8m0x2` scale factors. Only
/// `Rounding::Rz` and `Rounding::Rp` are supported.
///
/// Requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.6; `sm_100f`, `sm_101f` or `sm_120f`
/// with PTX ISA 8.8; or `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_ue8m0x2_bf16x2<const RND: Rounding, const SATFINITE: bool>(a: bf16x2) -> u16 {
    static_assert!(
        matches!(RND, Rounding::Rz | Rounding::Rp),
        "cvt_ue8m0x2_bf16x2 only supports the Rz and Rp rounding modes"
    );
    match (RND, SATFINITE) {
        (Rounding::Rz, false) => llvm_bf16x2_to_ue8m0x2_rz(a),
        (Rounding::Rz, true) => llvm_bf16x2_to_ue8m0x2_rz_satfinite(a),
        (Rounding::Rp, false) => llvm_bf16x2_to_ue8m0x2_rp(a),
        (Rounding::Rp, true) => llvm_bf16x2_to_ue8m0x2_rp_satfinite(a),
        (Rounding::Rn | Rounding::Rm, _) => unreachable!(),
    }
}

/// Converts packed `ue8m0x2` scale factors to a packed `bf16x2` value.
///
/// Requires `sm_100a`, `sm_101a` or `sm_120a` with PTX ISA 8.6; `sm_100f`, `sm_101f` or `sm_120f`
/// with PTX ISA 8.8; or `sm_100f`, `sm_110f` or `sm_120f` with PTX ISA 9.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_bf16x2_ue8m0x2(a: u16) -> bf16x2 {
    llvm_ue8m0x2_to_bf16x2(a)
}

/// Converts four `f32` values to packed `e2m1x4` values with stochastic rounding, using
/// `rbits` as the random bits. The values `a`, `b`, `c` and `d` are converted in PTX
/// operand order. `RELU` clamps negative results to zero.
///
/// Requires `sm_100a` or `sm_103a` with PTX ISA 8.7.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_rs_e2m1x4_f32<const RELU: bool>(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    rbits: u32,
) -> u16 {
    let x = f32x4::new(a, b, c, d);
    match RELU {
        false => llvm_f32x4_to_e2m1x4_rs_satfinite(x, rbits),
        true => llvm_f32x4_to_e2m1x4_rs_relu_satfinite(x, rbits),
    }
}

/// Converts four `f32` values to packed `e2m3x4` values with stochastic rounding, using
/// `rbits` as the random bits. The values `a`, `b`, `c` and `d` are converted in PTX
/// operand order. `RELU` clamps negative results to zero.
///
/// Requires `sm_100a` or `sm_103a` with PTX ISA 8.7.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_rs_e2m3x4_f32<const RELU: bool>(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    rbits: u32,
) -> u32 {
    let x = f32x4::new(a, b, c, d);
    match RELU {
        false => transmute(llvm_f32x4_to_e2m3x4_rs_satfinite(x, rbits)),
        true => transmute(llvm_f32x4_to_e2m3x4_rs_relu_satfinite(x, rbits)),
    }
}

/// Converts four `f32` values to packed `e3m2x4` values with stochastic rounding, using
/// `rbits` as the random bits. The values `a`, `b`, `c` and `d` are converted in PTX
/// operand order. `RELU` clamps negative results to zero.
///
/// Requires `sm_100a` or `sm_103a` with PTX ISA 8.7.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_rs_e3m2x4_f32<const RELU: bool>(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    rbits: u32,
) -> u32 {
    let x = f32x4::new(a, b, c, d);
    match RELU {
        false => transmute(llvm_f32x4_to_e3m2x4_rs_satfinite(x, rbits)),
        true => transmute(llvm_f32x4_to_e3m2x4_rs_relu_satfinite(x, rbits)),
    }
}

/// Converts four `f32` values to packed `e4m3x4` values with stochastic rounding, using
/// `rbits` as the random bits. The values `a`, `b`, `c` and `d` are converted in PTX
/// operand order. `RELU` clamps negative results to zero.
///
/// Requires `sm_100a` or `sm_103a` with PTX ISA 8.7.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_rs_e4m3x4_f32<const RELU: bool>(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    rbits: u32,
) -> u32 {
    let x = f32x4::new(a, b, c, d);
    match RELU {
        false => transmute(llvm_f32x4_to_e4m3x4_rs_satfinite(x, rbits)),
        true => transmute(llvm_f32x4_to_e4m3x4_rs_relu_satfinite(x, rbits)),
    }
}

/// Converts four `f32` values to packed `e5m2x4` values with stochastic rounding, using
/// `rbits` as the random bits. The values `a`, `b`, `c` and `d` are converted in PTX
/// operand order. `RELU` clamps negative results to zero.
///
/// Requires `sm_100a` or `sm_103a` with PTX ISA 8.7.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_rs_e5m2x4_f32<const RELU: bool>(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    rbits: u32,
) -> u32 {
    let x = f32x4::new(a, b, c, d);
    match RELU {
        false => transmute(llvm_f32x4_to_e5m2x4_rs_satfinite(x, rbits)),
        true => transmute(llvm_f32x4_to_e5m2x4_rs_relu_satfinite(x, rbits)),
    }
}
