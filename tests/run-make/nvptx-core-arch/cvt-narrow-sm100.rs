// --- LLM-generated --- //
// Checks the PTX emitted for the FP4, FP6 and FP8 conversions and the `ue8m0x2` scale
// conversions in `core::arch::nvptx` that target the `sm_100f` family.
// nvptx-target: sm_100f +ptx92

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry cvt_narrow_bf16x2_ue8m0x2(
// CHECK-DAG: cvt.rn.bf16x2.ue8m0x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_bf16x2_ue8m0x2(out: *mut bf16x2, a: u16) {
    unsafe {
        *out.add(0) = cvt_bf16x2_ue8m0x2(a);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e2m1x2_bf16x2(
// CHECK-DAG: cvt.rn.satfinite.e2m1x2.bf16x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.e2m1x2.bf16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e2m1x2_bf16x2(out: *mut u16, pa: *const bf16x2) {
    unsafe {
        *out.add(0) = cvt_e2m1x2_bf16x2::<false>(*pa);
        *out.add(1) = cvt_e2m1x2_bf16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e2m1x2_f16x2(
// CHECK-DAG: cvt.rn.satfinite.e2m1x2.f16x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.e2m1x2.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e2m1x2_f16x2(out: *mut u16, pa: *const f16x2) {
    unsafe {
        *out.add(0) = cvt_e2m1x2_f16x2::<false>(*pa);
        *out.add(1) = cvt_e2m1x2_f16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e2m1x2_f32(
// CHECK-DAG: cvt.rn.satfinite.e2m1x2.f32 %
// CHECK-DAG: cvt.rn.satfinite.relu.e2m1x2.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e2m1x2_f32(out: *mut u16, a: f32, b: f32) {
    unsafe {
        *out.add(0) = cvt_e2m1x2_f32::<false>(a, b);
        *out.add(1) = cvt_e2m1x2_f32::<true>(a, b);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e2m3x2_bf16x2(
// CHECK-DAG: cvt.rn.satfinite.e2m3x2.bf16x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.e2m3x2.bf16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e2m3x2_bf16x2(out: *mut u16, pa: *const bf16x2) {
    unsafe {
        *out.add(0) = cvt_e2m3x2_bf16x2::<false>(*pa);
        *out.add(1) = cvt_e2m3x2_bf16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e2m3x2_f16x2(
// CHECK-DAG: cvt.rn.satfinite.e2m3x2.f16x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.e2m3x2.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e2m3x2_f16x2(out: *mut u16, pa: *const f16x2) {
    unsafe {
        *out.add(0) = cvt_e2m3x2_f16x2::<false>(*pa);
        *out.add(1) = cvt_e2m3x2_f16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e2m3x2_f32(
// CHECK-DAG: cvt.rn.satfinite.e2m3x2.f32 %
// CHECK-DAG: cvt.rn.satfinite.relu.e2m3x2.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e2m3x2_f32(out: *mut u16, a: f32, b: f32) {
    unsafe {
        *out.add(0) = cvt_e2m3x2_f32::<false>(a, b);
        *out.add(1) = cvt_e2m3x2_f32::<true>(a, b);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e3m2x2_bf16x2(
// CHECK-DAG: cvt.rn.satfinite.e3m2x2.bf16x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.e3m2x2.bf16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e3m2x2_bf16x2(out: *mut u16, pa: *const bf16x2) {
    unsafe {
        *out.add(0) = cvt_e3m2x2_bf16x2::<false>(*pa);
        *out.add(1) = cvt_e3m2x2_bf16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e3m2x2_f16x2(
// CHECK-DAG: cvt.rn.satfinite.e3m2x2.f16x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.e3m2x2.f16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e3m2x2_f16x2(out: *mut u16, pa: *const f16x2) {
    unsafe {
        *out.add(0) = cvt_e3m2x2_f16x2::<false>(*pa);
        *out.add(1) = cvt_e3m2x2_f16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e3m2x2_f32(
// CHECK-DAG: cvt.rn.satfinite.e3m2x2.f32 %
// CHECK-DAG: cvt.rn.satfinite.relu.e3m2x2.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e3m2x2_f32(out: *mut u16, a: f32, b: f32) {
    unsafe {
        *out.add(0) = cvt_e3m2x2_f32::<false>(a, b);
        *out.add(1) = cvt_e3m2x2_f32::<true>(a, b);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e4m3x2_bf16x2(
// CHECK-DAG: cvt.rn.satfinite.e4m3x2.bf16x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.e4m3x2.bf16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e4m3x2_bf16x2(out: *mut u16, pa: *const bf16x2) {
    unsafe {
        *out.add(0) = cvt_e4m3x2_bf16x2::<false>(*pa);
        *out.add(1) = cvt_e4m3x2_bf16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_e5m2x2_bf16x2(
// CHECK-DAG: cvt.rn.satfinite.e5m2x2.bf16x2 %
// CHECK-DAG: cvt.rn.satfinite.relu.e5m2x2.bf16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_e5m2x2_bf16x2(out: *mut u16, pa: *const bf16x2) {
    unsafe {
        *out.add(0) = cvt_e5m2x2_bf16x2::<false>(*pa);
        *out.add(1) = cvt_e5m2x2_bf16x2::<true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_f16x2_e2m1x2(
// CHECK-DAG: cvt.rn.f16x2.e2m1x2 %
// CHECK-DAG: cvt.rn.relu.f16x2.e2m1x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_f16x2_e2m1x2(out: *mut f16x2, a: u16) {
    unsafe {
        *out.add(0) = cvt_f16x2_e2m1x2::<false>(a);
        *out.add(1) = cvt_f16x2_e2m1x2::<true>(a);
    }
}

// CHECK-LABEL: .entry cvt_narrow_f16x2_e2m3x2(
// CHECK-DAG: cvt.rn.f16x2.e2m3x2 %
// CHECK-DAG: cvt.rn.relu.f16x2.e2m3x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_f16x2_e2m3x2(out: *mut f16x2, a: u16) {
    unsafe {
        *out.add(0) = cvt_f16x2_e2m3x2::<false>(a);
        *out.add(1) = cvt_f16x2_e2m3x2::<true>(a);
    }
}

// CHECK-LABEL: .entry cvt_narrow_f16x2_e3m2x2(
// CHECK-DAG: cvt.rn.f16x2.e3m2x2 %
// CHECK-DAG: cvt.rn.relu.f16x2.e3m2x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_f16x2_e3m2x2(out: *mut f16x2, a: u16) {
    unsafe {
        *out.add(0) = cvt_f16x2_e3m2x2::<false>(a);
        *out.add(1) = cvt_f16x2_e3m2x2::<true>(a);
    }
}

// CHECK-LABEL: .entry cvt_narrow_scale_bf16x2_e2m1x2(
// CHECK-DAG: cvt.rn.scaled::n2::ue8m0.bf16x2.e2m1x2 %
// CHECK-DAG: cvt.rn.satfinite.scaled::n2::ue8m0.bf16x2.e2m1x2 %
// CHECK-DAG: cvt.rn.relu.scaled::n2::ue8m0.bf16x2.e2m1x2 %
// CHECK-DAG: cvt.rn.relu.satfinite.scaled::n2::ue8m0.bf16x2.e2m1x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_scale_bf16x2_e2m1x2(
    out: *mut bf16x2,
    a: u16,
    scale: u16,
) {
    unsafe {
        *out.add(0) = cvt_scale_bf16x2_e2m1x2::<false, false>(a, scale);
        *out.add(1) = cvt_scale_bf16x2_e2m1x2::<false, true>(a, scale);
        *out.add(2) = cvt_scale_bf16x2_e2m1x2::<true, false>(a, scale);
        *out.add(3) = cvt_scale_bf16x2_e2m1x2::<true, true>(a, scale);
    }
}

// CHECK-LABEL: .entry cvt_narrow_scale_bf16x2_e2m3x2(
// CHECK-DAG: cvt.rn.scaled::n2::ue8m0.bf16x2.e2m3x2 %
// CHECK-DAG: cvt.rn.satfinite.scaled::n2::ue8m0.bf16x2.e2m3x2 %
// CHECK-DAG: cvt.rn.relu.scaled::n2::ue8m0.bf16x2.e2m3x2 %
// CHECK-DAG: cvt.rn.relu.satfinite.scaled::n2::ue8m0.bf16x2.e2m3x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_scale_bf16x2_e2m3x2(
    out: *mut bf16x2,
    a: u16,
    scale: u16,
) {
    unsafe {
        *out.add(0) = cvt_scale_bf16x2_e2m3x2::<false, false>(a, scale);
        *out.add(1) = cvt_scale_bf16x2_e2m3x2::<false, true>(a, scale);
        *out.add(2) = cvt_scale_bf16x2_e2m3x2::<true, false>(a, scale);
        *out.add(3) = cvt_scale_bf16x2_e2m3x2::<true, true>(a, scale);
    }
}

// CHECK-LABEL: .entry cvt_narrow_scale_bf16x2_e3m2x2(
// CHECK-DAG: cvt.rn.scaled::n2::ue8m0.bf16x2.e3m2x2 %
// CHECK-DAG: cvt.rn.satfinite.scaled::n2::ue8m0.bf16x2.e3m2x2 %
// CHECK-DAG: cvt.rn.relu.scaled::n2::ue8m0.bf16x2.e3m2x2 %
// CHECK-DAG: cvt.rn.relu.satfinite.scaled::n2::ue8m0.bf16x2.e3m2x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_scale_bf16x2_e3m2x2(
    out: *mut bf16x2,
    a: u16,
    scale: u16,
) {
    unsafe {
        *out.add(0) = cvt_scale_bf16x2_e3m2x2::<false, false>(a, scale);
        *out.add(1) = cvt_scale_bf16x2_e3m2x2::<false, true>(a, scale);
        *out.add(2) = cvt_scale_bf16x2_e3m2x2::<true, false>(a, scale);
        *out.add(3) = cvt_scale_bf16x2_e3m2x2::<true, true>(a, scale);
    }
}

// CHECK-LABEL: .entry cvt_narrow_scale_bf16x2_e4m3x2(
// CHECK-DAG: cvt.rn.scaled::n2::ue8m0.bf16x2.e4m3x2 %
// CHECK-DAG: cvt.rn.satfinite.scaled::n2::ue8m0.bf16x2.e4m3x2 %
// CHECK-DAG: cvt.rn.relu.scaled::n2::ue8m0.bf16x2.e4m3x2 %
// CHECK-DAG: cvt.rn.relu.satfinite.scaled::n2::ue8m0.bf16x2.e4m3x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_scale_bf16x2_e4m3x2(
    out: *mut bf16x2,
    a: u16,
    scale: u16,
) {
    unsafe {
        *out.add(0) = cvt_scale_bf16x2_e4m3x2::<false, false>(a, scale);
        *out.add(1) = cvt_scale_bf16x2_e4m3x2::<false, true>(a, scale);
        *out.add(2) = cvt_scale_bf16x2_e4m3x2::<true, false>(a, scale);
        *out.add(3) = cvt_scale_bf16x2_e4m3x2::<true, true>(a, scale);
    }
}

// CHECK-LABEL: .entry cvt_narrow_scale_bf16x2_e5m2x2(
// CHECK-DAG: cvt.rn.scaled::n2::ue8m0.bf16x2.e5m2x2 %
// CHECK-DAG: cvt.rn.satfinite.scaled::n2::ue8m0.bf16x2.e5m2x2 %
// CHECK-DAG: cvt.rn.relu.scaled::n2::ue8m0.bf16x2.e5m2x2 %
// CHECK-DAG: cvt.rn.relu.satfinite.scaled::n2::ue8m0.bf16x2.e5m2x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_scale_bf16x2_e5m2x2(
    out: *mut bf16x2,
    a: u16,
    scale: u16,
) {
    unsafe {
        *out.add(0) = cvt_scale_bf16x2_e5m2x2::<false, false>(a, scale);
        *out.add(1) = cvt_scale_bf16x2_e5m2x2::<false, true>(a, scale);
        *out.add(2) = cvt_scale_bf16x2_e5m2x2::<true, false>(a, scale);
        *out.add(3) = cvt_scale_bf16x2_e5m2x2::<true, true>(a, scale);
    }
}

// CHECK-LABEL: .entry cvt_narrow_ue8m0x2_bf16x2(
// CHECK-DAG: cvt.rz.ue8m0x2.bf16x2 %
// CHECK-DAG: cvt.rz.satfinite.ue8m0x2.bf16x2 %
// CHECK-DAG: cvt.rp.ue8m0x2.bf16x2 %
// CHECK-DAG: cvt.rp.satfinite.ue8m0x2.bf16x2 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_ue8m0x2_bf16x2(out: *mut u16, pa: *const bf16x2) {
    unsafe {
        *out.add(0) = cvt_ue8m0x2_bf16x2::<{ Rounding::Rz }, false>(*pa);
        *out.add(1) = cvt_ue8m0x2_bf16x2::<{ Rounding::Rz }, true>(*pa);
        *out.add(2) = cvt_ue8m0x2_bf16x2::<{ Rounding::Rp }, false>(*pa);
        *out.add(3) = cvt_ue8m0x2_bf16x2::<{ Rounding::Rp }, true>(*pa);
    }
}

// CHECK-LABEL: .entry cvt_narrow_ue8m0x2_f32(
// CHECK-DAG: cvt.rz.ue8m0x2.f32 %
// CHECK-DAG: cvt.rz.satfinite.ue8m0x2.f32 %
// CHECK-DAG: cvt.rp.ue8m0x2.f32 %
// CHECK-DAG: cvt.rp.satfinite.ue8m0x2.f32 %
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_narrow_ue8m0x2_f32(out: *mut u16, a: f32, b: f32) {
    unsafe {
        *out.add(0) = cvt_ue8m0x2_f32::<{ Rounding::Rz }, false>(a, b);
        *out.add(1) = cvt_ue8m0x2_f32::<{ Rounding::Rz }, true>(a, b);
        *out.add(2) = cvt_ue8m0x2_f32::<{ Rounding::Rp }, false>(a, b);
        *out.add(3) = cvt_ue8m0x2_f32::<{ Rounding::Rp }, true>(a, b);
    }
}
