// --- LLM-generated --- //
// Checks the PTX emitted for the stochastic-rounding and `satfinite` conversions to 16-bit
// floating-point types and `tf32` in `core::arch::nvptx`, which require `sm_100a`.
// nvptx-target: sm_100a +ptx87

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry cvt_half_rs_bf16x2_f32(
// CHECK-DAG: cvt.rs.bf16x2.f32 %
// CHECK-DAG: cvt.rs.relu.bf16x2.f32 %
// CHECK-DAG: cvt.rs.satfinite.bf16x2.f32 %
// CHECK-DAG: cvt.rs.relu.satfinite.bf16x2.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_half_rs_bf16x2_f32(out: *mut bf16x2, a: f32, b: f32, r: u32) {
    unsafe {
        *out.add(0) = cvt_rs_bf16x2_f32::<false, false>(a, b, r);
        *out.add(1) = cvt_rs_bf16x2_f32::<true, false>(a, b, r);
        *out.add(2) = cvt_rs_bf16x2_f32::<false, true>(a, b, r);
        *out.add(3) = cvt_rs_bf16x2_f32::<true, true>(a, b, r);
    }
}

// CHECK-LABEL: .entry cvt_half_rs_f16x2_f32(
// CHECK-DAG: cvt.rs.f16x2.f32 %
// CHECK-DAG: cvt.rs.relu.f16x2.f32 %
// CHECK-DAG: cvt.rs.satfinite.f16x2.f32 %
// CHECK-DAG: cvt.rs.relu.satfinite.f16x2.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_half_rs_f16x2_f32(out: *mut f16x2, a: f32, b: f32, r: u32) {
    unsafe {
        *out.add(0) = cvt_rs_f16x2_f32::<false, false>(a, b, r);
        *out.add(1) = cvt_rs_f16x2_f32::<true, false>(a, b, r);
        *out.add(2) = cvt_rs_f16x2_f32::<false, true>(a, b, r);
        *out.add(3) = cvt_rs_f16x2_f32::<true, true>(a, b, r);
    }
}

// CHECK-LABEL: .entry cvt_half_tf32_f32_satfinite(
// CHECK-DAG: cvt.rn.satfinite.tf32.f32 %
// CHECK-DAG: cvt.rz.satfinite.tf32.f32 %
// CHECK-DAG: cvt.rn.relu.satfinite.tf32.f32 %
// CHECK-DAG: cvt.rz.relu.satfinite.tf32.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_half_tf32_f32_satfinite(out: *mut u32, a: f32) {
    unsafe {
        *out.add(0) = cvt_tf32_f32::<{ Rounding::Rn }, false, true>(a);
        *out.add(1) = cvt_tf32_f32::<{ Rounding::Rz }, false, true>(a);
        *out.add(2) = cvt_tf32_f32::<{ Rounding::Rn }, true, true>(a);
        *out.add(3) = cvt_tf32_f32::<{ Rounding::Rz }, true, true>(a);
    }
}
