// --- LLM-generated --- //
// Checks the PTX emitted for the `st.async` intrinsics in `core::arch::nvptx` that require
// `sm_100` and PTX ISA 8.7, excluding the `multimem` variants.
// nvptx-target: sm_100a +ptx87

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// CHECK-LABEL: .entry mem_st_async_gpu(
// CHECK: st.async.release.gpu.global.b16 [{{%rd[0-9]+}}], {{%rs[0-9]+}};
// CHECK: st.async.release.gpu.global.b32 [{{%rd[0-9]+}}], {{%r[0-9]+}};
// CHECK: st.async.release.gpu.global.b64 [{{%rd[0-9]+}}], {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_st_async_gpu(
    p: *const *mut c_void,
    value16: u16,
    value32: u32,
    value64: u64,
) {
    unsafe {
        st_async_gpu_u16::<false>(*p as *mut u16, value16);
        st_async_gpu_u32::<false>(*p.add(1) as *mut u32, value32);
        st_async_gpu_u64::<false>(*p.add(2) as *mut u64, value64);
    }
}

// CHECK-LABEL: .entry mem_st_async_mmio_sys(
// CHECK: st.async.mmio.release.sys.global.b16 [{{%rd[0-9]+}}], {{%rs[0-9]+}};
// CHECK: st.async.mmio.release.sys.global.b32 [{{%rd[0-9]+}}], {{%r[0-9]+}};
// CHECK: st.async.mmio.release.sys.global.b64 [{{%rd[0-9]+}}], {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_st_async_mmio_sys(
    p: *const *mut c_void,
    value16: u16,
    value32: u32,
    value64: u64,
) {
    unsafe {
        st_async_mmio_sys_u16(*p as *mut u16, value16);
        st_async_mmio_sys_u32(*p.add(1) as *mut u32, value32);
        st_async_mmio_sys_u64(*p.add(2) as *mut u64, value64);
    }
}

// CHECK-LABEL: .entry mem_st_async_sys(
// CHECK: st.async.release.sys.global.b16 [{{%rd[0-9]+}}], {{%rs[0-9]+}};
// CHECK: st.async.release.sys.global.b32 [{{%rd[0-9]+}}], {{%r[0-9]+}};
// CHECK: st.async.release.sys.global.b64 [{{%rd[0-9]+}}], {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_st_async_sys(
    p: *const *mut c_void,
    value16: u16,
    value32: u32,
    value64: u64,
) {
    unsafe {
        st_async_sys_u16::<false>(*p as *mut u16, value16);
        st_async_sys_u32::<false>(*p.add(1) as *mut u32, value32);
        st_async_sys_u64::<false>(*p.add(2) as *mut u64, value64);
    }
}
