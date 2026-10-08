// --- LLM-generated --- //
// Checks the PTX emitted for the `s2f6x2` to `bf16x2` scaled conversions and the stochastic
// rounding conversions in `core::arch::nvptx`, which require `sm_100a` or `sm_103a`.
// nvptx-target: sm_100a +ptx91

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry cvt_narrow_rs_e2m1x4_f32(
// CHECK-DAG: cvt.rs.satfinite.e2m1x4.f32 %
// CHECK-DAG: cvt.rs.relu.satfinite.e2m1x4.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_rs_e2m1x4_f32(
    out: *mut u16,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    rbits: u32,
) {
    unsafe {
        *out.add(0) = cvt_rs_e2m1x4_f32::<false>(a, b, c, d, rbits);
        *out.add(1) = cvt_rs_e2m1x4_f32::<true>(a, b, c, d, rbits);
    }
}

// CHECK-LABEL: .entry cvt_narrow_rs_e2m3x4_f32(
// CHECK-DAG: cvt.rs.satfinite.e2m3x4.f32 %
// CHECK-DAG: cvt.rs.relu.satfinite.e2m3x4.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_rs_e2m3x4_f32(
    out: *mut u32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    rbits: u32,
) {
    unsafe {
        *out.add(0) = cvt_rs_e2m3x4_f32::<false>(a, b, c, d, rbits);
        *out.add(1) = cvt_rs_e2m3x4_f32::<true>(a, b, c, d, rbits);
    }
}

// CHECK-LABEL: .entry cvt_narrow_rs_e3m2x4_f32(
// CHECK-DAG: cvt.rs.satfinite.e3m2x4.f32 %
// CHECK-DAG: cvt.rs.relu.satfinite.e3m2x4.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_rs_e3m2x4_f32(
    out: *mut u32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    rbits: u32,
) {
    unsafe {
        *out.add(0) = cvt_rs_e3m2x4_f32::<false>(a, b, c, d, rbits);
        *out.add(1) = cvt_rs_e3m2x4_f32::<true>(a, b, c, d, rbits);
    }
}

// CHECK-LABEL: .entry cvt_narrow_rs_e4m3x4_f32(
// CHECK-DAG: cvt.rs.satfinite.e4m3x4.f32 %
// CHECK-DAG: cvt.rs.relu.satfinite.e4m3x4.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_rs_e4m3x4_f32(
    out: *mut u32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    rbits: u32,
) {
    unsafe {
        *out.add(0) = cvt_rs_e4m3x4_f32::<false>(a, b, c, d, rbits);
        *out.add(1) = cvt_rs_e4m3x4_f32::<true>(a, b, c, d, rbits);
    }
}

// CHECK-LABEL: .entry cvt_narrow_rs_e5m2x4_f32(
// CHECK-DAG: cvt.rs.satfinite.e5m2x4.f32 %
// CHECK-DAG: cvt.rs.relu.satfinite.e5m2x4.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_rs_e5m2x4_f32(
    out: *mut u32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    rbits: u32,
) {
    unsafe {
        *out.add(0) = cvt_rs_e5m2x4_f32::<false>(a, b, c, d, rbits);
        *out.add(1) = cvt_rs_e5m2x4_f32::<true>(a, b, c, d, rbits);
    }
}

// CHECK-LABEL: .entry cvt_narrow_scale_bf16x2_s2f6x2(
// CHECK-DAG: cvt.rn.scaled::n2::ue8m0.bf16x2.s2f6x2 %
// CHECK-DAG: cvt.rn.satfinite.scaled::n2::ue8m0.bf16x2.s2f6x2 %
// CHECK-DAG: cvt.rn.relu.scaled::n2::ue8m0.bf16x2.s2f6x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.scaled::n2::ue8m0.bf16x2.s2f6x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_scale_bf16x2_s2f6x2(
    out: *mut bf16x2,
    a: u16,
    scale: u16,
) {
    unsafe {
        *out.add(0) = cvt_scale_bf16x2_s2f6x2::<false, false>(a, scale);
        *out.add(1) = cvt_scale_bf16x2_s2f6x2::<false, true>(a, scale);
        *out.add(2) = cvt_scale_bf16x2_s2f6x2::<true, false>(a, scale);
        *out.add(3) = cvt_scale_bf16x2_s2f6x2::<true, true>(a, scale);
    }
}
