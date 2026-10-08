// --- LLM-generated --- //
// Checks the PTX emitted for the floating-point minimum and maximum intrinsics in
// `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx, f16)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry minmax_max_bf16(
// CHECK: max.bf16 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
// CHECK: max.xorsign.abs.bf16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.NaN.bf16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.NaN.xorsign.abs.bf16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_max_bf16(out: *mut bf16, a: bf16, b: bf16) {
    unsafe {
        *out = max_bf16::<false, false>(a, b);
        *out.add(1) = max_bf16::<false, true>(a, b);
        *out.add(2) = max_bf16::<true, false>(a, b);
        *out.add(3) = max_bf16::<true, true>(a, b);
    }
}

// CHECK-LABEL: .entry minmax_max_bf16x2(
// CHECK: max.bf16x2 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
// CHECK: max.xorsign.abs.bf16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.NaN.bf16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.NaN.xorsign.abs.bf16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_max_bf16x2(
    out: *mut bf16x2,
    a: *const bf16x2,
    b: *const bf16x2,
) {
    unsafe {
        let (a, b) = (*a, *b);
        *out = max_bf16x2::<false, false>(a, b);
        *out.add(1) = max_bf16x2::<false, true>(a, b);
        *out.add(2) = max_bf16x2::<true, false>(a, b);
        *out.add(3) = max_bf16x2::<true, true>(a, b);
    }
}

// CHECK-LABEL: .entry minmax_max_f16(
// CHECK: max.f16 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
// CHECK: max.xorsign.abs.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.NaN.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.NaN.xorsign.abs.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.xorsign.abs.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.NaN.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.NaN.xorsign.abs.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_max_f16(out: *mut f16, a: f16, b: f16) {
    unsafe {
        *out = max_f16::<false, false, false>(a, b);
        *out.add(1) = max_f16::<false, false, true>(a, b);
        *out.add(2) = max_f16::<false, true, false>(a, b);
        *out.add(3) = max_f16::<false, true, true>(a, b);
        *out.add(4) = max_f16::<true, false, false>(a, b);
        *out.add(5) = max_f16::<true, false, true>(a, b);
        *out.add(6) = max_f16::<true, true, false>(a, b);
        *out.add(7) = max_f16::<true, true, true>(a, b);
    }
}

// CHECK-LABEL: .entry minmax_max_f16x2(
// CHECK: max.f16x2 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
// CHECK: max.xorsign.abs.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.NaN.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.NaN.xorsign.abs.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.xorsign.abs.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.NaN.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.NaN.xorsign.abs.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_max_f16x2(
    out: *mut f16x2,
    a: *const f16x2,
    b: *const f16x2,
) {
    unsafe {
        let (a, b) = (*a, *b);
        *out = max_f16x2::<false, false, false>(a, b);
        *out.add(1) = max_f16x2::<false, false, true>(a, b);
        *out.add(2) = max_f16x2::<false, true, false>(a, b);
        *out.add(3) = max_f16x2::<false, true, true>(a, b);
        *out.add(4) = max_f16x2::<true, false, false>(a, b);
        *out.add(5) = max_f16x2::<true, false, true>(a, b);
        *out.add(6) = max_f16x2::<true, true, false>(a, b);
        *out.add(7) = max_f16x2::<true, true, true>(a, b);
    }
}

// CHECK-LABEL: .entry minmax_max_f32(
// CHECK: max.f32 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
// CHECK: max.xorsign.abs.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.NaN.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.NaN.xorsign.abs.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.xorsign.abs.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.NaN.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: max.ftz.NaN.xorsign.abs.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_max_f32(out: *mut f32, a: f32, b: f32) {
    unsafe {
        *out = max_f32::<false, false, false>(a, b);
        *out.add(1) = max_f32::<false, false, true>(a, b);
        *out.add(2) = max_f32::<false, true, false>(a, b);
        *out.add(3) = max_f32::<false, true, true>(a, b);
        *out.add(4) = max_f32::<true, false, false>(a, b);
        *out.add(5) = max_f32::<true, false, true>(a, b);
        *out.add(6) = max_f32::<true, true, false>(a, b);
        *out.add(7) = max_f32::<true, true, true>(a, b);
    }
}

// CHECK-LABEL: .entry minmax_max_f64(
// CHECK: max.f64 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_max_f64(out: *mut f64, a: f64, b: f64) {
    unsafe { *out = max_f64(a, b) }
}

// CHECK-LABEL: .entry minmax_min_bf16(
// CHECK: min.bf16 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
// CHECK: min.xorsign.abs.bf16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.NaN.bf16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.NaN.xorsign.abs.bf16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_min_bf16(out: *mut bf16, a: bf16, b: bf16) {
    unsafe {
        *out = min_bf16::<false, false>(a, b);
        *out.add(1) = min_bf16::<false, true>(a, b);
        *out.add(2) = min_bf16::<true, false>(a, b);
        *out.add(3) = min_bf16::<true, true>(a, b);
    }
}

// CHECK-LABEL: .entry minmax_min_bf16x2(
// CHECK: min.bf16x2 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
// CHECK: min.xorsign.abs.bf16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.NaN.bf16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.NaN.xorsign.abs.bf16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_min_bf16x2(
    out: *mut bf16x2,
    a: *const bf16x2,
    b: *const bf16x2,
) {
    unsafe {
        let (a, b) = (*a, *b);
        *out = min_bf16x2::<false, false>(a, b);
        *out.add(1) = min_bf16x2::<false, true>(a, b);
        *out.add(2) = min_bf16x2::<true, false>(a, b);
        *out.add(3) = min_bf16x2::<true, true>(a, b);
    }
}

// CHECK-LABEL: .entry minmax_min_f16(
// CHECK: min.f16 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
// CHECK: min.xorsign.abs.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.NaN.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.NaN.xorsign.abs.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.xorsign.abs.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.NaN.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.NaN.xorsign.abs.f16 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_min_f16(out: *mut f16, a: f16, b: f16) {
    unsafe {
        *out = min_f16::<false, false, false>(a, b);
        *out.add(1) = min_f16::<false, false, true>(a, b);
        *out.add(2) = min_f16::<false, true, false>(a, b);
        *out.add(3) = min_f16::<false, true, true>(a, b);
        *out.add(4) = min_f16::<true, false, false>(a, b);
        *out.add(5) = min_f16::<true, false, true>(a, b);
        *out.add(6) = min_f16::<true, true, false>(a, b);
        *out.add(7) = min_f16::<true, true, true>(a, b);
    }
}

// CHECK-LABEL: .entry minmax_min_f16x2(
// CHECK: min.f16x2 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
// CHECK: min.xorsign.abs.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.NaN.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.NaN.xorsign.abs.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.xorsign.abs.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.NaN.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.NaN.xorsign.abs.f16x2 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_min_f16x2(
    out: *mut f16x2,
    a: *const f16x2,
    b: *const f16x2,
) {
    unsafe {
        let (a, b) = (*a, *b);
        *out = min_f16x2::<false, false, false>(a, b);
        *out.add(1) = min_f16x2::<false, false, true>(a, b);
        *out.add(2) = min_f16x2::<false, true, false>(a, b);
        *out.add(3) = min_f16x2::<false, true, true>(a, b);
        *out.add(4) = min_f16x2::<true, false, false>(a, b);
        *out.add(5) = min_f16x2::<true, false, true>(a, b);
        *out.add(6) = min_f16x2::<true, true, false>(a, b);
        *out.add(7) = min_f16x2::<true, true, true>(a, b);
    }
}

// CHECK-LABEL: .entry minmax_min_f32(
// CHECK: min.f32 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
// CHECK: min.xorsign.abs.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.NaN.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.NaN.xorsign.abs.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.xorsign.abs.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.NaN.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
// CHECK: min.ftz.NaN.xorsign.abs.f32 {{%[a-z]+[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_min_f32(out: *mut f32, a: f32, b: f32) {
    unsafe {
        *out = min_f32::<false, false, false>(a, b);
        *out.add(1) = min_f32::<false, false, true>(a, b);
        *out.add(2) = min_f32::<false, true, false>(a, b);
        *out.add(3) = min_f32::<false, true, true>(a, b);
        *out.add(4) = min_f32::<true, false, false>(a, b);
        *out.add(5) = min_f32::<true, false, true>(a, b);
        *out.add(6) = min_f32::<true, true, false>(a, b);
        *out.add(7) = min_f32::<true, true, true>(a, b);
    }
}

// CHECK-LABEL: .entry minmax_min_f64(
// CHECK: min.f64 {{%[a-z]+[0-9]+}}, [[A:%[a-z]+[0-9]+]], [[B:%[a-z]+[0-9]+]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn minmax_min_f64(out: *mut f64, a: f64, b: f64) {
    unsafe { *out = min_f64(a, b) }
}
