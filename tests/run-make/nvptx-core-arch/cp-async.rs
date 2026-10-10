// --- LLM-generated --- //
// Checks the PTX emitted for the `cp.async` and non-tensor `cp.async.bulk` intrinsics in
// `core::arch::nvptx`.
// nvptx-target: sm_90a +ptx86

#![feature(no_core, stdarch_nvptx, abi_ptx)]
#![no_core]

extern crate core;

use core::arch::nvptx::*;
use core::ffi::c_void;

// CHECK-LABEL: .entry async_bulk_g2s_cluster(
// CHECK: cp.async.bulk.shared::cluster.global.mbarrier::complete_tx::bytes [{{%r[0-9]+}}], [{{%rd[0-9]+}}], {{%r[0-9]+}}, [{{%r[0-9]+}}];
// CHECK: cp.async.bulk.shared::cluster.global.mbarrier::complete_tx::bytes.multicast::cluster [{{%r[0-9]+}}], [{{%rd[0-9]+}}], {{%r[0-9]+}}, [{{%r[0-9]+}}], {{%rs[0-9]+}};
// CHECK: cp.async.bulk.shared::cluster.global.mbarrier::complete_tx::bytes.L2::cache_hint [{{%r[0-9]+}}], [{{%rd[0-9]+}}], {{%r[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
// CHECK: cp.async.bulk.shared::cluster.global.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [{{%r[0-9]+}}], [{{%rd[0-9]+}}], {{%r[0-9]+}}, [{{%r[0-9]+}}], {{%rs[0-9]+}}, {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_bulk_g2s_cluster(
    p: *const *mut c_void,
    mask: u16,
    policy: u64,
) {
    unsafe {
        cp_async_bulk_global_to_shared_cluster::<false, false>(
            *p,
            *p.add(1),
            *p.add(2),
            64,
            mask,
            policy,
        );
        cp_async_bulk_global_to_shared_cluster::<true, false>(
            *p.add(3),
            *p.add(4),
            *p.add(5),
            64,
            mask,
            policy,
        );
        cp_async_bulk_global_to_shared_cluster::<false, true>(
            *p.add(6),
            *p.add(7),
            *p.add(8),
            64,
            mask,
            policy,
        );
        cp_async_bulk_global_to_shared_cluster::<true, true>(
            *p.add(9),
            *p.add(10),
            *p.add(11),
            64,
            mask,
            policy,
        );
    }
}

// CHECK-LABEL: .entry async_bulk_g2s_cta(
// CHECK: cp.async.bulk.shared::cta.global.mbarrier::complete_tx::bytes [{{%r[0-9]+}}], [{{%rd[0-9]+}}], {{%r[0-9]+}}, [{{%r[0-9]+}}];
// CHECK: cp.async.bulk.shared::cta.global.mbarrier::complete_tx::bytes.L2::cache_hint [{{%r[0-9]+}}], [{{%rd[0-9]+}}], {{%r[0-9]+}}, [{{%r[0-9]+}}], {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_bulk_g2s_cta(p: *const *mut c_void, policy: u64) {
    unsafe {
        cp_async_bulk_global_to_shared_cta::<false>(*p, *p.add(1), *p.add(2), 64, policy);
        cp_async_bulk_global_to_shared_cta::<true>(*p.add(3), *p.add(4), *p.add(5), 64, policy);
    }
}

// CHECK-LABEL: .entry async_bulk_groups(
// CHECK: cp.async.bulk.commit_group;
// CHECK: cp.async.bulk.wait_group 0;
// CHECK: cp.async.bulk.wait_group.read 1;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_bulk_groups() {
    unsafe {
        cp_async_bulk_commit_group();
        cp_async_bulk_wait_group::<0, false>();
        cp_async_bulk_wait_group::<1, true>();
    }
}

// CHECK-LABEL: .entry async_bulk_prefetch_l2(
// CHECK: cp.async.bulk.prefetch.L2.global [{{%rd[0-9]+}}], {{%r[0-9]+}};
// CHECK: cp.async.bulk.prefetch.L2.global.L2::cache_hint [{{%rd[0-9]+}}], {{%r[0-9]+}}, {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_bulk_prefetch_l2(p: *const *const c_void, policy: u64) {
    unsafe {
        cp_async_bulk_prefetch_l2::<false>(*p, 64, 0);
        cp_async_bulk_prefetch_l2::<true>(*p.add(1), 64, policy);
    }
}

// CHECK-LABEL: .entry async_bulk_shared_cta_to_cluster(
// CHECK: cp.async.bulk.shared::cluster.shared::cta.mbarrier::complete_tx::bytes [{{%r[0-9]+}}], [{{%r[0-9]+}}], {{%r[0-9]+}}, [{{%r[0-9]+}}];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_bulk_shared_cta_to_cluster(p: *const *mut c_void) {
    unsafe {
        cp_async_bulk_shared_cta_to_cluster(*p, *p.add(1), *p.add(2), 64);
    }
}

// CHECK-LABEL: .entry async_bulk_shared_cta_to_global(
// CHECK: cp.async.bulk.global.shared::cta.bulk_group [{{%rd[0-9]+}}], [{{%r[0-9]+}}], {{%r[0-9]+}};
// CHECK: cp.async.bulk.global.shared::cta.bulk_group.L2::cache_hint [{{%rd[0-9]+}}], [{{%r[0-9]+}}], {{%r[0-9]+}}, {{%rd[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_bulk_shared_cta_to_global(
    p: *const *mut c_void,
    policy: u64,
) {
    unsafe {
        cp_async_bulk_shared_cta_to_global::<false>(*p, *p.add(1), 64, 0);
        cp_async_bulk_shared_cta_to_global::<true>(*p.add(2), *p.add(3), 64, policy);
    }
}

// The pointers are loaded from memory so that LLVM can't infer their address spaces.
// CHECK-LABEL: .entry async_ca(
// CHECK: cp.async.ca.shared.global [{{%r[0-9]+}}], [{{%rd[0-9]+}}], 4;
// CHECK: cp.async.ca.shared.global [{{%r[0-9]+}}], [{{%rd[0-9]+}}], 8;
// CHECK: cp.async.ca.shared.global [{{%r[0-9]+}}], [{{%rd[0-9]+}}], 16;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_ca(p: *const *mut c_void) {
    unsafe {
        cp_async_ca::<4>(*p, *p.add(1));
        cp_async_ca::<8>(*p.add(2), *p.add(3));
        cp_async_ca::<16>(*p.add(4), *p.add(5));
    }
}

// CHECK-LABEL: .entry async_ca_src_size(
// CHECK: cp.async.ca.shared.global [{{%r[0-9]+}}], [{{%rd[0-9]+}}], 4, {{%r[0-9]+}};
// CHECK: cp.async.ca.shared.global [{{%r[0-9]+}}], [{{%rd[0-9]+}}], 8, {{%r[0-9]+}};
// CHECK: cp.async.ca.shared.global [{{%r[0-9]+}}], [{{%rd[0-9]+}}], 16, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_ca_src_size(p: *const *mut c_void, size: u32) {
    unsafe {
        cp_async_ca_src_size::<4>(*p, *p.add(1), size);
        cp_async_ca_src_size::<8>(*p.add(2), *p.add(3), size);
        cp_async_ca_src_size::<16>(*p.add(4), *p.add(5), size);
    }
}

// CHECK-LABEL: .entry async_cg(
// CHECK: cp.async.cg.shared.global [{{%r[0-9]+}}], [{{%rd[0-9]+}}], 16;
// CHECK: cp.async.cg.shared.global [{{%r[0-9]+}}], [{{%rd[0-9]+}}], 16, {{%r[0-9]+}};
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_cg(p: *const *mut c_void, size: u32) {
    unsafe {
        cp_async_cg(*p, *p.add(1));
        cp_async_cg_src_size(*p.add(2), *p.add(3), size);
    }
}

// CHECK-LABEL: .entry async_groups(
// CHECK: cp.async.commit_group;
// CHECK: cp.async.wait_group 0;
// CHECK: cp.async.wait_group 2;
// CHECK: cp.async.wait_all;
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_groups() {
    unsafe {
        cp_async_commit_group();
        cp_async_wait_group::<0>();
        cp_async_wait_group::<2>();
        cp_async_wait_all();
    }
}

// CHECK-LABEL: .entry async_mbarrier_arrive(
// CHECK: cp.async.mbarrier.arrive.b64 [{{%rd[0-9]+}}];
// CHECK: cp.async.mbarrier.arrive.noinc.b64 [{{%rd[0-9]+}}];
// CHECK: cp.async.mbarrier.arrive.shared.b64 [{{%r[0-9]+}}];
// CHECK: cp.async.mbarrier.arrive.noinc.shared.b64 [{{%r[0-9]+}}];
#[unsafe(no_mangle)]
pub unsafe extern "ptx-kernel" fn async_mbarrier_arrive(p: *const *mut c_void) {
    unsafe {
        cp_async_mbarrier_arrive::<false, { StateSpace::Generic }>(*p);
        cp_async_mbarrier_arrive::<true, { StateSpace::Generic }>(*p.add(1));
        cp_async_mbarrier_arrive::<false, { StateSpace::SharedCta }>(*p.add(2));
        cp_async_mbarrier_arrive::<true, { StateSpace::SharedCta }>(*p.add(3));
    }
}
