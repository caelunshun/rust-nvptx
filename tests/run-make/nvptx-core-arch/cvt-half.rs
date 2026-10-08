// --- LLM-generated --- //
// Checks the PTX emitted for the conversions to 16-bit floating-point types and `tf32` in
// `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx, f16)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry cvt_half_bf16_f32(
// CHECK-DAG: cvt.rn.bf16.f32 %
// CHECK-DAG: cvt.rz.bf16.f32 %
// CHECK-DAG: cvt.rn.relu.bf16.f32 %
// CHECK-DAG: cvt.rz.relu.bf16.f32 %
// CHECK-DAG: cvt.rn.satfinite.bf16.f32 %
// CHECK-DAG: cvt.rz.satfinite.bf16.f32 %
// CHECK-DAG: cvt.rn.relu.satfinite.bf16.f32 %
// CHECK-DAG: cvt.rz.relu.satfinite.bf16.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_half_bf16_f32(out: *mut bf16, a: f32) {
    unsafe {
        *out.add(0) = cvt_bf16_f32::<{ Rounding::Rn }, false, false>(a);
        *out.add(1) = cvt_bf16_f32::<{ Rounding::Rz }, false, false>(a);
        *out.add(2) = cvt_bf16_f32::<{ Rounding::Rn }, true, false>(a);
        *out.add(3) = cvt_bf16_f32::<{ Rounding::Rz }, true, false>(a);
        *out.add(4) = cvt_bf16_f32::<{ Rounding::Rn }, false, true>(a);
        *out.add(5) = cvt_bf16_f32::<{ Rounding::Rz }, false, true>(a);
        *out.add(6) = cvt_bf16_f32::<{ Rounding::Rn }, true, true>(a);
        *out.add(7) = cvt_bf16_f32::<{ Rounding::Rz }, true, true>(a);
    }
}

// CHECK-LABEL: .entry cvt_half_bf16x2_f32(
// CHECK-DAG: cvt.rn.bf16x2.f32 %
// CHECK-DAG: cvt.rz.bf16x2.f32 %
// CHECK-DAG: cvt.rn.relu.bf16x2.f32 %
// CHECK-DAG: cvt.rz.relu.bf16x2.f32 %
// CHECK-DAG: cvt.rn.satfinite.bf16x2.f32 %
// CHECK-DAG: cvt.rz.satfinite.bf16x2.f32 %
// CHECK-DAG: cvt.rn.relu.satfinite.bf16x2.f32 %
// CHECK-DAG: cvt.rz.relu.satfinite.bf16x2.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_half_bf16x2_f32(out: *mut bf16x2, a: f32, b: f32) {
    unsafe {
        *out.add(0) = cvt_bf16x2_f32::<{ Rounding::Rn }, false, false>(a, b);
        *out.add(1) = cvt_bf16x2_f32::<{ Rounding::Rz }, false, false>(a, b);
        *out.add(2) = cvt_bf16x2_f32::<{ Rounding::Rn }, true, false>(a, b);
        *out.add(3) = cvt_bf16x2_f32::<{ Rounding::Rz }, true, false>(a, b);
        *out.add(4) = cvt_bf16x2_f32::<{ Rounding::Rn }, false, true>(a, b);
        *out.add(5) = cvt_bf16x2_f32::<{ Rounding::Rz }, false, true>(a, b);
        *out.add(6) = cvt_bf16x2_f32::<{ Rounding::Rn }, true, true>(a, b);
        *out.add(7) = cvt_bf16x2_f32::<{ Rounding::Rz }, true, true>(a, b);
    }
}

// CHECK-LABEL: .entry cvt_half_f16_f32(
// CHECK-DAG: cvt.rn.f16.f32 %
// CHECK-DAG: cvt.rn.ftz.f16.f32 %
// CHECK-DAG: cvt.rz.f16.f32 %
// CHECK-DAG: cvt.rn.relu.f16.f32 %
// CHECK-DAG: cvt.rz.relu.f16.f32 %
// CHECK-DAG: cvt.rn.satfinite.f16.f32 %
// CHECK-DAG: cvt.rz.satfinite.f16.f32 %
// CHECK-DAG: cvt.rn.relu.satfinite.f16.f32 %
// CHECK-DAG: cvt.rz.relu.satfinite.f16.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_half_f16_f32(out: *mut f16, a: f32) {
    unsafe {
        *out.add(0) = cvt_f16_f32::<{ Rounding::Rn }, false, false, false>(a);
        *out.add(1) = cvt_f16_f32::<{ Rounding::Rn }, true, false, false>(a);
        *out.add(2) = cvt_f16_f32::<{ Rounding::Rz }, false, false, false>(a);
        *out.add(3) = cvt_f16_f32::<{ Rounding::Rn }, false, true, false>(a);
        *out.add(4) = cvt_f16_f32::<{ Rounding::Rz }, false, true, false>(a);
        *out.add(5) = cvt_f16_f32::<{ Rounding::Rn }, false, false, true>(a);
        *out.add(6) = cvt_f16_f32::<{ Rounding::Rz }, false, false, true>(a);
        *out.add(7) = cvt_f16_f32::<{ Rounding::Rn }, false, true, true>(a);
        *out.add(8) = cvt_f16_f32::<{ Rounding::Rz }, false, true, true>(a);
    }
}

// CHECK-LABEL: .entry cvt_half_f16x2_f32(
// CHECK-DAG: cvt.rn.f16x2.f32 %
// CHECK-DAG: cvt.rz.f16x2.f32 %
// CHECK-DAG: cvt.rn.relu.f16x2.f32 %
// CHECK-DAG: cvt.rz.relu.f16x2.f32 %
// CHECK-DAG: cvt.rn.satfinite.f16x2.f32 %
// CHECK-DAG: cvt.rz.satfinite.f16x2.f32 %
// CHECK-DAG: cvt.rn.relu.satfinite.f16x2.f32 %
// CHECK-DAG: cvt.rz.relu.satfinite.f16x2.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_half_f16x2_f32(out: *mut f16x2, a: f32, b: f32) {
    unsafe {
        *out.add(0) = cvt_f16x2_f32::<{ Rounding::Rn }, false, false>(a, b);
        *out.add(1) = cvt_f16x2_f32::<{ Rounding::Rz }, false, false>(a, b);
        *out.add(2) = cvt_f16x2_f32::<{ Rounding::Rn }, true, false>(a, b);
        *out.add(3) = cvt_f16x2_f32::<{ Rounding::Rz }, true, false>(a, b);
        *out.add(4) = cvt_f16x2_f32::<{ Rounding::Rn }, false, true>(a, b);
        *out.add(5) = cvt_f16x2_f32::<{ Rounding::Rz }, false, true>(a, b);
        *out.add(6) = cvt_f16x2_f32::<{ Rounding::Rn }, true, true>(a, b);
        *out.add(7) = cvt_f16x2_f32::<{ Rounding::Rz }, true, true>(a, b);
    }
}

// CHECK-LABEL: .entry cvt_half_rna_tf32_f32(
// CHECK-DAG: cvt.rna.tf32.f32 %
// CHECK-DAG: cvt.rna.satfinite.tf32.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_half_rna_tf32_f32(out: *mut u32, a: f32) {
    unsafe {
        *out.add(0) = cvt_rna_tf32_f32::<false>(a);
        *out.add(1) = cvt_rna_tf32_f32::<true>(a);
    }
}

// CHECK-LABEL: .entry cvt_half_tf32_f32(
// CHECK-DAG: cvt.rn.tf32.f32 %
// CHECK-DAG: cvt.rz.tf32.f32 %
// CHECK-DAG: cvt.rn.relu.tf32.f32 %
// CHECK-DAG: cvt.rz.relu.tf32.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_half_tf32_f32(out: *mut u32, a: f32) {
    unsafe {
        *out.add(0) = cvt_tf32_f32::<{ Rounding::Rn }, false, false>(a);
        *out.add(1) = cvt_tf32_f32::<{ Rounding::Rz }, false, false>(a);
        *out.add(2) = cvt_tf32_f32::<{ Rounding::Rn }, true, false>(a);
        *out.add(3) = cvt_tf32_f32::<{ Rounding::Rz }, true, false>(a);
    }
}
