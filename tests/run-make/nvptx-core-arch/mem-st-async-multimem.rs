// --- LLM-generated --- //
// Checks the PTX emitted for the `multimem` variants of the `st.async` intrinsics in
// `core::arch::nvptx`, which require `sm_100` and PTX ISA 9.3.
// nvptx-target: sm_100a +ptx93

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// CHECK-LABEL: .entry mem_st_async_multimem_gpu(
// CHECK: multimem.st.async.release.gpu.global.b16 [{{%rd[0-9]+}}], {{%rs[0-9]+}};
// CHECK: multimem.st.async.release.gpu.global.b32 [{{%rd[0-9]+}}], {{%r[0-9]+}};
// CHECK: multimem.st.async.release.gpu.global.b64 [{{%rd[0-9]+}}], {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_st_async_multimem_gpu(
    p: *const *mut c_void,
    value16: u16,
    value32: u32,
    value64: u64,
) {
    unsafe {
        st_async_gpu_u16::<true>(*p as *mut u16, value16);
        st_async_gpu_u32::<true>(*p.add(1) as *mut u32, value32);
        st_async_gpu_u64::<true>(*p.add(2) as *mut u64, value64);
    }
}

// CHECK-LABEL: .entry mem_st_async_multimem_sys(
// CHECK: multimem.st.async.release.sys.global.b16 [{{%rd[0-9]+}}], {{%rs[0-9]+}};
// CHECK: multimem.st.async.release.sys.global.b32 [{{%rd[0-9]+}}], {{%r[0-9]+}};
// CHECK: multimem.st.async.release.sys.global.b64 [{{%rd[0-9]+}}], {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn mem_st_async_multimem_sys(
    p: *const *mut c_void,
    value16: u16,
    value32: u32,
    value64: u64,
) {
    unsafe {
        st_async_sys_u16::<true>(*p as *mut u16, value16);
        st_async_sys_u32::<true>(*p.add(1) as *mut u32, value32);
        st_async_sys_u64::<true>(*p.add(2) as *mut u64, value64);
    }
}
