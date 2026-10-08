// --- LLM-generated --- //
// Checks the PTX emitted for the warp-level intrinsics that require `sm_100a`.
// nvptx-target: sm_100a +ptx88

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;

// CHECK-LABEL: .entry warp_redux_sync_f32(
// CHECK: redux.sync.min.f32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.min.NaN.f32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.min.abs.f32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.min.abs.NaN.f32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.max.f32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.max.NaN.f32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.max.abs.f32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
// CHECK: redux.sync.max.abs.NaN.f32 {{%r[0-9]+}}, {{%r[0-9]+}}, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn warp_redux_sync_f32(out: *mut f32, a: f32, mask: u32) {
    unsafe {
        *out = redux_sync_f32::<{ ReduxOp::Min }, false, false>(a, mask);
        *out.add(1) = redux_sync_f32::<{ ReduxOp::Min }, false, true>(a, mask);
        *out.add(2) = redux_sync_f32::<{ ReduxOp::Min }, true, false>(a, mask);
        *out.add(3) = redux_sync_f32::<{ ReduxOp::Min }, true, true>(a, mask);
        *out.add(4) = redux_sync_f32::<{ ReduxOp::Max }, false, false>(a, mask);
        *out.add(5) = redux_sync_f32::<{ ReduxOp::Max }, false, true>(a, mask);
        *out.add(6) = redux_sync_f32::<{ ReduxOp::Max }, true, false>(a, mask);
        *out.add(7) = redux_sync_f32::<{ ReduxOp::Max }, true, true>(a, mask);
    }
}
