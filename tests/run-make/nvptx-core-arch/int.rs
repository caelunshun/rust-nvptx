// --- LLM-generated --- //
// Checks the PTX emitted for the integer arithmetic and bit manipulation intrinsics in
// `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry int_bfind(
// CHECK: bfind.u32 {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: bfind.shiftamt.u32 {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: bfind.s32 {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: bfind.shiftamt.s32 {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: bfind.u64 {{%r[0-9]+}}, {{%rd[0-9]+}};
// CHECK: bfind.shiftamt.u64 {{%r[0-9]+}}, {{%rd[0-9]+}};
// CHECK: bfind.s64 {{%r[0-9]+}}, {{%rd[0-9]+}};
// CHECK: bfind.shiftamt.s64 {{%r[0-9]+}}, {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_bfind(out: *mut u32, a: u32, b: i32, c: u64, d: i64) {
    unsafe {
        *out = bfind_u32::<false>(a);
        *out.add(1) = bfind_u32::<true>(a);
        *out.add(2) = bfind_s32::<false>(b);
        *out.add(3) = bfind_s32::<true>(b);
        *out.add(4) = bfind_u64::<false>(c);
        *out.add(5) = bfind_u64::<true>(c);
        *out.add(6) = bfind_s64::<false>(d);
        *out.add(7) = bfind_s64::<true>(d);
    }
}

// CHECK-LABEL: .entry int_bmsk(
// CHECK: ld.param.b32 [[A:%r[0-9]+]], [int_bmsk_param_1];
// CHECK: ld.param.b32 [[B:%r[0-9]+]], [int_bmsk_param_2];
// CHECK: bmsk.clamp.b32 {{%r[0-9]+}}, [[A]], [[B]];
// CHECK: bmsk.wrap.b32 {{%r[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_bmsk(out: *mut u32, a: u32, b: u32) {
    unsafe {
        *out = bmsk::<{ ClampMode::Clamp }>(a, b);
        *out.add(1) = bmsk::<{ ClampMode::Wrap }>(a, b);
    }
}

// CHECK-LABEL: .entry int_dp2a(
// CHECK: ld.param.b32 [[A:%r[0-9]+]], [int_dp2a_param_1];
// CHECK: ld.param.b32 [[B:%r[0-9]+]], [int_dp2a_param_2];
// CHECK: ld.param.b32 [[C:%r[0-9]+]], [int_dp2a_param_3];
// CHECK: dp2a.lo.u32.u32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: dp2a.hi.u32.u32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: dp2a.lo.u32.s32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: dp2a.hi.u32.s32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: dp2a.lo.s32.u32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: dp2a.hi.s32.u32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: dp2a.lo.s32.s32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: dp2a.hi.s32.s32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_dp2a(out: *mut u32, a: u32, b: u32, c: u32) {
    unsafe {
        *out = dp2a_u32_u32::<{ Dp2aMode::Lo }>(a, b, c);
        *out.add(1) = dp2a_u32_u32::<{ Dp2aMode::Hi }>(a, b, c);
        *out.add(2) = dp2a_u32_s32::<{ Dp2aMode::Lo }>(a, b as i32, c as i32) as u32;
        *out.add(3) = dp2a_u32_s32::<{ Dp2aMode::Hi }>(a, b as i32, c as i32) as u32;
        *out.add(4) = dp2a_s32_u32::<{ Dp2aMode::Lo }>(a as i32, b, c as i32) as u32;
        *out.add(5) = dp2a_s32_u32::<{ Dp2aMode::Hi }>(a as i32, b, c as i32) as u32;
        *out.add(6) = dp2a_s32_s32::<{ Dp2aMode::Lo }>(a as i32, b as i32, c as i32) as u32;
        *out.add(7) = dp2a_s32_s32::<{ Dp2aMode::Hi }>(a as i32, b as i32, c as i32) as u32;
    }
}

// CHECK-LABEL: .entry int_dp4a(
// CHECK: dp4a.u32.u32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: dp4a.u32.s32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: dp4a.s32.u32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: dp4a.s32.s32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_dp4a(out: *mut u32, a: u32, b: u32, c: u32) {
    unsafe {
        *out = dp4a_u32_u32(a, b, c);
        *out.add(1) = dp4a_u32_s32(a, b as i32, c as i32) as u32;
        *out.add(2) = dp4a_s32_u32(a as i32, b, c as i32) as u32;
        *out.add(3) = dp4a_s32_s32(a as i32, b as i32, c as i32) as u32;
    }
}

// CHECK-LABEL: .entry int_fns(
// CHECK: ld.param.b32 [[MASK:%r[0-9]+]], [int_fns_param_1];
// CHECK: ld.param.b32 [[BASE:%r[0-9]+]], [int_fns_param_2];
// CHECK: ld.param.b32 [[OFFSET:%r[0-9]+]], [int_fns_param_3];
// CHECK: fns.b32 {{%r[0-9]+}}, [[MASK]], [[BASE]], [[OFFSET]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_fns(out: *mut u32, mask: u32, base: u32, offset: i32) {
    unsafe { *out = fns(mask, base, offset) }
}

// CHECK-LABEL: .entry int_mul24(
// CHECK: mul24.lo.s32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: mul24.lo.u32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_mul24(out: *mut u32, a: u32, b: u32) {
    unsafe {
        *out = mul24_lo_s32(a as i32, b as i32) as u32;
        *out.add(1) = mul24_lo_u32(a, b);
    }
}

// CHECK-LABEL: .entry int_mul_hi(
// CHECK: mul.hi.s16 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK: mul.hi.u16 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK: mul.hi.s32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK: mul.hi.u32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK: mul.hi.s64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK: mul.hi.u64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_mul_hi(out: *mut u64, a: u64, b: u64) {
    unsafe {
        *out = mul_hi_s16(a as i16, b as i16) as u64;
        *out.add(1) = mul_hi_u16(a as u16, b as u16) as u64;
        *out.add(2) = mul_hi_s32(a as i32, b as i32) as u64;
        *out.add(3) = mul_hi_u32(a as u32, b as u32) as u64;
        *out.add(4) = mul_hi_s64(a as i64, b as i64) as u64;
        *out.add(5) = mul_hi_u64(a, b);
    }
}

// CHECK-LABEL: .entry int_sad(
// CHECK: sad.s16 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK: sad.u16 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK: sad.s32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK: sad.u32 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK: sad.s64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
// CHECK: sad.u64 {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}}, {{%[a-z]+[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_sad(out: *mut u64, a: u64, b: u64, c: u64) {
    unsafe {
        *out = sad_s16(a as i16, b as i16, c as i16) as u64;
        *out.add(1) = sad_u16(a as u16, b as u16, c as u16) as u64;
        *out.add(2) = sad_s32(a as i32, b as i32, c as i32) as u64;
        *out.add(3) = sad_u32(a as u32, b as u32, c as u32) as u64;
        *out.add(4) = sad_s64(a as i64, b as i64, c as i64) as u64;
        *out.add(5) = sad_u64(a, b, c);
    }
}

// CHECK-LABEL: .entry int_shf_l(
// CHECK: ld.param.b32 [[A:%r[0-9]+]], [int_shf_l_param_1];
// CHECK: ld.param.b32 [[B:%r[0-9]+]], [int_shf_l_param_2];
// CHECK: ld.param.b32 [[C:%r[0-9]+]], [int_shf_l_param_3];
// CHECK: shf.l.clamp.b32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: shf.l.wrap.b32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_shf_l(out: *mut u32, a: u32, b: u32, c: u32) {
    unsafe {
        *out = shf_l::<{ ClampMode::Clamp }>(a, b, c);
        *out.add(1) = shf_l::<{ ClampMode::Wrap }>(a, b, c);
    }
}

// CHECK-LABEL: .entry int_shf_r(
// CHECK: ld.param.b32 [[A:%r[0-9]+]], [int_shf_r_param_1];
// CHECK: ld.param.b32 [[B:%r[0-9]+]], [int_shf_r_param_2];
// CHECK: ld.param.b32 [[C:%r[0-9]+]], [int_shf_r_param_3];
// CHECK: shf.r.clamp.b32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: shf.r.wrap.b32 {{%r[0-9]+}}, [[A]], [[B]], [[C]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_shf_r(out: *mut u32, a: u32, b: u32, c: u32) {
    unsafe {
        *out = shf_r::<{ ClampMode::Clamp }>(a, b, c);
        *out.add(1) = shf_r::<{ ClampMode::Wrap }>(a, b, c);
    }
}

// CHECK-LABEL: .entry int_szext(
// CHECK: ld.param.b32 [[A:%r[0-9]+]], [int_szext_param_1];
// CHECK: ld.param.b32 [[B:%r[0-9]+]], [int_szext_param_2];
// CHECK: szext.clamp.s32 {{%r[0-9]+}}, [[A]], [[B]];
// CHECK: szext.wrap.s32 {{%r[0-9]+}}, [[A]], [[B]];
// CHECK: szext.clamp.u32 {{%r[0-9]+}}, [[A]], [[B]];
// CHECK: szext.wrap.u32 {{%r[0-9]+}}, [[A]], [[B]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn int_szext(out: *mut u32, a: u32, b: u32) {
    unsafe {
        *out = szext_s32::<{ ClampMode::Clamp }>(a as i32, b) as u32;
        *out.add(1) = szext_s32::<{ ClampMode::Wrap }>(a as i32, b) as u32;
        *out.add(2) = szext_u32::<{ ClampMode::Clamp }>(a, b);
        *out.add(3) = szext_u32::<{ ClampMode::Wrap }>(a, b);
    }
}
