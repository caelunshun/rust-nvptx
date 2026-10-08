// --- LLM-generated --- //
// Checks the PTX emitted for the `e4m3x2` and `e5m2x2` conversions in `core::arch::nvptx`,
// which require `sm_89` with PTX ISA 8.1 or `sm_90` with PTX ISA 7.8.
// nvptx-target: sm_90 +ptx78

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry cvt_narrow_e4m3x2_f16x2(
// CHECK-DAG: cvt.rn.satfinite.e4m3x2.f16x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.e4m3x2.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e4m3x2_f16x2(out: *mut u16, pa: *const f16x2) {
    unsafe {
        *out.add(0) = cvt_e4m3x2_f16x2::<false>(*pa);
        *out.add(1) = cvt_e4m3x2_f16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e4m3x2_f32(
// CHECK-DAG: cvt.rn.satfinite.e4m3x2.f32 %
// CHECK-DAG: cvt.rn.satfinite.relu.e4m3x2.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e4m3x2_f32(out: *mut u16, a: f32, b: f32) {
    unsafe {
        *out.add(0) = cvt_e4m3x2_f32::<false>(a, b);
        *out.add(1) = cvt_e4m3x2_f32::<true>(a, b);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e5m2x2_f16x2(
// CHECK-DAG: cvt.rn.satfinite.e5m2x2.f16x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.e5m2x2.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e5m2x2_f16x2(out: *mut u16, pa: *const f16x2) {
    unsafe {
        *out.add(0) = cvt_e5m2x2_f16x2::<false>(*pa);
        *out.add(1) = cvt_e5m2x2_f16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e5m2x2_f32(
// CHECK-DAG: cvt.rn.satfinite.e5m2x2.f32 %
// CHECK-DAG: cvt.rn.satfinite.relu.e5m2x2.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e5m2x2_f32(out: *mut u16, a: f32, b: f32) {
    unsafe {
        *out.add(0) = cvt_e5m2x2_f32::<false>(a, b);
        *out.add(1) = cvt_e5m2x2_f32::<true>(a, b);
    }
}

// CHECK-LABEL: .entry cvt_narrow_f16x2_e4m3x2(
// CHECK-DAG: cvt.rn.f16x2.e4m3x2 %
// CHECK-DAG: cvt.rn.relu.f16x2.e4m3x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_f16x2_e4m3x2(out: *mut f16x2, a: u16) {
    unsafe {
        *out.add(0) = cvt_f16x2_e4m3x2::<false>(a);
        *out.add(1) = cvt_f16x2_e4m3x2::<true>(a);
    }
}

// CHECK-LABEL: .entry cvt_narrow_f16x2_e5m2x2(
// CHECK-DAG: cvt.rn.f16x2.e5m2x2 %
// CHECK-DAG: cvt.rn.relu.f16x2.e5m2x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_f16x2_e5m2x2(out: *mut f16x2, a: u16) {
    unsafe {
        *out.add(0) = cvt_f16x2_e5m2x2::<false>(a);
        *out.add(1) = cvt_f16x2_e5m2x2::<true>(a);
    }
}
