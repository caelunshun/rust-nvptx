// --- LLM-generated --- //
// Checks the PTX emitted for the conversion and rounding intrinsics in `core::arch::nvptx`.
// nvptx-target: sm_75 +ptx70

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry cvt_test_f32_f32_round(
// CHECK-DAG: cvt.rni.f32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.ftz.f32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.f32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.ftz.f32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.f32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.ftz.f32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.f32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.ftz.f32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_test_f32_f32_round(out: *mut f32, a: f32) {
    unsafe {
        *out = cvt_f32_f32::<{ Rounding::Rn }, false>(a);
        *out.add(1) = cvt_f32_f32::<{ Rounding::Rn }, true>(a);
        *out.add(2) = cvt_f32_f32::<{ Rounding::Rz }, false>(a);
        *out.add(3) = cvt_f32_f32::<{ Rounding::Rz }, true>(a);
        *out.add(4) = cvt_f32_f32::<{ Rounding::Rm }, false>(a);
        *out.add(5) = cvt_f32_f32::<{ Rounding::Rm }, true>(a);
        *out.add(6) = cvt_f32_f32::<{ Rounding::Rp }, false>(a);
        *out.add(7) = cvt_f32_f32::<{ Rounding::Rp }, true>(a);
    }
}

// CHECK-LABEL: .entry cvt_test_f32_f32_sat(
// CHECK-DAG: cvt.sat.f32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.ftz.sat.f32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_test_f32_f32_sat(out: *mut f32, a: f32) {
    unsafe {
        *out = cvt_sat_f32_f32::<false>(a);
        *out.add(1) = cvt_sat_f32_f32::<true>(a);
    }
}

// CHECK-LABEL: .entry cvt_test_f32_f64(
// CHECK-DAG: cvt.rn.f32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rn.ftz.f32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rz.f32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rz.ftz.f32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rm.f32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rm.ftz.f32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rp.f32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rp.ftz.f32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_test_f32_f64(out: *mut f32, a: f64) {
    unsafe {
        *out = cvt_f32_f64::<{ Rounding::Rn }, false>(a);
        *out.add(1) = cvt_f32_f64::<{ Rounding::Rn }, true>(a);
        *out.add(2) = cvt_f32_f64::<{ Rounding::Rz }, false>(a);
        *out.add(3) = cvt_f32_f64::<{ Rounding::Rz }, true>(a);
        *out.add(4) = cvt_f32_f64::<{ Rounding::Rm }, false>(a);
        *out.add(5) = cvt_f32_f64::<{ Rounding::Rm }, true>(a);
        *out.add(6) = cvt_f32_f64::<{ Rounding::Rp }, false>(a);
        *out.add(7) = cvt_f32_f64::<{ Rounding::Rp }, true>(a);
    }
}

// CHECK-LABEL: .entry cvt_test_f32_from_int(
// CHECK-DAG: cvt.rn.f32.s32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rz.f32.s32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rm.f32.s32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rp.f32.s32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rn.f32.u32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rz.f32.u32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rm.f32.u32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rp.f32.u32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rn.f32.s64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rz.f32.s64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rm.f32.s64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rp.f32.s64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rn.f32.u64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rz.f32.u64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rm.f32.u64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rp.f32.u64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_test_f32_from_int(
    out: *mut f32,
    a: i32,
    b: u32,
    c: i64,
    d: u64,
) {
    unsafe {
        *out = cvt_f32_s32::<{ Rounding::Rn }>(a);
        *out.add(1) = cvt_f32_s32::<{ Rounding::Rz }>(a);
        *out.add(2) = cvt_f32_s32::<{ Rounding::Rm }>(a);
        *out.add(3) = cvt_f32_s32::<{ Rounding::Rp }>(a);
        *out.add(4) = cvt_f32_u32::<{ Rounding::Rn }>(b);
        *out.add(5) = cvt_f32_u32::<{ Rounding::Rz }>(b);
        *out.add(6) = cvt_f32_u32::<{ Rounding::Rm }>(b);
        *out.add(7) = cvt_f32_u32::<{ Rounding::Rp }>(b);
        *out.add(8) = cvt_f32_s64::<{ Rounding::Rn }>(c);
        *out.add(9) = cvt_f32_s64::<{ Rounding::Rz }>(c);
        *out.add(10) = cvt_f32_s64::<{ Rounding::Rm }>(c);
        *out.add(11) = cvt_f32_s64::<{ Rounding::Rp }>(c);
        *out.add(12) = cvt_f32_u64::<{ Rounding::Rn }>(d);
        *out.add(13) = cvt_f32_u64::<{ Rounding::Rz }>(d);
        *out.add(14) = cvt_f32_u64::<{ Rounding::Rm }>(d);
        *out.add(15) = cvt_f32_u64::<{ Rounding::Rp }>(d);
    }
}

// CHECK-LABEL: .entry cvt_test_f64_f64(
// CHECK-DAG: cvt.rni.f64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.f64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.f64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.f64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.sat.f64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_test_f64_f64(out: *mut f64, a: f64) {
    unsafe {
        *out = cvt_f64_f64::<{ Rounding::Rn }>(a);
        *out.add(1) = cvt_f64_f64::<{ Rounding::Rz }>(a);
        *out.add(2) = cvt_f64_f64::<{ Rounding::Rm }>(a);
        *out.add(3) = cvt_f64_f64::<{ Rounding::Rp }>(a);
        *out.add(4) = cvt_sat_f64_f64(a);
    }
}

// CHECK-LABEL: .entry cvt_test_f64_from_int(
// CHECK-DAG: cvt.rn.f64.s32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rz.f64.s32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rm.f64.s32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rp.f64.s32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rn.f64.u32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rz.f64.u32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rm.f64.u32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rp.f64.u32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rn.f64.s64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rz.f64.s64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rm.f64.s64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rp.f64.s64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rn.f64.u64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rz.f64.u64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rm.f64.u64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rp.f64.u64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_test_f64_from_int(
    out: *mut f64,
    a: i32,
    b: u32,
    c: i64,
    d: u64,
) {
    unsafe {
        *out = cvt_f64_s32::<{ Rounding::Rn }>(a);
        *out.add(1) = cvt_f64_s32::<{ Rounding::Rz }>(a);
        *out.add(2) = cvt_f64_s32::<{ Rounding::Rm }>(a);
        *out.add(3) = cvt_f64_s32::<{ Rounding::Rp }>(a);
        *out.add(4) = cvt_f64_u32::<{ Rounding::Rn }>(b);
        *out.add(5) = cvt_f64_u32::<{ Rounding::Rz }>(b);
        *out.add(6) = cvt_f64_u32::<{ Rounding::Rm }>(b);
        *out.add(7) = cvt_f64_u32::<{ Rounding::Rp }>(b);
        *out.add(8) = cvt_f64_s64::<{ Rounding::Rn }>(c);
        *out.add(9) = cvt_f64_s64::<{ Rounding::Rz }>(c);
        *out.add(10) = cvt_f64_s64::<{ Rounding::Rm }>(c);
        *out.add(11) = cvt_f64_s64::<{ Rounding::Rp }>(c);
        *out.add(12) = cvt_f64_u64::<{ Rounding::Rn }>(d);
        *out.add(13) = cvt_f64_u64::<{ Rounding::Rz }>(d);
        *out.add(14) = cvt_f64_u64::<{ Rounding::Rm }>(d);
        *out.add(15) = cvt_f64_u64::<{ Rounding::Rp }>(d);
    }
}

// CHECK-LABEL: .entry cvt_test_int_from_f32(
// CHECK-DAG: cvt.rni.s32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.ftz.s32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.s32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.ftz.s32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.s32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.ftz.s32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.s32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.ftz.s32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.u32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.ftz.u32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.u32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.ftz.u32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.u32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.ftz.u32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.u32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.ftz.u32.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.s64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.ftz.s64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.s64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.ftz.s64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.s64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.ftz.s64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.s64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.ftz.s64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.u64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.ftz.u64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.u64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.ftz.u64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.u64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.ftz.u64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.u64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.ftz.u64.f32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_test_int_from_f32(out: *mut u64, a: f32) {
    unsafe {
        *out = cvt_s32_f32::<{ Rounding::Rn }, false>(a) as u64;
        *out.add(1) = cvt_s32_f32::<{ Rounding::Rn }, true>(a) as u64;
        *out.add(2) = cvt_s32_f32::<{ Rounding::Rz }, false>(a) as u64;
        *out.add(3) = cvt_s32_f32::<{ Rounding::Rz }, true>(a) as u64;
        *out.add(4) = cvt_s32_f32::<{ Rounding::Rm }, false>(a) as u64;
        *out.add(5) = cvt_s32_f32::<{ Rounding::Rm }, true>(a) as u64;
        *out.add(6) = cvt_s32_f32::<{ Rounding::Rp }, false>(a) as u64;
        *out.add(7) = cvt_s32_f32::<{ Rounding::Rp }, true>(a) as u64;
        *out.add(8) = cvt_u32_f32::<{ Rounding::Rn }, false>(a) as u64;
        *out.add(9) = cvt_u32_f32::<{ Rounding::Rn }, true>(a) as u64;
        *out.add(10) = cvt_u32_f32::<{ Rounding::Rz }, false>(a) as u64;
        *out.add(11) = cvt_u32_f32::<{ Rounding::Rz }, true>(a) as u64;
        *out.add(12) = cvt_u32_f32::<{ Rounding::Rm }, false>(a) as u64;
        *out.add(13) = cvt_u32_f32::<{ Rounding::Rm }, true>(a) as u64;
        *out.add(14) = cvt_u32_f32::<{ Rounding::Rp }, false>(a) as u64;
        *out.add(15) = cvt_u32_f32::<{ Rounding::Rp }, true>(a) as u64;
        *out.add(16) = cvt_s64_f32::<{ Rounding::Rn }, false>(a) as u64;
        *out.add(17) = cvt_s64_f32::<{ Rounding::Rn }, true>(a) as u64;
        *out.add(18) = cvt_s64_f32::<{ Rounding::Rz }, false>(a) as u64;
        *out.add(19) = cvt_s64_f32::<{ Rounding::Rz }, true>(a) as u64;
        *out.add(20) = cvt_s64_f32::<{ Rounding::Rm }, false>(a) as u64;
        *out.add(21) = cvt_s64_f32::<{ Rounding::Rm }, true>(a) as u64;
        *out.add(22) = cvt_s64_f32::<{ Rounding::Rp }, false>(a) as u64;
        *out.add(23) = cvt_s64_f32::<{ Rounding::Rp }, true>(a) as u64;
        *out.add(24) = cvt_u64_f32::<{ Rounding::Rn }, false>(a) as u64;
        *out.add(25) = cvt_u64_f32::<{ Rounding::Rn }, true>(a) as u64;
        *out.add(26) = cvt_u64_f32::<{ Rounding::Rz }, false>(a) as u64;
        *out.add(27) = cvt_u64_f32::<{ Rounding::Rz }, true>(a) as u64;
        *out.add(28) = cvt_u64_f32::<{ Rounding::Rm }, false>(a) as u64;
        *out.add(29) = cvt_u64_f32::<{ Rounding::Rm }, true>(a) as u64;
        *out.add(30) = cvt_u64_f32::<{ Rounding::Rp }, false>(a) as u64;
        *out.add(31) = cvt_u64_f32::<{ Rounding::Rp }, true>(a) as u64;
    }
}

// CHECK-LABEL: .entry cvt_test_int_from_f64(
// CHECK-DAG: cvt.rni.s32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.s32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.s32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.s32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.u32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.u32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.u32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.u32.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.s64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.s64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.s64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.s64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rni.u64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rzi.u64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rmi.u64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK-DAG: cvt.rpi.u64.f64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn cvt_test_int_from_f64(out: *mut u64, a: f64) {
    unsafe {
        *out = cvt_s32_f64::<{ Rounding::Rn }>(a) as u64;
        *out.add(1) = cvt_s32_f64::<{ Rounding::Rz }>(a) as u64;
        *out.add(2) = cvt_s32_f64::<{ Rounding::Rm }>(a) as u64;
        *out.add(3) = cvt_s32_f64::<{ Rounding::Rp }>(a) as u64;
        *out.add(4) = cvt_u32_f64::<{ Rounding::Rn }>(a) as u64;
        *out.add(5) = cvt_u32_f64::<{ Rounding::Rz }>(a) as u64;
        *out.add(6) = cvt_u32_f64::<{ Rounding::Rm }>(a) as u64;
        *out.add(7) = cvt_u32_f64::<{ Rounding::Rp }>(a) as u64;
        *out.add(8) = cvt_s64_f64::<{ Rounding::Rn }>(a) as u64;
        *out.add(9) = cvt_s64_f64::<{ Rounding::Rz }>(a) as u64;
        *out.add(10) = cvt_s64_f64::<{ Rounding::Rm }>(a) as u64;
        *out.add(11) = cvt_s64_f64::<{ Rounding::Rp }>(a) as u64;
        *out.add(12) = cvt_u64_f64::<{ Rounding::Rn }>(a) as u64;
        *out.add(13) = cvt_u64_f64::<{ Rounding::Rz }>(a) as u64;
        *out.add(14) = cvt_u64_f64::<{ Rounding::Rm }>(a) as u64;
        *out.add(15) = cvt_u64_f64::<{ Rounding::Rp }>(a) as u64;
    }
}
