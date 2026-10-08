// --- LLM-generated --- //
//! Floating-point arithmetic instructions.

use super::modifiers::Rounding;
use super::packed::*;
use crate::intrinsics::simd::*;

#[allow(improper_ctypes)]
unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.nvvm.add.rn.f"]
    fn llvm_add_rn_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rn.sat.f"]
    fn llvm_add_rn_sat_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rn.ftz.f"]
    fn llvm_add_rn_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rn.ftz.sat.f"]
    fn llvm_add_rn_ftz_sat_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rz.f"]
    fn llvm_add_rz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rz.sat.f"]
    fn llvm_add_rz_sat_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rz.ftz.f"]
    fn llvm_add_rz_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rz.ftz.sat.f"]
    fn llvm_add_rz_ftz_sat_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rm.f"]
    fn llvm_add_rm_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rm.sat.f"]
    fn llvm_add_rm_sat_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rm.ftz.f"]
    fn llvm_add_rm_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rm.ftz.sat.f"]
    fn llvm_add_rm_ftz_sat_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rp.f"]
    fn llvm_add_rp_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rp.sat.f"]
    fn llvm_add_rp_sat_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rp.ftz.f"]
    fn llvm_add_rp_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rp.ftz.sat.f"]
    fn llvm_add_rp_ftz_sat_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.add.rn.d"]
    fn llvm_add_rn_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.add.rz.d"]
    fn llvm_add_rz_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.add.rm.d"]
    fn llvm_add_rm_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.add.rp.d"]
    fn llvm_add_rp_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.add.rn.sat.f16"]
    fn llvm_add_rn_sat_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.add.rn.ftz.sat.f16"]
    fn llvm_add_rn_ftz_sat_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.add.rn.sat.v2f16"]
    fn llvm_add_rn_sat_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.add.rn.ftz.sat.v2f16"]
    fn llvm_add_rn_ftz_sat_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.mul.rn.f"]
    fn llvm_mul_rn_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.mul.rn.ftz.f"]
    fn llvm_mul_rn_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.mul.rz.f"]
    fn llvm_mul_rz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.mul.rz.ftz.f"]
    fn llvm_mul_rz_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.mul.rm.f"]
    fn llvm_mul_rm_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.mul.rm.ftz.f"]
    fn llvm_mul_rm_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.mul.rp.f"]
    fn llvm_mul_rp_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.mul.rp.ftz.f"]
    fn llvm_mul_rp_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.mul.rn.d"]
    fn llvm_mul_rn_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.mul.rz.d"]
    fn llvm_mul_rz_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.mul.rm.d"]
    fn llvm_mul_rm_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.mul.rp.d"]
    fn llvm_mul_rp_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.mul.rn.sat.f16"]
    fn llvm_mul_rn_sat_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.mul.rn.ftz.sat.f16"]
    fn llvm_mul_rn_ftz_sat_f16(a: f16, b: f16) -> f16;
    #[link_name = "llvm.nvvm.mul.rn.sat.v2f16"]
    fn llvm_mul_rn_sat_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.mul.rn.ftz.sat.v2f16"]
    fn llvm_mul_rn_ftz_sat_f16x2(a: f16x2, b: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fma.rn.f"]
    fn llvm_fma_rn_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rn.sat.f"]
    fn llvm_fma_rn_sat_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rn.ftz.f"]
    fn llvm_fma_rn_ftz_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rn.ftz.sat.f"]
    fn llvm_fma_rn_ftz_sat_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rz.f"]
    fn llvm_fma_rz_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rz.sat.f"]
    fn llvm_fma_rz_sat_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rz.ftz.f"]
    fn llvm_fma_rz_ftz_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rz.ftz.sat.f"]
    fn llvm_fma_rz_ftz_sat_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rm.f"]
    fn llvm_fma_rm_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rm.sat.f"]
    fn llvm_fma_rm_sat_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rm.ftz.f"]
    fn llvm_fma_rm_ftz_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rm.ftz.sat.f"]
    fn llvm_fma_rm_ftz_sat_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rp.f"]
    fn llvm_fma_rp_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rp.sat.f"]
    fn llvm_fma_rp_sat_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rp.ftz.f"]
    fn llvm_fma_rp_ftz_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rp.ftz.sat.f"]
    fn llvm_fma_rp_ftz_sat_f32(a: f32, b: f32, c: f32) -> f32;
    #[link_name = "llvm.nvvm.fma.rn.d"]
    fn llvm_fma_rn_f64(a: f64, b: f64, c: f64) -> f64;
    #[link_name = "llvm.nvvm.fma.rz.d"]
    fn llvm_fma_rz_f64(a: f64, b: f64, c: f64) -> f64;
    #[link_name = "llvm.nvvm.fma.rm.d"]
    fn llvm_fma_rm_f64(a: f64, b: f64, c: f64) -> f64;
    #[link_name = "llvm.nvvm.fma.rp.d"]
    fn llvm_fma_rp_f64(a: f64, b: f64, c: f64) -> f64;
    #[link_name = "llvm.nvvm.fma.rn.f16"]
    fn llvm_fma_rn_f16(a: f16, b: f16, c: f16) -> f16;
    #[link_name = "llvm.nvvm.fma.rn.ftz.f16"]
    fn llvm_fma_rn_ftz_f16(a: f16, b: f16, c: f16) -> f16;
    #[link_name = "llvm.nvvm.fma.rn.sat.f16"]
    fn llvm_fma_rn_sat_f16(a: f16, b: f16, c: f16) -> f16;
    #[link_name = "llvm.nvvm.fma.rn.ftz.sat.f16"]
    fn llvm_fma_rn_ftz_sat_f16(a: f16, b: f16, c: f16) -> f16;
    #[link_name = "llvm.nvvm.fma.rn.relu.f16"]
    fn llvm_fma_rn_relu_f16(a: f16, b: f16, c: f16) -> f16;
    #[link_name = "llvm.nvvm.fma.rn.ftz.relu.f16"]
    fn llvm_fma_rn_ftz_relu_f16(a: f16, b: f16, c: f16) -> f16;
    #[link_name = "llvm.nvvm.fma.rn.f16x2"]
    fn llvm_fma_rn_f16x2(a: f16x2, b: f16x2, c: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fma.rn.ftz.f16x2"]
    fn llvm_fma_rn_ftz_f16x2(a: f16x2, b: f16x2, c: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fma.rn.sat.f16x2"]
    fn llvm_fma_rn_sat_f16x2(a: f16x2, b: f16x2, c: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fma.rn.ftz.sat.f16x2"]
    fn llvm_fma_rn_ftz_sat_f16x2(a: f16x2, b: f16x2, c: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fma.rn.relu.f16x2"]
    fn llvm_fma_rn_relu_f16x2(a: f16x2, b: f16x2, c: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fma.rn.ftz.relu.f16x2"]
    fn llvm_fma_rn_ftz_relu_f16x2(a: f16x2, b: f16x2, c: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fma.rn.bf16"]
    fn llvm_fma_rn_bf16(a: bf16, b: bf16, c: bf16) -> bf16;
    #[link_name = "llvm.nvvm.fma.rn.relu.bf16"]
    fn llvm_fma_rn_relu_bf16(a: bf16, b: bf16, c: bf16) -> bf16;
    #[link_name = "llvm.nvvm.fma.rn.bf16x2"]
    fn llvm_fma_rn_bf16x2(a: bf16x2, b: bf16x2, c: bf16x2) -> bf16x2;
    #[link_name = "llvm.nvvm.fma.rn.relu.bf16x2"]
    fn llvm_fma_rn_relu_bf16x2(a: bf16x2, b: bf16x2, c: bf16x2) -> bf16x2;
    #[link_name = "llvm.nvvm.fma.rn.oob.f16"]
    fn llvm_fma_rn_oob_f16(a: f16, b: f16, c: f16) -> f16;
    #[link_name = "llvm.nvvm.fma.rn.oob.relu.f16"]
    fn llvm_fma_rn_oob_relu_f16(a: f16, b: f16, c: f16) -> f16;
    #[link_name = "llvm.nvvm.fma.rn.oob.v2f16"]
    fn llvm_fma_rn_oob_f16x2(a: f16x2, b: f16x2, c: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fma.rn.oob.relu.v2f16"]
    fn llvm_fma_rn_oob_relu_f16x2(a: f16x2, b: f16x2, c: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.div.rn.f"]
    fn llvm_div_rn_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.rn.ftz.f"]
    fn llvm_div_rn_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.rz.f"]
    fn llvm_div_rz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.rz.ftz.f"]
    fn llvm_div_rz_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.rm.f"]
    fn llvm_div_rm_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.rm.ftz.f"]
    fn llvm_div_rm_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.rp.f"]
    fn llvm_div_rp_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.rp.ftz.f"]
    fn llvm_div_rp_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.rn.d"]
    fn llvm_div_rn_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.div.rz.d"]
    fn llvm_div_rz_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.div.rm.d"]
    fn llvm_div_rm_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.div.rp.d"]
    fn llvm_div_rp_f64(a: f64, b: f64) -> f64;
    #[link_name = "llvm.nvvm.div.approx.f"]
    fn llvm_div_approx_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.approx.ftz.f"]
    fn llvm_div_approx_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.full"]
    fn llvm_div_full_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.div.full.ftz"]
    fn llvm_div_full_ftz_f32(a: f32, b: f32) -> f32;
    #[link_name = "llvm.nvvm.rcp.rn.f"]
    fn llvm_rcp_rn_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rcp.rn.ftz.f"]
    fn llvm_rcp_rn_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rcp.rz.f"]
    fn llvm_rcp_rz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rcp.rz.ftz.f"]
    fn llvm_rcp_rz_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rcp.rm.f"]
    fn llvm_rcp_rm_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rcp.rm.ftz.f"]
    fn llvm_rcp_rm_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rcp.rp.f"]
    fn llvm_rcp_rp_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rcp.rp.ftz.f"]
    fn llvm_rcp_rp_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rcp.rn.d"]
    fn llvm_rcp_rn_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.rcp.rz.d"]
    fn llvm_rcp_rz_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.rcp.rm.d"]
    fn llvm_rcp_rm_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.rcp.rp.d"]
    fn llvm_rcp_rp_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.rcp.approx.ftz.f"]
    fn llvm_rcp_approx_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rcp.approx.ftz.d"]
    fn llvm_rcp_approx_ftz_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.sqrt.rn.f"]
    fn llvm_sqrt_rn_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.sqrt.rn.ftz.f"]
    fn llvm_sqrt_rn_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.sqrt.rz.f"]
    fn llvm_sqrt_rz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.sqrt.rz.ftz.f"]
    fn llvm_sqrt_rz_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.sqrt.rm.f"]
    fn llvm_sqrt_rm_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.sqrt.rm.ftz.f"]
    fn llvm_sqrt_rm_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.sqrt.rp.f"]
    fn llvm_sqrt_rp_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.sqrt.rp.ftz.f"]
    fn llvm_sqrt_rp_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.sqrt.rn.d"]
    fn llvm_sqrt_rn_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.sqrt.rz.d"]
    fn llvm_sqrt_rz_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.sqrt.rm.d"]
    fn llvm_sqrt_rm_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.sqrt.rp.d"]
    fn llvm_sqrt_rp_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.sqrt.approx.f"]
    fn llvm_sqrt_approx_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.sqrt.approx.ftz.f"]
    fn llvm_sqrt_approx_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rsqrt.approx.f"]
    fn llvm_rsqrt_approx_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rsqrt.approx.ftz.f"]
    fn llvm_rsqrt_approx_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.rsqrt.approx.d"]
    fn llvm_rsqrt_approx_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.rsqrt.approx.ftz.d"]
    fn llvm_rsqrt_approx_ftz_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.sin.approx.f"]
    fn llvm_sin_approx_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.sin.approx.ftz.f"]
    fn llvm_sin_approx_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.cos.approx.f"]
    fn llvm_cos_approx_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.cos.approx.ftz.f"]
    fn llvm_cos_approx_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.lg2.approx.f"]
    fn llvm_lg2_approx_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.lg2.approx.ftz.f"]
    fn llvm_lg2_approx_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.ex2.approx.f"]
    fn llvm_ex2_approx_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.ex2.approx.ftz.f"]
    fn llvm_ex2_approx_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.ex2.approx.f16"]
    fn llvm_ex2_approx_f16(a: f16) -> f16;
    #[link_name = "llvm.nvvm.ex2.approx.v2f16"]
    fn llvm_ex2_approx_f16x2(a: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fabs.f32"]
    fn llvm_fabs_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.fabs.ftz.f32"]
    fn llvm_fabs_ftz_f32(a: f32) -> f32;
    #[link_name = "llvm.nvvm.fabs.f64"]
    fn llvm_fabs_f64(a: f64) -> f64;
    #[link_name = "llvm.nvvm.fabs.f16"]
    fn llvm_fabs_f16(a: f16) -> f16;
    #[link_name = "llvm.nvvm.fabs.ftz.f16"]
    fn llvm_fabs_ftz_f16(a: f16) -> f16;
    #[link_name = "llvm.nvvm.fabs.v2f16"]
    fn llvm_fabs_f16x2(a: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.fabs.ftz.v2f16"]
    fn llvm_fabs_ftz_f16x2(a: f16x2) -> f16x2;
    #[link_name = "llvm.nvvm.neg.bf16"]
    fn llvm_neg_bf16(a: bf16) -> bf16;
    #[link_name = "llvm.nvvm.neg.bf16x2"]
    fn llvm_neg_bf16x2(a: bf16x2) -> bf16x2;
}

/// Adds two `f32` values.
///
/// `RND` selects the rounding mode. `FTZ` flushes subnormal inputs and results to zero, and
/// `SAT` clamps the result to `[0.0, 1.0]`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-add>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn add_f32<const RND: Rounding, const FTZ: bool, const SAT: bool>(
    a: f32,
    b: f32,
) -> f32 {
    match (RND, FTZ, SAT) {
        (Rounding::Rn, false, false) => llvm_add_rn_f32(a, b),
        (Rounding::Rn, false, true) => llvm_add_rn_sat_f32(a, b),
        (Rounding::Rn, true, false) => llvm_add_rn_ftz_f32(a, b),
        (Rounding::Rn, true, true) => llvm_add_rn_ftz_sat_f32(a, b),
        (Rounding::Rz, false, false) => llvm_add_rz_f32(a, b),
        (Rounding::Rz, false, true) => llvm_add_rz_sat_f32(a, b),
        (Rounding::Rz, true, false) => llvm_add_rz_ftz_f32(a, b),
        (Rounding::Rz, true, true) => llvm_add_rz_ftz_sat_f32(a, b),
        (Rounding::Rm, false, false) => llvm_add_rm_f32(a, b),
        (Rounding::Rm, false, true) => llvm_add_rm_sat_f32(a, b),
        (Rounding::Rm, true, false) => llvm_add_rm_ftz_f32(a, b),
        (Rounding::Rm, true, true) => llvm_add_rm_ftz_sat_f32(a, b),
        (Rounding::Rp, false, false) => llvm_add_rp_f32(a, b),
        (Rounding::Rp, false, true) => llvm_add_rp_sat_f32(a, b),
        (Rounding::Rp, true, false) => llvm_add_rp_ftz_f32(a, b),
        (Rounding::Rp, true, true) => llvm_add_rp_ftz_sat_f32(a, b),
    }
}

/// Adds two `f64` values, rounding with `RND`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-add>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn add_f64<const RND: Rounding>(a: f64, b: f64) -> f64 {
    match RND {
        Rounding::Rn => llvm_add_rn_f64(a, b),
        Rounding::Rz => llvm_add_rz_f64(a, b),
        Rounding::Rm => llvm_add_rm_f64(a, b),
        Rounding::Rp => llvm_add_rp_f64(a, b),
    }
}

/// Adds two `f16` values, rounding to nearest even.
///
/// `FTZ` flushes subnormal inputs and results to zero, and `SAT` clamps the result to
/// `[0.0, 1.0]`. `FTZ` is only supported together with `SAT`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-add>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn add_f16<const FTZ: bool, const SAT: bool>(a: f16, b: f16) -> f16 {
    static_assert!(!FTZ || SAT, "add_f16 only supports FTZ together with SAT");
    match (FTZ, SAT) {
        (false, false) => a + b,
        (false, true) => llvm_add_rn_sat_f16(a, b),
        (true, true) => llvm_add_rn_ftz_sat_f16(a, b),
        (true, false) => unreachable!(),
    }
}

/// Adds two `f16x2` vectors element-wise, rounding to nearest even.
///
/// `FTZ` flushes subnormal inputs and results to zero, and `SAT` clamps each result to
/// `[0.0, 1.0]`. `FTZ` is only supported together with `SAT`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-add>
///
/// Corresponds to the CUDA C intrinsics:
///  - [`__hadd2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g921c795176eaa31265bd80ef4fe4b8e6)
///  - [`__hadd2_rn`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g6cd8ddb2c3d670e1a10c3eb2e7644f82)
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn add_f16x2<const FTZ: bool, const SAT: bool>(a: f16x2, b: f16x2) -> f16x2 {
    static_assert!(!FTZ || SAT, "add_f16x2 only supports FTZ together with SAT");
    match (FTZ, SAT) {
        (false, false) => simd_add(a, b),
        (false, true) => llvm_add_rn_sat_f16x2(a, b),
        (true, true) => llvm_add_rn_ftz_sat_f16x2(a, b),
        (true, false) => unreachable!(),
    }
}

/// Subtracts two `f16x2` vectors element-wise, rounding to nearest even.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-sub>
///
/// Corresponds to the CUDA C intrinsics:
///  - [`__hsub2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1ga5536c9c3d853d8c8b9de60e18b41e54)
///  - [`__hsub2_rn`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g8adc164c68d553354f749f0f0645a874)
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sub_f16x2(a: f16x2, b: f16x2) -> f16x2 {
    simd_sub(a, b)
}

/// Multiplies two `f32` values.
///
/// `RND` selects the rounding mode. `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-mul>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul_f32<const RND: Rounding, const FTZ: bool>(a: f32, b: f32) -> f32 {
    match (RND, FTZ) {
        (Rounding::Rn, false) => llvm_mul_rn_f32(a, b),
        (Rounding::Rn, true) => llvm_mul_rn_ftz_f32(a, b),
        (Rounding::Rz, false) => llvm_mul_rz_f32(a, b),
        (Rounding::Rz, true) => llvm_mul_rz_ftz_f32(a, b),
        (Rounding::Rm, false) => llvm_mul_rm_f32(a, b),
        (Rounding::Rm, true) => llvm_mul_rm_ftz_f32(a, b),
        (Rounding::Rp, false) => llvm_mul_rp_f32(a, b),
        (Rounding::Rp, true) => llvm_mul_rp_ftz_f32(a, b),
    }
}

/// Multiplies two `f64` values, rounding with `RND`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-mul>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul_f64<const RND: Rounding>(a: f64, b: f64) -> f64 {
    match RND {
        Rounding::Rn => llvm_mul_rn_f64(a, b),
        Rounding::Rz => llvm_mul_rz_f64(a, b),
        Rounding::Rm => llvm_mul_rm_f64(a, b),
        Rounding::Rp => llvm_mul_rp_f64(a, b),
    }
}

/// Multiplies two `f16` values, rounding to nearest even.
///
/// `FTZ` flushes subnormal inputs and results to zero, and `SAT` clamps the result to
/// `[0.0, 1.0]`. `FTZ` is only supported together with `SAT`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-mul>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul_f16<const FTZ: bool, const SAT: bool>(a: f16, b: f16) -> f16 {
    static_assert!(!FTZ || SAT, "mul_f16 only supports FTZ together with SAT");
    match (FTZ, SAT) {
        (false, false) => a * b,
        (false, true) => llvm_mul_rn_sat_f16(a, b),
        (true, true) => llvm_mul_rn_ftz_sat_f16(a, b),
        (true, false) => unreachable!(),
    }
}

/// Multiplies two `f16x2` vectors element-wise, rounding to nearest even.
///
/// `FTZ` flushes subnormal inputs and results to zero, and `SAT` clamps each result to
/// `[0.0, 1.0]`. `FTZ` is only supported together with `SAT`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-mul>
///
/// Corresponds to the CUDA C intrinsics:
///  - [`__hmul2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g70de3f2ee48babe4e0969397ac17708e)
///  - [`__hmul2_rn`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g99f8fe23a4b4c6898d6faf999afaa76e)
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn mul_f16x2<const FTZ: bool, const SAT: bool>(a: f16x2, b: f16x2) -> f16x2 {
    static_assert!(!FTZ || SAT, "mul_f16x2 only supports FTZ together with SAT");
    match (FTZ, SAT) {
        (false, false) => simd_mul(a, b),
        (false, true) => llvm_mul_rn_sat_f16x2(a, b),
        (true, true) => llvm_mul_rn_ftz_sat_f16x2(a, b),
        (true, false) => unreachable!(),
    }
}

/// Computes `a * b + c` with a single rounding.
///
/// `RND` selects the rounding mode. `FTZ` flushes subnormal inputs and results to zero, and
/// `SAT` clamps the result to `[0.0, 1.0]`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-fma>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fma_f32<const RND: Rounding, const FTZ: bool, const SAT: bool>(
    a: f32,
    b: f32,
    c: f32,
) -> f32 {
    match (RND, FTZ, SAT) {
        (Rounding::Rn, false, false) => llvm_fma_rn_f32(a, b, c),
        (Rounding::Rn, false, true) => llvm_fma_rn_sat_f32(a, b, c),
        (Rounding::Rn, true, false) => llvm_fma_rn_ftz_f32(a, b, c),
        (Rounding::Rn, true, true) => llvm_fma_rn_ftz_sat_f32(a, b, c),
        (Rounding::Rz, false, false) => llvm_fma_rz_f32(a, b, c),
        (Rounding::Rz, false, true) => llvm_fma_rz_sat_f32(a, b, c),
        (Rounding::Rz, true, false) => llvm_fma_rz_ftz_f32(a, b, c),
        (Rounding::Rz, true, true) => llvm_fma_rz_ftz_sat_f32(a, b, c),
        (Rounding::Rm, false, false) => llvm_fma_rm_f32(a, b, c),
        (Rounding::Rm, false, true) => llvm_fma_rm_sat_f32(a, b, c),
        (Rounding::Rm, true, false) => llvm_fma_rm_ftz_f32(a, b, c),
        (Rounding::Rm, true, true) => llvm_fma_rm_ftz_sat_f32(a, b, c),
        (Rounding::Rp, false, false) => llvm_fma_rp_f32(a, b, c),
        (Rounding::Rp, false, true) => llvm_fma_rp_sat_f32(a, b, c),
        (Rounding::Rp, true, false) => llvm_fma_rp_ftz_f32(a, b, c),
        (Rounding::Rp, true, true) => llvm_fma_rp_ftz_sat_f32(a, b, c),
    }
}

/// Computes `a * b + c` with a single rounding, rounding with `RND`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-fma>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fma_f64<const RND: Rounding>(a: f64, b: f64, c: f64) -> f64 {
    match RND {
        Rounding::Rn => llvm_fma_rn_f64(a, b, c),
        Rounding::Rz => llvm_fma_rz_f64(a, b, c),
        Rounding::Rm => llvm_fma_rm_f64(a, b, c),
        Rounding::Rp => llvm_fma_rp_f64(a, b, c),
    }
}

/// Fused multiply-add of `f16` values, rounding to nearest even.
///
/// `FTZ` flushes subnormal inputs and results to zero. `SAT` clamps the result to `[0.0, 1.0]`.
/// `RELU` clamps negative results to zero. `SAT` and `RELU` cannot be combined.
///
/// `RELU` requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-fma>
///
/// Corresponds to the CUDA C intrinsics:
///  - [`__fma2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g43628ba21ded8b1e188a367348008dab)
///  - [`__fma2_rn`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g43628ba21ded8b1e188a367348008dab)
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fma_f16<const FTZ: bool, const SAT: bool, const RELU: bool>(
    a: f16,
    b: f16,
    c: f16,
) -> f16 {
    static_assert!(
        !SAT || !RELU,
        "fma_f16 does not support SAT and RELU together"
    );
    match (FTZ, SAT, RELU) {
        (false, false, false) => llvm_fma_rn_f16(a, b, c),
        (true, false, false) => llvm_fma_rn_ftz_f16(a, b, c),
        (false, true, false) => llvm_fma_rn_sat_f16(a, b, c),
        (true, true, false) => llvm_fma_rn_ftz_sat_f16(a, b, c),
        (false, false, true) => llvm_fma_rn_relu_f16(a, b, c),
        (true, false, true) => llvm_fma_rn_ftz_relu_f16(a, b, c),
        (_, true, true) => unreachable!(),
    }
}

/// Fused multiply-add of `f16x2` values, rounding to nearest even.
///
/// `FTZ` flushes subnormal inputs and results to zero. `SAT` clamps the result to `[0.0, 1.0]`.
/// `RELU` clamps negative results to zero. `SAT` and `RELU` cannot be combined.
///
/// `RELU` requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-fma>
///
/// Corresponds to the CUDA C intrinsics:
///  - [`__fma2`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g43628ba21ded8b1e188a367348008dab)
///  - [`__fma2_rn`](https://docs.nvidia.com/cuda/cuda-math-api/group__CUDA__MATH____HALF2__ARITHMETIC.html#group__CUDA__MATH____HALF2__ARITHMETIC_1g43628ba21ded8b1e188a367348008dab)
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fma_f16x2<const FTZ: bool, const SAT: bool, const RELU: bool>(
    a: f16x2,
    b: f16x2,
    c: f16x2,
) -> f16x2 {
    static_assert!(
        !SAT || !RELU,
        "fma_f16x2 does not support SAT and RELU together"
    );
    match (FTZ, SAT, RELU) {
        (false, false, false) => llvm_fma_rn_f16x2(a, b, c),
        (true, false, false) => llvm_fma_rn_ftz_f16x2(a, b, c),
        (false, true, false) => llvm_fma_rn_sat_f16x2(a, b, c),
        (true, true, false) => llvm_fma_rn_ftz_sat_f16x2(a, b, c),
        (false, false, true) => llvm_fma_rn_relu_f16x2(a, b, c),
        (true, false, true) => llvm_fma_rn_ftz_relu_f16x2(a, b, c),
        (_, true, true) => unreachable!(),
    }
}

/// Fused multiply-add of `bf16` values, rounding to nearest even.
///
/// `RELU` clamps negative results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-fma>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fma_bf16<const RELU: bool>(a: bf16, b: bf16, c: bf16) -> bf16 {
    match RELU {
        false => llvm_fma_rn_bf16(a, b, c),
        true => llvm_fma_rn_relu_bf16(a, b, c),
    }
}

/// Fused multiply-add of `bf16x2` values, rounding to nearest even.
///
/// `RELU` clamps negative results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-fma>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fma_bf16x2<const RELU: bool>(a: bf16x2, b: bf16x2, c: bf16x2) -> bf16x2 {
    match RELU {
        false => llvm_fma_rn_bf16x2(a, b, c),
        true => llvm_fma_rn_relu_bf16x2(a, b, c),
    }
}

/// Fused multiply-add with the out-of-bounds (OOB) modifier.
///
/// The result is clamped to zero if either operand is an OOB NaN, as produced by out-of-bounds
/// tensor accesses. `RELU` clamps negative results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-fma>
#[inline]
#[target_feature(enable = "sm_90,ptx81")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fma_oob_f16<const RELU: bool>(a: f16, b: f16, c: f16) -> f16 {
    match RELU {
        false => llvm_fma_rn_oob_f16(a, b, c),
        true => llvm_fma_rn_oob_relu_f16(a, b, c),
    }
}

/// Fused multiply-add with the out-of-bounds (OOB) modifier.
///
/// The result is clamped to zero if either operand is an OOB NaN, as produced by out-of-bounds
/// tensor accesses. `RELU` clamps negative results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-fma>
#[inline]
#[target_feature(enable = "sm_90,ptx81")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn fma_oob_f16x2<const RELU: bool>(a: f16x2, b: f16x2, c: f16x2) -> f16x2 {
    match RELU {
        false => llvm_fma_rn_oob_f16x2(a, b, c),
        true => llvm_fma_rn_oob_relu_f16x2(a, b, c),
    }
}

/// Divides two `f32` values.
///
/// `RND` selects the rounding mode. `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-div>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn div_f32<const RND: Rounding, const FTZ: bool>(a: f32, b: f32) -> f32 {
    match (RND, FTZ) {
        (Rounding::Rn, false) => llvm_div_rn_f32(a, b),
        (Rounding::Rn, true) => llvm_div_rn_ftz_f32(a, b),
        (Rounding::Rz, false) => llvm_div_rz_f32(a, b),
        (Rounding::Rz, true) => llvm_div_rz_ftz_f32(a, b),
        (Rounding::Rm, false) => llvm_div_rm_f32(a, b),
        (Rounding::Rm, true) => llvm_div_rm_ftz_f32(a, b),
        (Rounding::Rp, false) => llvm_div_rp_f32(a, b),
        (Rounding::Rp, true) => llvm_div_rp_ftz_f32(a, b),
    }
}

/// Divides two `f64` values, rounding with `RND`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-div>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn div_f64<const RND: Rounding>(a: f64, b: f64) -> f64 {
    match RND {
        Rounding::Rn => llvm_div_rn_f64(a, b),
        Rounding::Rz => llvm_div_rz_f64(a, b),
        Rounding::Rm => llvm_div_rm_f64(a, b),
        Rounding::Rp => llvm_div_rp_f64(a, b),
    }
}

/// Divides two `f32` values using the approximate `div.approx` instruction.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-div>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn div_approx_f32<const FTZ: bool>(a: f32, b: f32) -> f32 {
    match FTZ {
        false => llvm_div_approx_f32(a, b),
        true => llvm_div_approx_ftz_f32(a, b),
    }
}

/// Divides two `f32` values using the `div.full` instruction, which is faster than
/// IEEE 754 division but less accurate.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-div>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn div_full_f32<const FTZ: bool>(a: f32, b: f32) -> f32 {
    match FTZ {
        false => llvm_div_full_f32(a, b),
        true => llvm_div_full_ftz_f32(a, b),
    }
}

/// Takes the reciprocal of an `f32` value.
///
/// `RND` selects the rounding mode. `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-rcp>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn rcp_f32<const RND: Rounding, const FTZ: bool>(a: f32) -> f32 {
    match (RND, FTZ) {
        (Rounding::Rn, false) => llvm_rcp_rn_f32(a),
        (Rounding::Rn, true) => llvm_rcp_rn_ftz_f32(a),
        (Rounding::Rz, false) => llvm_rcp_rz_f32(a),
        (Rounding::Rz, true) => llvm_rcp_rz_ftz_f32(a),
        (Rounding::Rm, false) => llvm_rcp_rm_f32(a),
        (Rounding::Rm, true) => llvm_rcp_rm_ftz_f32(a),
        (Rounding::Rp, false) => llvm_rcp_rp_f32(a),
        (Rounding::Rp, true) => llvm_rcp_rp_ftz_f32(a),
    }
}

/// Takes the reciprocal of an `f64` value, rounding with `RND`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-rcp>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn rcp_f64<const RND: Rounding>(a: f64) -> f64 {
    match RND {
        Rounding::Rn => llvm_rcp_rn_f64(a),
        Rounding::Rz => llvm_rcp_rz_f64(a),
        Rounding::Rm => llvm_rcp_rm_f64(a),
        Rounding::Rp => llvm_rcp_rp_f64(a),
    }
}

/// Takes a fast, approximate reciprocal of an `f32` value.
///
/// Subnormal inputs and results are flushed to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-rcp>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn rcp_approx_ftz_f32(a: f32) -> f32 {
    llvm_rcp_approx_ftz_f32(a)
}

/// Takes a fast, approximate reciprocal of an `f64` value.
///
/// Only the upper 32 bits of the result are approximated; the lower 32 bits are zero.
/// Subnormal inputs and results are flushed to zero, as `.ftz` is mandatory for `.f64`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-rcp-approx-ftz-f64>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn rcp_approx_f64(a: f64) -> f64 {
    llvm_rcp_approx_ftz_f64(a)
}

/// Takes the square root of an `f32` value.
///
/// `RND` selects the rounding mode. `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-sqrt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sqrt_f32<const RND: Rounding, const FTZ: bool>(a: f32) -> f32 {
    match (RND, FTZ) {
        (Rounding::Rn, false) => llvm_sqrt_rn_f32(a),
        (Rounding::Rn, true) => llvm_sqrt_rn_ftz_f32(a),
        (Rounding::Rz, false) => llvm_sqrt_rz_f32(a),
        (Rounding::Rz, true) => llvm_sqrt_rz_ftz_f32(a),
        (Rounding::Rm, false) => llvm_sqrt_rm_f32(a),
        (Rounding::Rm, true) => llvm_sqrt_rm_ftz_f32(a),
        (Rounding::Rp, false) => llvm_sqrt_rp_f32(a),
        (Rounding::Rp, true) => llvm_sqrt_rp_ftz_f32(a),
    }
}

/// Takes the square root of an `f64` value, rounding with `RND`.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-sqrt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sqrt_f64<const RND: Rounding>(a: f64) -> f64 {
    match RND {
        Rounding::Rn => llvm_sqrt_rn_f64(a),
        Rounding::Rz => llvm_sqrt_rz_f64(a),
        Rounding::Rm => llvm_sqrt_rm_f64(a),
        Rounding::Rp => llvm_sqrt_rp_f64(a),
    }
}

/// Takes a fast, approximate square root of an `f32` value.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-sqrt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sqrt_approx_f32<const FTZ: bool>(a: f32) -> f32 {
    match FTZ {
        false => llvm_sqrt_approx_f32(a),
        true => llvm_sqrt_approx_ftz_f32(a),
    }
}

/// Takes a fast, approximate reciprocal square root of an `f32` value.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-rsqrt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn rsqrt_approx_f32<const FTZ: bool>(a: f32) -> f32 {
    match FTZ {
        false => llvm_rsqrt_approx_f32(a),
        true => llvm_rsqrt_approx_ftz_f32(a),
    }
}

/// Takes a fast, approximate reciprocal square root of an `f64` value.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-rsqrt>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn rsqrt_approx_f64<const FTZ: bool>(a: f64) -> f64 {
    match FTZ {
        false => llvm_rsqrt_approx_f64(a),
        true => llvm_rsqrt_approx_ftz_f64(a),
    }
}

/// Computes the sine of an `f32` value, approximately.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-sin>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn sin_f32<const FTZ: bool>(a: f32) -> f32 {
    match FTZ {
        false => llvm_sin_approx_f32(a),
        true => llvm_sin_approx_ftz_f32(a),
    }
}

/// Computes the cosine of an `f32` value, approximately.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-cos>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn cos_f32<const FTZ: bool>(a: f32) -> f32 {
    match FTZ {
        false => llvm_cos_approx_f32(a),
        true => llvm_cos_approx_ftz_f32(a),
    }
}

/// Computes the base-2 logarithm of an `f32` value, approximately.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-lg2>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn lg2_f32<const FTZ: bool>(a: f32) -> f32 {
    match FTZ {
        false => llvm_lg2_approx_f32(a),
        true => llvm_lg2_approx_ftz_f32(a),
    }
}

/// Computes the base-2 exponential of an `f32` value, approximately.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-ex2>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ex2_f32<const FTZ: bool>(a: f32) -> f32 {
    match FTZ {
        false => llvm_ex2_approx_f32(a),
        true => llvm_ex2_approx_ftz_f32(a),
    }
}

/// Computes `2^a` of an `f16` value, approximately.
///
/// Requires `sm_75` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-ex2>
#[inline]
#[target_feature(enable = "sm_75,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ex2_f16(a: f16) -> f16 {
    llvm_ex2_approx_f16(a)
}

/// Computes `2^a` of an `f16x2` vector, element-wise, approximately.
///
/// Requires `sm_75` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-ex2>
#[inline]
#[target_feature(enable = "sm_75,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn ex2_f16x2(a: f16x2) -> f16x2 {
    llvm_ex2_approx_f16x2(a)
}

/// Computes the absolute value of an `f32` value.
///
/// `FTZ` flushes subnormal inputs and results to zero.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-abs>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn abs_f32<const FTZ: bool>(a: f32) -> f32 {
    match FTZ {
        false => llvm_fabs_f32(a),
        true => llvm_fabs_ftz_f32(a),
    }
}

/// Computes the absolute value of an `f64` value.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-abs>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn abs_f64(a: f64) -> f64 {
    llvm_fabs_f64(a)
}

/// Computes the absolute value of an `f16` value.
///
/// `FTZ` flushes subnormal inputs and results to zero. Requires PTX ISA 6.5.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-abs>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn abs_f16<const FTZ: bool>(a: f16) -> f16 {
    match FTZ {
        false => llvm_fabs_f16(a),
        true => llvm_fabs_ftz_f16(a),
    }
}

/// Computes the absolute value of an `f16x2` vector.
///
/// `FTZ` flushes subnormal inputs and results to zero. Requires PTX ISA 6.5.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-abs>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn abs_f16x2<const FTZ: bool>(a: f16x2) -> f16x2 {
    match FTZ {
        false => llvm_fabs_f16x2(a),
        true => llvm_fabs_ftz_f16x2(a),
    }
}

/// Negates an `f16` value.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-neg>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn neg_f16(a: f16) -> f16 {
    -a
}

/// Negates each element of an `f16x2` vector.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-neg>
#[inline]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn neg_f16x2(a: f16x2) -> f16x2 {
    simd_neg(a)
}

/// Negates a `bf16` value.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-neg>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn neg_bf16(a: bf16) -> bf16 {
    llvm_neg_bf16(a)
}

/// Negates each element of a `bf16x2` vector.
///
/// Requires `sm_80` and PTX ISA 7.0.
///
/// <https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-neg>
#[inline]
#[target_feature(enable = "sm_80,ptx70")]
#[unstable(feature = "stdarch_nvptx", issue = "111199")]
pub unsafe fn neg_bf16x2(a: bf16x2) -> bf16x2 {
    llvm_neg_bf16x2(a)
}
