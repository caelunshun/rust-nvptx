// --- LLM-generated --- //
// Checks the PTX emitted for the miscellaneous intrinsics in `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx81

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry misc_exit(
// CHECK: exit;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn misc_exit() {
    unsafe { exit() }
}

// CHECK-LABEL: .entry misc_nanosleep(
// CHECK: nanosleep.u32 {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn misc_nanosleep(t: u32) {
    unsafe { nanosleep(t) }
}

// CHECK-LABEL: .entry misc_pmevent_mask(
// CHECK: pmevent.mask 0x81U;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn misc_pmevent_mask() {
    unsafe { pmevent_mask::<0x81>() }
}

// CHECK-LABEL: .entry misc_prmt(
// CHECK: prmt.b32 {{%r[0-9]+}}, [[A:%r[0-9]+]], [[B:%r[0-9]+]], [[C:%r[0-9]+]];
// CHECK: prmt.b32.f4e {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: prmt.b32.b4e {{%r[0-9]+}}, [[A]], [[B]], [[C]];
// CHECK: prmt.b32.rc8 {{%r[0-9]+}}, [[A]], 0, [[C]];
// CHECK: prmt.b32.ecl {{%r[0-9]+}}, [[A]], 0, [[C]];
// CHECK: prmt.b32.ecr {{%r[0-9]+}}, [[A]], 0, [[C]];
// CHECK: prmt.b32.rc16 {{%r[0-9]+}}, [[A]], 0, [[C]];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn misc_prmt(out: *mut u32, a: u32, b: u32, c: u32) {
    unsafe {
        *out = prmt::<{ PrmtMode::Generic }>(a, b, c);
        *out.add(1) = prmt::<{ PrmtMode::F4e }>(a, b, c);
        *out.add(2) = prmt::<{ PrmtMode::B4e }>(a, b, c);
        *out.add(3) = prmt::<{ PrmtMode::Rc8 }>(a, b, c);
        *out.add(4) = prmt::<{ PrmtMode::Ecl }>(a, b, c);
        *out.add(5) = prmt::<{ PrmtMode::Ecr }>(a, b, c);
        *out.add(6) = prmt::<{ PrmtMode::Rc16 }>(a, b, c);
    }
}

// CHECK-LABEL: .entry misc_setmaxnreg(
// CHECK: setmaxnreg.inc.sync.aligned.u32 240;
// CHECK: setmaxnreg.dec.sync.aligned.u32 24;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn misc_setmaxnreg() {
    unsafe {
        setmaxnreg::<{ SetmaxnregAction::Inc }, 240>();
        setmaxnreg::<{ SetmaxnregAction::Dec }, 24>();
    }
}
