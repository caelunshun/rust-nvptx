// --- LLM-generated --- //
// Checks the PTX emitted for the non-tensor `cp.async.bulk` intrinsics in `core::arch::nvptx`
// that require `sm_100` and PTX ISA 8.6.
// nvptx-target: sm_100a +ptx86

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// CHECK-LABEL: .entry async_bulk_shared_cta_to_global_bytemask(
// CHECK: cp.async.bulk.global.shared::cta.bulk_group.cp_mask [{{%rd[0-9]+}}], [{{%rd[0-9]+}}], {{%r[0-9]+}}, {{%rs[0-9]+}};
// CHECK: cp.async.bulk.global.shared::cta.bulk_group.L2::cache_hint.cp_mask [{{%rd[0-9]+}}], [{{%rd[0-9]+}}], {{%r[0-9]+}}, {{%rd[0-9]+}}, {{%rs[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_bulk_shared_cta_to_global_bytemask(
    p: *const *mut c_void,
    mask: u16,
    policy: u64,
) {
    unsafe {
        cp_async_bulk_shared_cta_to_global_bytemask::<false>(*p, *p.add(1), 64, 0, mask);
        cp_async_bulk_shared_cta_to_global_bytemask::<true>(*p.add(2), *p.add(3), 64, policy, mask);
    }
}
