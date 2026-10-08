// --- LLM-generated --- //
// Checks the PTX emitted for the cluster launch control intrinsics in `core::arch::nvptx`, which
// require `sm_100` and PTX ISA 8.6.
// nvptx-target: sm_100a +ptx86

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry sync_clusterlaunchcontrol_query_cancel_get_first_ctaid(
// CHECK: clusterlaunchcontrol.query_cancel.get_first_ctaid::x.b32.b128 {{%r[0-9]+}}, %clc_handle;
// CHECK: clusterlaunchcontrol.query_cancel.get_first_ctaid::y.b32.b128 {{%r[0-9]+}}, %clc_handle;
// CHECK: clusterlaunchcontrol.query_cancel.get_first_ctaid::z.b32.b128 {{%r[0-9]+}}, %clc_handle;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_clusterlaunchcontrol_query_cancel_get_first_ctaid(
    response: *const u128,
    out: *mut u32,
) {
    unsafe {
        let response = *response;
        *out = clusterlaunchcontrol_query_cancel_get_first_ctaid_x(response);
        *out.add(1) = clusterlaunchcontrol_query_cancel_get_first_ctaid_y(response);
        *out.add(2) = clusterlaunchcontrol_query_cancel_get_first_ctaid_z(response);
    }
}

// CHECK-LABEL: .entry sync_clusterlaunchcontrol_query_cancel_is_canceled(
// CHECK: clusterlaunchcontrol.query_cancel.is_canceled.pred.b128 {{%p[0-9]+}}, %clc_handle;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn sync_clusterlaunchcontrol_query_cancel_is_canceled(
    response: *const u128,
    out: *mut u32,
) {
    unsafe {
        let response = *response;
        *out = clusterlaunchcontrol_query_cancel_is_canceled(response) as u32;
    }
}
