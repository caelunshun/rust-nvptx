// --- LLM-generated --- //
//! Conversions to 16-bit floating-point types and `tf32`.

use super::modifiers::Rounding;
use super::packed::*;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.f2h.rn"]
    fn llvm_f2h_rn(a: f32) -> u16;
    #[link_name = "llvm.nvvm.f2h.rn.ftz"]
    fn llvm_f2h_rn_ftz(a: f32) -> u16;
    #[link_name = "llvm.nvvm.f2f16.rn"]
    fn llvm_f2f16_rn(a: f32) -> f16;
    #[link_name = "llvm.nvvm.f2f16.rn.relu"]
    fn llvm_f2f16_rn_relu(a: f32) -> f16;
    #[link_name = "llvm.nvvm.f2f16.rn.satfinite"]
    fn llvm_f2f16_rn_satfinite(a: f32) -> f16;
    #[link_name = "llvm.nvvm.f2f16.rn.relu.satfinite"]
    fn llvm_f2f16_rn_relu_satfinite(a: f32) -> f16;
    #[link_name = "llvm.nvvm.f2f16.rz"]
    fn llvm_f2f16_rz(a: f32) -> f16;
    #[link_name = "llvm.nvvm.f2f16.rz.relu"]
    fn llvm_f2f16_rz_relu(a: f32) -> f16;
    #[link_name = "llvm.nvvm.f2f16.rz.satfinite"]
    fn llvm_f2f16_rz_satfinite(a: f32) -> f16;
    #[link_name = "llvm.nvvm.f2f16.rz.relu.satfinite"]
    fn llvm_f2f16_rz_relu_satfinite(a: f32) -> f16;
    #[link_name = "llvm.nvvm.f2bf16.rn"]
    fn llvm_f2bf16_rn(a: f32) -> bf16;
    #[link_name = "llvm.nvvm.f2bf16.rn.relu"]
    fn llvm_f2bf16_rn_relu(a: f32) -> bf16;
    #[link_name = "llvm.nvvm.f2bf16.rn.satfinite"]
    fn llvm_f2bf16_rn_satfinite(a: f32) -> bf16;
    #[link_name = "llvm.nvvm.f2bf16.rn.relu.satfinite"]
    fn llvm_f2bf16_rn_relu_satfinite(a: f32) -> bf16;
    #[link_name = "llvm.nvvm.f2bf16.rz"]
    fn llvm_f2bf16_rz(a: f32) -> bf16;
    #[link_name = "llvm.nvvm.f2bf16.rz.relu"]
    fn llvm_f2bf16_rz_relu(a: f32) -> bf16;
    #[link_name = "llvm.nvvm.f2bf16.rz.satfinite"]
    fn llvm_f2bf16_rz_satfinite(a: f32) -> bf16;
    #[link_name = "llvm.nvvm.f2bf16.rz.relu.satfinite"]
    fn llvm_f2bf16_rz_relu_satfinite(a: f32) -> bf16;
    #[link_name = "llvm.nvvm.ff2f16x2.rn"]
    fn llvm_ff2f16x2_rn(a: f32, b: f32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rn.relu"]
    fn llvm_ff2f16x2_rn_relu(a: f32, b: f32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rn.satfinite"]
    fn llvm_ff2f16x2_rn_satfinite(a: f32, b: f32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rn.relu.satfinite"]
    fn llvm_ff2f16x2_rn_relu_satfinite(a: f32, b: f32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rz"]
    fn llvm_ff2f16x2_rz(a: f32, b: f32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rz.relu"]
    fn llvm_ff2f16x2_rz_relu(a: f32, b: f32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rz.satfinite"]
    fn llvm_ff2f16x2_rz_satfinite(a: f32, b: f32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rz.relu.satfinite"]
    fn llvm_ff2f16x2_rz_relu_satfinite(a: f32, b: f32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rs"]
    fn llvm_ff2f16x2_rs(a: f32, b: f32, rbits: u32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rs.relu"]
    fn llvm_ff2f16x2_rs_relu(a: f32, b: f32, rbits: u32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rs.satfinite"]
    fn llvm_ff2f16x2_rs_satfinite(a: f32, b: f32, rbits: u32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2f16x2.rs.relu.satfinite"]
    fn llvm_ff2f16x2_rs_relu_satfinite(a: f32, b: f32, rbits: u32) -> f16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rn"]
    fn llvm_ff2bf16x2_rn(a: f32, b: f32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rn.relu"]
    fn llvm_ff2bf16x2_rn_relu(a: f32, b: f32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rn.satfinite"]
    fn llvm_ff2bf16x2_rn_satfinite(a: f32, b: f32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rn.relu.satfinite"]
    fn llvm_ff2bf16x2_rn_relu_satfinite(a: f32, b: f32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rz"]
    fn llvm_ff2bf16x2_rz(a: f32, b: f32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rz.relu"]
    fn llvm_ff2bf16x2_rz_relu(a: f32, b: f32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rz.satfinite"]
    fn llvm_ff2bf16x2_rz_satfinite(a: f32, b: f32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rz.relu.satfinite"]
    fn llvm_ff2bf16x2_rz_relu_satfinite(a: f32, b: f32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rs"]
    fn llvm_ff2bf16x2_rs(a: f32, b: f32, rbits: u32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rs.relu"]
    fn llvm_ff2bf16x2_rs_relu(a: f32, b: f32, rbits: u32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rs.satfinite"]
    fn llvm_ff2bf16x2_rs_satfinite(a: f32, b: f32, rbits: u32) -> bf16x2;
    #[link_name = "llvm.nvvm.ff2bf16x2.rs.relu.satfinite"]
    fn llvm_ff2bf16x2_rs_relu_satfinite(a: f32, b: f32, rbits: u32) -> bf16x2;
    #[link_name = "llvm.nvvm.f2tf32.rn"]
    fn llvm_f2tf32_rn(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2tf32.rn.relu"]
    fn llvm_f2tf32_rn_relu(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2tf32.rn.satfinite"]
    fn llvm_f2tf32_rn_satfinite(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2tf32.rn.relu.satfinite"]
    fn llvm_f2tf32_rn_relu_satfinite(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2tf32.rz"]
    fn llvm_f2tf32_rz(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2tf32.rz.relu"]
    fn llvm_f2tf32_rz_relu(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2tf32.rz.satfinite"]
    fn llvm_f2tf32_rz_satfinite(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2tf32.rz.relu.satfinite"]
    fn llvm_f2tf32_rz_relu_satfinite(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2tf32.rna"]
    fn llvm_f2tf32_rna(a: f32) -> u32;
    #[link_name = "llvm.nvvm.f2tf32.rna.satfinite"]
    fn llvm_f2tf32_rna_satfinite(a: f32) -> u32;
}

/// Converts an `f32` to `f16`.
///
/// `RND` selects the rounding mode and must be `Rn` or `Rz`. `FTZ` flushes subnormal results to
/// zero and is only supported with `Rn`, without `RELU` and `SATFINITE`. `RELU` clamps negative
/// results to zero. `SATFINITE` clamps the result to the finite range of `f16`.
///
/// `SATFINITE` requires `sm_80` and PTX ISA 8.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f16_f32<
    const RND: Rounding,
    const FTZ: bool,
    const RELU: bool,
    const SATFINITE: bool,
>(
    a: f32,
) -> f16 {
    static_assert!(
        matches!(RND, Rounding::Rn | Rounding::Rz),
        "cvt_f16_f32 only supports Rn and Rz rounding"
    );
    static_assert!(
        !FTZ || (matches!(RND, Rounding::Rn) && !RELU && !SATFINITE),
        "cvt_f16_f32 only supports FTZ with Rn rounding and without RELU or SATFINITE"
    );
    match (RND, FTZ, RELU, SATFINITE) {
        (Rounding::Rn, false, false, false) => f16::from_bits(llvm_f2h_rn(a)),
        (Rounding::Rn, true, false, false) => f16::from_bits(llvm_f2h_rn_ftz(a)),
        (Rounding::Rn, false, true, false) => llvm_f2f16_rn_relu(a),
        (Rounding::Rn, false, false, true) => llvm_f2f16_rn_satfinite(a),
        (Rounding::Rn, false, true, true) => llvm_f2f16_rn_relu_satfinite(a),
        (Rounding::Rz, false, false, false) => llvm_f2f16_rz(a),
        (Rounding::Rz, false, true, false) => llvm_f2f16_rz_relu(a),
        (Rounding::Rz, false, false, true) => llvm_f2f16_rz_satfinite(a),
        (Rounding::Rz, false, true, true) => llvm_f2f16_rz_relu_satfinite(a),
        _ => unreachable!(),
    }
}

/// Converts an `f32` to `bf16`.
///
/// `RND` selects the rounding mode and must be `Rn` or `Rz`. `RELU` clamps negative results to
/// zero. `SATFINITE` clamps the result to the finite range of `bf16`.
///
/// `SATFINITE` requires `sm_80` and PTX ISA 8.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_bf16_f32<const RND: Rounding, const RELU: bool, const SATFINITE: bool>(
    a: f32,
) -> bf16 {
    static_assert!(
        matches!(RND, Rounding::Rn | Rounding::Rz),
        "cvt_bf16_f32 only supports Rn and Rz rounding"
    );
    match (RND, RELU, SATFINITE) {
        (Rounding::Rn, false, false) => llvm_f2bf16_rn(a),
        (Rounding::Rn, true, false) => llvm_f2bf16_rn_relu(a),
        (Rounding::Rn, false, true) => llvm_f2bf16_rn_satfinite(a),
        (Rounding::Rn, true, true) => llvm_f2bf16_rn_relu_satfinite(a),
        (Rounding::Rz, false, false) => llvm_f2bf16_rz(a),
        (Rounding::Rz, true, false) => llvm_f2bf16_rz_relu(a),
        (Rounding::Rz, false, true) => llvm_f2bf16_rz_satfinite(a),
        (Rounding::Rz, true, true) => llvm_f2bf16_rz_relu_satfinite(a),
        _ => unreachable!(),
    }
}

/// Converts two `f32` values to an `f16x2` vector, rounding with `RND`.
///
/// `a` is converted to the upper half and `b` to the lower half of the result. `RND` must be `Rn`
/// or `Rz`. `RELU` clamps negative results to zero. `SATFINITE` clamps each result to the finite
/// range of `f16`.
///
/// `SATFINITE` requires `sm_80` and PTX ISA 8.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_f16x2_f32<const RND: Rounding, const RELU: bool, const SATFINITE: bool>(
    a: f32,
    b: f32,
) -> f16x2 {
    static_assert!(
        matches!(RND, Rounding::Rn | Rounding::Rz),
        "cvt_f16x2_f32 only supports Rn and Rz rounding"
    );
    match (RND, RELU, SATFINITE) {
        (Rounding::Rn, false, false) => llvm_ff2f16x2_rn(a, b),
        (Rounding::Rn, true, false) => llvm_ff2f16x2_rn_relu(a, b),
        (Rounding::Rn, false, true) => llvm_ff2f16x2_rn_satfinite(a, b),
        (Rounding::Rn, true, true) => llvm_ff2f16x2_rn_relu_satfinite(a, b),
        (Rounding::Rz, false, false) => llvm_ff2f16x2_rz(a, b),
        (Rounding::Rz, true, false) => llvm_ff2f16x2_rz_relu(a, b),
        (Rounding::Rz, false, true) => llvm_ff2f16x2_rz_satfinite(a, b),
        (Rounding::Rz, true, true) => llvm_ff2f16x2_rz_relu_satfinite(a, b),
        _ => unreachable!(),
    }
}

/// Converts two `f32` values to an `f16x2` vector using stochastic rounding.
///
/// `a` is converted to the upper half and `b` to the lower half of the result. `rbits` supplies
/// the random bits that drive the rounding. `RELU` clamps negative results to zero. `SATFINITE`
/// clamps each result to the finite range of `f16`.
///
/// Requires `sm_100a` and PTX ISA 8.7, or `sm_103a` and PTX ISA 8.7.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_rs_f16x2_f32<const RELU: bool, const SATFINITE: bool>(
    a: f32,
    b: f32,
    rbits: u32,
) -> f16x2 {
    match (RELU, SATFINITE) {
        (false, false) => llvm_ff2f16x2_rs(a, b, rbits),
        (true, false) => llvm_ff2f16x2_rs_relu(a, b, rbits),
        (false, true) => llvm_ff2f16x2_rs_satfinite(a, b, rbits),
        (true, true) => llvm_ff2f16x2_rs_relu_satfinite(a, b, rbits),
    }
}

/// Converts two `f32` values to a `bf16x2` vector, rounding with `RND`.
///
/// `a` is converted to the upper half and `b` to the lower half of the result. `RND` must be `Rn`
/// or `Rz`. `RELU` clamps negative results to zero. `SATFINITE` clamps each result to the finite
/// range of `bf16`.
///
/// `SATFINITE` requires `sm_80` and PTX ISA 8.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_bf16x2_f32<const RND: Rounding, const RELU: bool, const SATFINITE: bool>(
    a: f32,
    b: f32,
) -> bf16x2 {
    static_assert!(
        matches!(RND, Rounding::Rn | Rounding::Rz),
        "cvt_bf16x2_f32 only supports Rn and Rz rounding"
    );
    match (RND, RELU, SATFINITE) {
        (Rounding::Rn, false, false) => llvm_ff2bf16x2_rn(a, b),
        (Rounding::Rn, true, false) => llvm_ff2bf16x2_rn_relu(a, b),
        (Rounding::Rn, false, true) => llvm_ff2bf16x2_rn_satfinite(a, b),
        (Rounding::Rn, true, true) => llvm_ff2bf16x2_rn_relu_satfinite(a, b),
        (Rounding::Rz, false, false) => llvm_ff2bf16x2_rz(a, b),
        (Rounding::Rz, true, false) => llvm_ff2bf16x2_rz_relu(a, b),
        (Rounding::Rz, false, true) => llvm_ff2bf16x2_rz_satfinite(a, b),
        (Rounding::Rz, true, true) => llvm_ff2bf16x2_rz_relu_satfinite(a, b),
        _ => unreachable!(),
    }
}

/// Converts two `f32` values to a `bf16x2` vector using stochastic rounding.
///
/// `a` is converted to the upper half and `b` to the lower half of the result. `rbits` supplies
/// the random bits that drive the rounding. `RELU` clamps negative results to zero. `SATFINITE`
/// clamps each result to the finite range of `bf16`.
///
/// Requires `sm_100a` and PTX ISA 8.7, or `sm_103a` and PTX ISA 8.7.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_rs_bf16x2_f32<const RELU: bool, const SATFINITE: bool>(
    a: f32,
    b: f32,
    rbits: u32,
) -> bf16x2 {
    match (RELU, SATFINITE) {
        (false, false) => llvm_ff2bf16x2_rs(a, b, rbits),
        (true, false) => llvm_ff2bf16x2_rs_relu(a, b, rbits),
        (false, true) => llvm_ff2bf16x2_rs_satfinite(a, b, rbits),
        (true, true) => llvm_ff2bf16x2_rs_relu_satfinite(a, b, rbits),
    }
}

/// Converts an `f32` to the `tf32` format, returning its bit pattern.
///
/// `RND` selects the rounding mode and must be `Rn` or `Rz`. `RELU` clamps negative results to
/// zero. `SATFINITE` clamps the result to the finite range of `tf32`.
///
/// Requires `sm_90` and PTX ISA 7.8. `SATFINITE` additionally requires `sm_100` and PTX ISA 8.6.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[target_feature(enable = "sm_90,ptx78")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_tf32_f32<const RND: Rounding, const RELU: bool, const SATFINITE: bool>(
    a: f32,
) -> u32 {
    static_assert!(
        matches!(RND, Rounding::Rn | Rounding::Rz),
        "cvt_tf32_f32 only supports Rn and Rz rounding"
    );
    match (RND, RELU, SATFINITE) {
        (Rounding::Rn, false, false) => llvm_f2tf32_rn(a),
        (Rounding::Rn, true, false) => llvm_f2tf32_rn_relu(a),
        (Rounding::Rn, false, true) => llvm_f2tf32_rn_satfinite(a),
        (Rounding::Rn, true, true) => llvm_f2tf32_rn_relu_satfinite(a),
        (Rounding::Rz, false, false) => llvm_f2tf32_rz(a),
        (Rounding::Rz, true, false) => llvm_f2tf32_rz_relu(a),
        (Rounding::Rz, false, true) => llvm_f2tf32_rz_satfinite(a),
        (Rounding::Rz, true, true) => llvm_f2tf32_rz_relu_satfinite(a),
        _ => unreachable!(),
    }
}

/// Converts an `f32` to the `tf32` format, rounding to nearest with ties away from zero, and
/// returns its bit pattern.
///
/// `SATFINITE` clamps the result to the finite range of `tf32`.
///
/// Requires `sm_80` and PTX ISA 7.0. `SATFINITE` additionally requires PTX ISA 8.1.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cvt_rna_tf32_f32<const SATFINITE: bool>(a: f32) -> u32 {
    match SATFINITE {
        false => llvm_f2tf32_rna(a),
        true => llvm_f2tf32_rna_satfinite(a),
    }
}
